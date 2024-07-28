use nannou::color::DARKBLUE;
use nannou::event::Event;
use nannou::image::{DynamicImage, GenericImageView, Rgba};
use nannou::prelude::*;
use num::Complex;

pub mod mandelbrot;
pub mod fractal_colouring;

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

struct Model {
    _window: WindowId,
    image: DynamicImage,
    zoom: f64,
    center: DVec2, // f64 equivalent to Point2
    colors: Vec<Rgba<u8>>
}

impl Model {
    // this can be heavily parallelized if we split the image into chunks
    fn render(&mut self) {
        let width = self.image.width() as f64;
        let height = self.image.height() as f64;

        let dx = (MAX_X - MIN_X) / self.zoom as f64;
        let dy = (MAX_Y - MIN_Y) / self.zoom as f64;

        // Centers the new view in the existing center point
        let min_x = self.center.x - dx / 2.0;
        let max_x = self.center.x + dx / 2.0;
        let min_y = self.center.y - dy / 2.0;
        let max_y = self.center.y + dy / 2.0;

        for (x, y, pixel) in self.image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
            let fx = map_range(x as f64, 0.0, width, min_x, max_x);
            let fy = map_range(y as f64, 0.0, height, min_y, max_y);
            let color = mandelbrot_color_mapping(fx, fy, &self.colors);
            *pixel = nannou::image::Rgb([color.0[0], color.0[1], color.0[2]]);
        }
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());
    let _window = app
        .new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();

    let image = DynamicImage::new_rgb8(IMAGE_RESOLUTION.0, IMAGE_RESOLUTION.1);
    let mut model = Model {
        _window,
        image,
        zoom: 1.0,
        center: DVec2::new((MIN_X + MAX_X) / 2.0, (MIN_Y + MAX_Y) / 2.0),
        colors: fractal_colouring::create_color_array()
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
        (true, _) => {
            return Rgba([0, 0, 0, 255]);
        }
        (false, it) => {
            let rgb = fractal_colouring::get_interpolated_color(colors, it);
            return rgb
        }
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
