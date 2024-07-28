use nannou::color::DARKBLUE;
use nannou::image::{DynamicImage, GenericImageView, RgbImage, Rgba};
use nannou::prelude::*;
use nannou::event::Event;
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
    click_coords: Point2
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
    let mut model = Model { window, image, zoom: 1.0, last_m_event: WindowEvent::Focused, click_coords: Point2::new(0.0, 0.0)};
    render(app, &mut model);
    model
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple: Some(w_event) } => {
            // println!("event: {:?}", _event);
            match w_event {
                WindowEvent::MouseReleased(_) => {
                    match model.last_m_event {
                        WindowEvent::MouseMoved(p) => {
                            model.zoom *= 1.1;
                            model.click_coords = p;
                            render(app, model);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
            model.last_m_event = w_event
        }
        _ => {}
    }
}

fn handle_zoom(delta: MouseScrollDelta, phase: TouchPhase) -> Option<f64> {
    // maybe only calculate if the phase is correct
    let zoom_factor = match delta {
        MouseScrollDelta::LineDelta(x, y) => {
            println!("{:?} {:?}", x, y);
            if y > 0.0 { 1.1 } else { 0.9 }
        }
        MouseScrollDelta::PixelDelta(pos) => {
            println!("pixel: {:?} {:?}", pos.x, pos.y);
            if pos.y > 0.0 { 1.1 } else { 0.9 }
        }
    };

    println!("{:?}", phase);
    match phase {
        TouchPhase::Ended => {
            Some(zoom_factor)
        }
        _ => {
            // Scroll gesture was interrupted
            None
        }
    }

    // for ref:
    // match phase {
    //     TouchPhase::Started => {
    //         // Scroll gesture has started
    //     }
    //     TouchPhase::Moved => {
    //         // Scroll is in progress
    //         model.zoom *= zoom_factor;
    //     }
    //     TouchPhase::Ended => {
    //         // Scroll gesture has ended
    //     }
    //     TouchPhase::Cancelled => {
    //         // Scroll gesture was interrupted
    //     }
    // }


    // Flag that we need to redraw
}



// this can be heavily optimized
fn render(app: &App, model: &mut Model) {
    let width = model.image.width() as f32;
    let height = model.image.height() as f32;
    let z = model.zoom as f32;

    for (x, y, pixel) in model.image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let fx: f32 = map_range(x as f32, 0.0, width, MIN_X, MAX_X) / z;
        let fy: f32 = map_range(y as f32, 0.0, height, MIN_Y, MAX_Y) / z;
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
