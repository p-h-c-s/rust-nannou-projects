use nannou::color::DARKBLUE;
use nannou::image::{DynamicImage, GenericImageView, RgbImage, Rgba};
use nannou::prelude::*;
use num::Complex;

pub mod mandelbrot;

fn main() {
    nannou::app(model).run();
}

// boundaries of the complex plane so the set is nicely visible. Taken from https://en.wikipedia.org/wiki/Mandelbrot_set
const MIN_X: f32 = -2.00;
const MAX_X: f32 = 0.47;
const MIN_Y: f32 = -1.12;
const MAX_Y: f32 = 1.12;

struct Model {
    window: WindowId,
    image: DynamicImage,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());
    let window = app
        .new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    let mut image = DynamicImage::new_rgb8(1000, 1000); // Adjust size as needed
    render(app, &mut image);
    Model { window, image }
}

fn render(app: &App, image: &mut DynamicImage) {
    let width = image.width() as f32;
    let height = image.height() as f32;

    for (x, y, pixel) in image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let fx: f32 = map_range(x as f32, 0.0, width, MIN_X, MAX_X);
        let fy: f32 = map_range(y as f32, 0.0, height, MIN_Y, MAX_Y);
        let color = color_function(fx, fy);
        *pixel = nannou::image::Rgb([color.0[0], color.0[1], color.0[2]]);
    }
}

fn color_function(x: f32, y: f32) -> Rgba<u8> {
    match mandelbrot::is_in_set(Complex::new(x, y)) {
        (true, _) => {
            return Rgba([0, 0, 0, 255]);
        }
        (false, it) => {
            let r = 0;
            let g = 0;
            let b = (map_range(it as f32, MIN_X, MAX_X, 64.0, 255.0) as u8);
            return Rgba([r, g, b, 255]);
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
