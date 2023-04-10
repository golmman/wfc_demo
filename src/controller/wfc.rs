use std::path::Path;

use log::error;
use log::info;
use term2d::model::rgba::Rgba;

pub type RawImage = (u32, u32, Vec<u8>);

pub fn run<T: AsRef<Path>>(path: T, pattern_width: u32, pattern_height: u32) -> RawImage {
    extract_patterns(path, pattern_width, pattern_height);
    build_propagator();

    for i in 0..10 {
        observe();
        propagate();
    }

    combine_observations()
}

fn extract_patterns<T: AsRef<Path>>(
    path: T,
    pattern_width: u32,
    pattern_height: u32,
) -> Vec<Vec<u8>> {
    let patterns = Vec::new();

    //let (image_width, image_height, image) = helper::load_image_raw("data/flowers.png");
    let (image_width, image_height, image) = helper::load_image_as_8bit("data/flowers.png");

    let image_width = image_width as i32;
    let image_height = image_height as i32;
    let pattern_width = pattern_width as i32;
    let pattern_height = pattern_height as i32;

    let left = 1 - pattern_width;
    let right = image_width + pattern_width - 1;
    let top = 1 - pattern_height;
    let bottom = image_height + pattern_height - 1;

    let mut debug_count = 0;

    for image_y in left..image_width {
        for image_x in top..image_height {
            for pattern_y in 0..pattern_width {
                for pattern_x in 0..pattern_width {
                    let scan_x = helper::euclidean_remainder(image_x + pattern_x, image_width);
                    let scan_y = helper::euclidean_remainder(image_y + pattern_y, image_height);
                    let index = 4 * (image_width * scan_y + scan_x);

                    debug_count += 1;
                }
            }
        }
    }

    info!("hey");
    info!("len {}", image.len());
    info!("w {}", image_width);
    info!("h {}", image_height);
    info!("debug_count {}", debug_count);

    patterns
}

fn build_propagator() {}

fn observe() {}

fn propagate() {}

fn combine_observations() -> RawImage {
    helper::load_image_raw("data/flowers.png")
}

mod helper {
    use super::*;

    use std::path::Path;

    use image::GenericImageView;
    use log::debug;

    use crate::model::color_map::ColorMap;
    use crate::model::color_map::CompressedImage;
    use crate::model::color_map::RawRgba;

    use super::RawImage;

    pub fn load_image_raw<T: AsRef<Path>>(path: T) -> RawImage {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let raw = img.into_bytes();
        (width, height, raw)
    }

    pub fn load_image_as_8bit<T: AsRef<Path>>(path: T) -> RawImage {
        let (width, height, raw32bit) = load_image_raw(path);

        let image_8bit = (width, height, Vec::new()); // TODO: remove
        let mut compressed_image = CompressedImage::new(width, height);
        let color_map = ColorMap::new();

        if width * height * 4 != raw32bit.len() as u32 {
            panic!("ERROR: Input image has to be in 32 bit RGBA format");
        }

        for y in 0..height {
            for x in 0..width {
                let i = (width * y + x) as usize;
                let rgba = RawRgba(
                    raw32bit[i],
                    raw32bit[i + 1],
                    raw32bit[i + 2],
                    raw32bit[i + 3],
                );

                if let Some(&color_index) = compressed_image.color_map.indices.get(&rgba) {
                    debug!("reusing color {:?} with index {}", rgba, color_index);
                    compressed_image.data[i] = color_index;
                } else if compressed_image.color_map.indices.len() < 256 {
                    let index = compressed_image.color_map.indices.len() as u8;
                    debug!("adding color {:?} with index {}", rgba, index);
                    compressed_image.color_map.indices.insert(rgba, index);
                } else {
                    panic!("ERROR: Input image has more than 256 colors");
                }
            }
        }

        info!(
            "successfully compressed image with {} colors",
            compressed_image.color_map.indices.len()
        );

        image_8bit
    }

    pub fn euclidean_remainder(dividend: i32, divisor: i32) -> i32 {
        // modulo function but with euclidiean division, see https://en.wikipedia.org/wiki/Modulo#Variants_of_the_definition
        let mut remainder = dividend % divisor;
        if remainder < 0 {
            remainder += divisor.abs();
        }
        remainder
    }
}
