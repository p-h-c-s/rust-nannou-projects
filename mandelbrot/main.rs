use std::sync::Arc;

use nannou::color::DARKBLUE;
use nannou::event::Event;
use nannou::image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use nannou::prelude::*;
use num::integer::sqrt;
use num::Complex;

pub mod fractal_colouring;
pub mod mandelbrot;

fn main() {
    nannou::app(model).event(event).run();
}

/// boundaries of the complex plane so the set is nicely visible. Taken from https://en.wikipedia.org/wiki/Mandelbrot_set
/// These values represent our view of the original image plane. So the points in the image buffer are mapped to these ranges
const MIN_X: f64 = -2.00;
const MAX_X: f64 = 0.47;
const MIN_Y: f64 = -1.12;
const MAX_Y: f64 = 1.12;

/// Quality x Performance settings.
/// MAX_ITER defines the amount of computation to be done per point to assess set belonging.
/// IMAGE_RESOLUTION defines the number of pixels in the x and y direction of the rendered image.
/// Low values of MAX_ITER creates less detailed fractals
const MAX_ITER: usize = 150;
const IMAGE_RESOLUTION: (u32, u32) = (1920, 1080);
const CHUNKS_TO_PARALLEL_RENDER: u32 = 9;


struct Model {
    _window: WindowId,
    image: DynamicImage,
    zoom: f64,
    center: DVec2, // f64 equivalent to Point2
    colors: Vec<Rgba<u8>>,
}

impl Model {
    // this can be heavily parallelized if we split the image into chunks
    fn render(&mut self) {
        let width = self.image.width() as f64;
        let height = self.image.height() as f64;
        println!("{width}");

        let dx = (MAX_X - MIN_X) / self.zoom as f64;
        let dy = (MAX_Y - MIN_Y) / self.zoom as f64;

        // Centers the new view in the existing center point
        let min_x = self.center.x - dx / 2.0;
        let max_x = self.center.x + dx / 2.0;
        let min_y = self.center.y - dy / 2.0;
        let max_y = self.center.y + dy / 2.0;

        // divide (1920, 1080) in 4 squares
        // 2073600 pixels
        // each area needs 518400 pixes
        // 1080 / 2 = 540
        // 1920 / 2 = 960

        // input -> (1920, 1080)
        // sqr1 -> (0,0) -> (960, 540)
        // sqr2 -> (960,0) -> (1920, 540)
        // sqr3 -> (0,540) -> (960, 1080)
        // sqr4 -> (960,540) -> (1920, 1080)

        // [x, y] + [x/boundary + y/boundary]
        // [x, y] = combinações de [(x/boundary) * chunk_num, (y/boundary)] e [x, y] = combinações de [(x/boundary), (y/boundary) * chunk_num]


        // x and y are known. We can manually get the pixes and paralellze the image
        let pixel_range = split_pixel_range(width as u32, height as u32, CHUNKS_TO_PARALLEL_RENDER);

        let mut image_ref = Arc::new(self.image);
        for range in pixel_range {
            let starting_point = range.0;
            let end_point = range.1;
            for x in starting_point.0..end_point.0 {
                for y in starting_point.1..end_point.1 {
                    iterate_image(&mut image_ref, x, y, width, height, min_x, min_y, max_x, max_y, &self.colors);
                }
            }
        }
    }


} 

