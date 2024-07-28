// controls points:
// Position = 0.0     Color = (  0,   7, 100)
// Position = 0.16    Color = ( 32, 107, 203)
// Position = 0.42    Color = (237, 255, 255)
// Position = 0.6425  Color = (255, 170,   0)
// Position = 0.8575  Color = (  0,   2,   0)

// This module needs to cubic interpolate between these points and generate a 2048 sized array of colors
//
// double smoothed = Math.Log2(Math.Log2(re * re + im * im) / 2);  // log_2(log_2(|p|))
// int colorI = (int)(Math.Sqrt(i + 10 - smoothed) * gradient.Scale) % colors.Length;
// Color color = colors[colorI];

use nannou::prelude::*;
use num::integer::Roots;

pub const ARRAY_SIZE: usize = 2048;

struct ControlPoint {
    position: f64,
    color: (u8, u8, u8),
}

pub fn interpolate_colors() -> [(u8, u8, u8); ARRAY_SIZE] {
    let control_points = [
        ControlPoint { position: 0.0, color: (0, 7, 100) },
        ControlPoint { position: 0.16, color: (32, 107, 203) },
        ControlPoint { position: 0.42, color: (237, 255, 255) },
        ControlPoint { position: 0.6425, color: (255, 170, 0) },
        ControlPoint { position: 0.8575, color: (0, 2, 0) },
    ];

    let mut colors = [(0, 0, 0); ARRAY_SIZE];

    for i in 0..control_points.len() - 1 {
        let start = &control_points[i];
        let end = &control_points[i + 1];

        let start_idx = (start.position * ARRAY_SIZE as f64) as usize;
        let end_idx = (end.position * ARRAY_SIZE as f64) as usize;

        for j in start_idx..=end_idx {
            let t = (j - start_idx) as f64 / (end_idx - start_idx) as f64;

            colors[j] = (
                map_range(t, 0.0, 1.0, start.color.0 as f64, end.color.0 as f64).clamp(0.0, 255.0) as u8,
                map_range(t, 0.0, 1.0, start.color.1 as f64, end.color.1 as f64).clamp(0.0, 255.0) as u8,
                map_range(t, 0.0, 1.0, start.color.2 as f64, end.color.2 as f64).clamp(0.0, 255.0) as u8,
            );
        }
    }

    colors
}

#[inline]
pub fn get_interpolated_color(colors: &[(u8, u8, u8)], iterations: usize) -> (u8, u8, u8) {
    let idx = (((iterations + 10).sqrt() as f64) * 256.0) % 2048.0;
    colors[idx as usize]
}