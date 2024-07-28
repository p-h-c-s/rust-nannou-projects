use nannou::color::DARKBLUE;
use nannou::event::Event;
use nannou::image::{DynamicImage, GenericImageView, RgbImage, Rgba};
use nannou::prelude::*;
use nannou::winit::event::{Touch, TouchPhase};
use num::Complex;
use window::MouseMovedFn;

pub mod mandelbrot;

fn main() {
    nannou::app(model).event(event).run();
}

// boundaries of the complex plane so the set is nicely visible. Taken from https://en.wikipedia.org/wiki/Mandelbrot_set
const MIN_X: f32 = -2.00;
const MAX_X: f32 = 0.47;
const MIN_Y: f32 = -1.12;
const MAX_Y: f32 = 1.12;

struct Model {
    window: WindowId,
    image: DynamicImage,
    zoom: f64,
    last_m_event: WindowEvent,
    center: Point2,
}

fn model(app: &App) -> Model {
    // nannou continuously tries to draw the texture from the rendered image by calling "view" for each frame
    // Wait mode only redraws when an event happens
    app.set_loop_mode(LoopMode::wait());
    let window = app
        .new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    let image = DynamicImage::new_rgb8(1000, 1000); // Adjust size as needed
    let mut model = Model {
        window,
        image,
        zoom: 1.0,
        last_m_event: WindowEvent::Focused,
        center: Point2::new((MIN_X + MAX_X) / 2.0, (MIN_Y + MAX_Y) / 2.0),
    };
    render(app, &mut model);
    model
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple: Some(w_event) } => {
            match w_event {
                WindowEvent::MousePressed(button) => {
                    let mouse_pos = app.mouse.position();
                    let image_width = model.image.width() as f32;
                    let image_height = model.image.height() as f32;
                    
                    // Convert mouse position to image coordinates
                    let image_x = map_range(mouse_pos.x, -app.window_rect().w()/2.0, app.window_rect().w()/2.0, 0.0, image_width);
                    let image_y = map_range(mouse_pos.y, -app.window_rect().h()/2.0, app.window_rect().h()/2.0, image_height, 0.0);
                    
                    // Convert image coordinates to complex plane coordinates
                    let dx = (MAX_X - MIN_X) / model.zoom as f32;
                    let dy = (MAX_Y - MIN_Y) / model.zoom as f32;
                    let new_x = model.center.x + (image_x / image_width - 0.5) * dx;
                    let new_y = model.center.y + (image_y / image_height - 0.5) * dy;
                    
                    // Update center and zoom
                    model.center = Point2::new(new_x, new_y);
                    match button {
                        MouseButton::Left => model.zoom *= 2.0,  // Zoom in
                        MouseButton::Right => model.zoom /= 2.0, // Zoom out
                        _ => {}
                    }
                    
                    render(app, model);
                }
                _ => {}
            }
            model.last_m_event = w_event;
        }
        _ => {}
    }
}

// this can be heavily optimized
fn render(app: &App, model: &mut Model) {
    let width = model.image.width() as f32;
    let height = model.image.height() as f32;
    
    let dx = (MAX_X - MIN_X) / model.zoom as f32;
    let dy = (MAX_Y - MIN_Y) / model.zoom as f32;
    
    let min_x = model.center.x - dx / 2.0;
    let max_x = model.center.x + dx / 2.0;
    let min_y = model.center.y - dy / 2.0;
    let max_y = model.center.y + dy / 2.0;

    for (x, y, pixel) in model.image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let fx = map_range(x as f32, 0.0, width, min_x, max_x);
        let fy = map_range(y as f32, 0.0, height, min_y, max_y);
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
    // render(app, model);

    let texture = wgpu::Texture::from_image(app, &model.image);
    draw.texture(&texture)
        .w_h(app.window_rect().w(), app.window_rect().h());

    draw.to_frame(app, &frame).unwrap();
}