fn iterate_image(image: &mut DynamicImage, x: u32, y: u32, width: f64, height: f64, min_x:f64, min_y: f64, max_x: f64, max_y: f64, colors: &Vec<Rgba<u8>>) {
    let fx = map_range(x as f64, 0.0, width, min_x, max_x);
    let fy = map_range(y as f64, 0.0, height, min_y, max_y);
    let color = mandelbrot_color_mapping(fx, fy, colors);
    // let pixel = self.image.get_pixel(x,  y);
    let pixel = nannou::image::Rgba([color.0[0], color.0[1], color.0[2], 1]);
    image.put_pixel(x, y, pixel);
}
// Splits a x,y space into chunks of equal ranges. chunks parameter square root must be a whole number
// Logic: each boundary starting point is (n * x_b, m * y_b). n is the column number, m the row number of the chunks matrix
// x_b is the chunk boundary horizontal size, and y_b is the chunk boundary vertical size
// Each starting point gets summed to (x_b, y_b), representing the diagonal
// The splitting doesn't need to be in equal squares. I realized that after implementing this. You can "serialize" the image and split it in a single buffer
fn split_pixel_range(x: u32, y: u32, chunks: u32) -> Vec<((u32, u32), (u32, u32))> {
    let mut output: Vec<((u32, u32), (u32, u32))> = Vec::new();
    let chunk_matrix_end = sqrt(chunks);
    let x_boundary = x / chunk_matrix_end;
    let y_boundary = y / chunk_matrix_end;
    let diag_sum = (x_boundary, y_boundary);
    for n in 0..chunk_matrix_end {
        for m in 0..chunk_matrix_end {
            let starting_point = ((n * x_boundary), (m * y_boundary));
            let end_point = ((starting_point.0 + diag_sum.0), (starting_point.1 + diag_sum.1));
            output.push((starting_point, end_point));
        }
    }
    return output
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());
    let _window = app
        .new_window()
        .size(1600, 1600)
        .view(view)
        .build()
        .unwrap();

    let image = DynamicImage::new_rgb8(IMAGE_RESOLUTION.0, IMAGE_RESOLUTION.1);
    let mut model = Model {
        _window,
        image,
        zoom: 1.0,
        center: DVec2::new((MIN_X + MAX_X) / 2.0, (MIN_Y + MAX_Y) / 2.0),
        colors: fractal_colouring::create_color_array(),
    };
    model.render();
    model
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(w_event),
        } => {
            match w_event {
                WindowEvent::MousePressed(button) => {
                    let mouse_pos = app.mouse.position();
                    let image_width = model.image.width() as f64;
                    let image_height = model.image.height() as f64;

                    // Convert mouse position to image coordinates. The mouse position are associated to the original cartesian plane in window
                    let image_x = map_range(
                        mouse_pos.x,
                        -app.window_rect().w() / 2.0,
                        app.window_rect().w() / 2.0,
                        0.0,
                        image_width,
                    );
                    let image_y = map_range(
                        mouse_pos.y,
                        -app.window_rect().h() / 2.0,
                        app.window_rect().h() / 2.0,
                        image_height,
                        0.0,
                    );

                    // Convert image coordinates to complex plane coordinates
                    let dx = (MAX_X - MIN_X) / model.zoom as f64;
                    let dy = (MAX_Y - MIN_Y) / model.zoom as f64;
                    let new_x = model.center.x + (image_x / image_width - 0.5) * dx;
                    let new_y = model.center.y + (image_y / image_height - 0.5) * dy;

                    // Update center and zoom
                    model.center = DVec2::new(new_x, new_y);
                    match button {
                        MouseButton::Left => model.zoom *= 2.0,  // Zoom in
                        MouseButton::Right => model.zoom /= 2.0, // Zoom out
                        _ => {}
                    }
                    model.render();
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn mandelbrot_color_mapping(x: f64, y: f64, colors: &Vec<Rgba<u8>>) -> Rgba<u8> {
    match mandelbrot::is_in_set(Complex::new(x, y)) {
        (true, _, _) => Rgba([0, 0, 0, 255]),
        (false, it, complex_n) => fractal_colouring::get_interpolated_color(colors, it, complex_n),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKBLUE);

    let texture = wgpu::Texture::from_image(app, &model.image);
    draw.texture(&texture)
        .w_h(app.window_rect().w(), app.window_rect().h());

    draw.to_frame(app, &frame).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pixel_range() {
        // let image = DynamicImage::new_rgb8(IMAGE_RESOLUTION.0, IMAGE_RESOLUTION.1);
        let res = split_pixel_range(IMAGE_RESOLUTION.0, IMAGE_RESOLUTION.1, 4);
        // println!("{res:?}");
    }
}
