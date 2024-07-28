// controls points:
// Position = 0.0     Color = (  0,   7, 100)
// Position = 0.16    Color = ( 32, 107, 203)
// Position = 0.42    Color = (237, 255, 255)
// Position = 0.6425  Color = (255, 170,   0)
// Position = 0.8575  Color = (  0,   2,   0)

// This module needs to monotonic cubic interpolate between these points and generate a 2048 sized array of colors
//
// double smoothed = Math.Log2(Math.Log2(re * re + im * im) / 2);  // log_2(log_2(|p|))
// int colorI = (int)(Math.Sqrt(i + 10 - smoothed) * gradient.Scale) % colors.Length;
// Color color = colors[colorI];

use std::ops::Div;

use nannou::prelude::*;
use num::{integer::Roots, Complex};
use nannou::image::Rgba;

pub const ARRAY_SIZE: usize = 2048;

struct ControlPoint {
    position: f64,
    color: (u8, u8, u8),
}

fn monotonic_cubic_interpolate(x: f64, points: &[ControlPoint]) -> (u8, u8, u8) {
    if points.len() < 2 {
        return points[0].color;
    }

    // Find the segment containing x
    let mut i = 0;
    while i < points.len() - 1 && x > points[i + 1].position {
        i += 1;
    }

    if i == points.len() - 1 {
        return points[i].color;
    }

    let x0 = points[i].position;
    let x1 = points[i + 1].position;
    let t = (x - x0) / (x1 - x0);

    let p0 = points[i].color;
    let p1 = points[i + 1].color;

    let m0 = if i > 0 {
        calculate_slope(points[i - 1].color, p0, points[i - 1].position, x0)
    } else {
        calculate_slope(p0, p1, x0, x1)
    };

    let m1 = if i < points.len() - 2 {
        calculate_slope(p0, points[i + 2].color, x0, points[i + 2].position)
    } else {
        calculate_slope(p0, p1, x0, x1)
    };

    let (r, g, b) = cubic_hermite(p0, p1, m0, m1, t);
    (r as u8, g as u8, b as u8)
}

fn calculate_slope(p0: (u8, u8, u8), p1: (u8, u8, u8), x0: f64, x1: f64) -> (f64, f64, f64) {
    let dx = x1 - x0;
    (
        (p1.0 as f64 - p0.0 as f64) / dx,
        (p1.1 as f64 - p0.1 as f64) / dx,
        (p1.2 as f64 - p0.2 as f64) / dx,
    )
}

fn cubic_hermite(p0: (u8, u8, u8), p1: (u8, u8, u8), m0: (f64, f64, f64), m1: (f64, f64, f64), t: f64) -> (f64, f64, f64) {
    let t2 = t * t;
    let t3 = t2 * t;
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    (
        h00 * p0.0 as f64 + h10 * m0.0 + h01 * p1.0 as f64 + h11 * m1.0,
        h00 * p0.1 as f64 + h10 * m0.1 + h01 * p1.1 as f64 + h11 * m1.1,
        h00 * p0.2 as f64 + h10 * m0.2 + h01 * p1.2 as f64 + h11 * m1.2,
    )
}

pub fn create_color_array() -> Vec<Rgba<u8>> {
    let control_points = vec![
        ControlPoint { position: 0.0, color: (0, 7, 100) },
        ControlPoint { position: 0.16, color: (32, 107, 203) },
        ControlPoint { position: 0.42, color: (237, 255, 255) },
        ControlPoint { position: 0.6425, color: (255, 170, 0) },
        ControlPoint { position: 0.8575, color: (0, 2, 0) },
    ];

    let mut colors = Vec::with_capacity(ARRAY_SIZE);

    for i in 0..ARRAY_SIZE {
        let x = i as f64 / (ARRAY_SIZE - 1) as f64;
        let (r, g, b) = monotonic_cubic_interpolate(x, &control_points);
        colors.push(Rgba([r, g, b, 255]));
    }

    colors
}


/// Inspired in: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
#[inline]
pub fn get_interpolated_color(colors: &Vec<Rgba<u8>>, iterations: usize, z: Complex<f64>) -> Rgba<u8> {
    let smoothed = (z.re * z.re + z.im * z.im).log2().div(2.0).log2();
    let idx = (((iterations as f64 - smoothed).sqrt()) * 256.0) % 2048.0;
    colors[idx as usize]
}