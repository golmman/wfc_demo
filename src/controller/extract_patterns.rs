use std::path::Path;

use crate::model::{pattern_data::PatternData, raw_image::RawImage};

use image::GenericImageView;
use log::info;

pub fn extract_patterns<T: AsRef<Path>>(
    path: T,
    pattern_width: u32,
    pattern_height: u32,
) -> PatternData {
    let mut patterns = Vec::new();

    let (image_width, image_height, image_data) = load_image(path);

    for image_y in 0..image_width {
        for image_x in 0..image_height {
            patterns.push(vec![0; (pattern_width * pattern_height) as usize]);
            let patterns_last = patterns.len() - 1;

            for pattern_y in 0..pattern_width {
                for pattern_x in 0..pattern_width {
                    let scan_x = (image_x + pattern_x) % image_width;
                    let scan_y = (image_y + pattern_y) % image_height;
                    let image_index = (image_width * scan_y + scan_x) as usize;
                    let pattern_index = (pattern_width * pattern_y + pattern_x) as usize;

                    patterns[patterns_last][pattern_index] = image_data[image_index];
                }
            }
        }
    }

    info!("pattern width: {}", pattern_width);
    info!("pattern height: {}", pattern_height);
    info!("image width: {}", image_width);
    info!("image height: {}", image_height);
    info!(
        "number of patterns: {} (should equal image_w * image_h)",
        patterns.len()
    );

    PatternData {
        patterns,
        // TODO: is this additional data even needed?
        image_height,
        image_width,
        pattern_height,
        pattern_width,
    }
}

fn load_image_raw<T: AsRef<Path>>(path: T) -> RawImage {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    let raw = img.into_bytes();
    (width, height, raw)
}

fn load_image<T: AsRef<Path>>(path: T) -> (u32, u32, Vec<u32>) {
    let (width, height, raw32bit) = load_image_raw(path);
    let mut data = vec![0; (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let i = (width * y + x) as usize;
            let rgba = (raw32bit[4 * i] as u32)
                | (raw32bit[4 * i + 1] as u32) << 8
                | (raw32bit[4 * i + 2] as u32) << 16
                | (raw32bit[4 * i + 3] as u32) << 24;

            data[i] = rgba;
        }
    }

    (width, height, data)
}
