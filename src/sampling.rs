use std::f64::consts::TAU;

use image::{Rgb, RgbImage};
use serde::{Deserialize, Serialize};

use crate::error::{ErrorKind, Result};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SamplingLevel {
    Single,
    #[default]
    Low,
    Medium,
    High,
    Ultra,
    Extreme,
}

pub fn spiral_sampling_points(sampling_level: Option<SamplingLevel>) -> Vec<(f64, f64)> {
    const R: f64 = 1.2;

    // Maybe too precise but yes.
    const PHI: f64 = 1.618033988749895;

    let n: i32 = match sampling_level.unwrap_or_default() {
        SamplingLevel::Single => 1,
        SamplingLevel::Low => 4,
        SamplingLevel::Medium => 8,
        SamplingLevel::High => 16,
        SamplingLevel::Ultra => 32,
        SamplingLevel::Extreme => 64,
    };

    (0..n)
        .map(|i| (i as f64 / PHI % 1., i as f64 / (n - 1) as f64))
        .map(|(x, y)| {
            let r = y;
            let theta = TAU * x;
            (r * R * theta.cos(), r * R * theta.sin())
        })
        .collect::<Vec<_>>()
}

pub fn preview_sampling_points(sampling_points: &Vec<(f64, f64)>) -> Result<()> {
    let size = 250;
    let center = size / 2;
    let px = 50;
    let mut preview = RgbImage::new(size, size);
    // preview.fill(255);

    for &(x, y) in sampling_points {
        preview.put_pixel(
            (center as f64 + px as f64 * x) as u32,
            (center as f64 + px as f64 * y) as u32,
            Rgb([255, 255, 255]),
        );
    }

    preview.put_pixel(center - px, center - px, Rgb([255, 0, 0]));
    preview.put_pixel(center - px, center, Rgb([255, 0, 0]));
    preview.put_pixel(center - px, center + px, Rgb([255, 0, 0]));
    preview.put_pixel(center, center - px, Rgb([255, 0, 0]));
    preview.put_pixel(center, center + px, Rgb([255, 0, 0]));
    preview.put_pixel(center + px, center - px, Rgb([255, 0, 0]));
    preview.put_pixel(center + px, center, Rgb([255, 0, 0]));
    preview.put_pixel(center + px, center + px, Rgb([255, 0, 0]));

    preview
        .save("_sampling_pattern.png")
        .map_err(ErrorKind::SaveImage)?;

    Ok(())
}
