use nannou::prelude::*;
use nannou::image::{DynamicImage, GenericImageView, RgbImage, Rgba};
use nannou::color::DARKBLUE;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
    image: DynamicImage,
}

fn model(app: &App) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    let mut image = DynamicImage::new_rgb8(100, 100); // Adjust size as needed
    render(app, &mut image);
    Model { window, image }
}

fn render(app: &App, image: &mut DynamicImage) {
    let win = app.window_rect();
    let width = image.width() as f32;
    let height = image.height() as f32;

    for (x, y, pixel) in image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let fx = map_range(x as f32, 0.0, width, win.left(), win.right());
        let fy = map_range(y as f32, 0.0, height, win.bottom(), win.top());
        let color = color_function(fx, fy);
        *pixel = nannou::image::Rgb([color.0[0], color.0[1], color.0[2]]);
    }
}

fn color_function(x: f32, y: f32) -> Rgba<u8> {
    // Replace this with your custom color function
    // Example: create a gradient based on x and y
    let r = (map_range(x, -300.0, 300.0, 0.0, 255.0) as u8);
    let g = (map_range(y, -300.0, 300.0, 0.0, 255.0) as u8);
    let b = 128;
    Rgba([r, g, b, 255])
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKBLUE);

    let texture = wgpu::Texture::from_image(app, &model.image);
    draw.texture(&texture)
        .w_h(app.window_rect().w(), app.window_rect().h());
    // draw.texture(&texture);

    draw.to_frame(app, &frame).unwrap();
}
