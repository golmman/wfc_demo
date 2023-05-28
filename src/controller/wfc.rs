use std::path::Path;

use log::info;

use crate::model::args::Args;
use crate::model::pattern_data::PatternData;
use crate::model::pattern_propagator::PatternPropagator;
use crate::model::raw_image::RawImage;
use crate::model::wave::Wave;

use super::extract_patterns::extract_patterns;

pub fn run<T: AsRef<Path>>(args: Args<T>) -> RawImage {
    let Args {
        path,
        pattern_width,
        pattern_height,
        target_image_width,
        target_image_height,
    } = args;

    let pattern_data = extract_patterns(path, pattern_width, pattern_height);
    let pattern_propagator = build_propagator(pattern_data);
    let wave = initialize_wave(&pattern_propagator, target_image_width, target_image_height); // this is not mentioned in the original implementation

    for i in 0..10 {
        observe();
        propagate();
    }

    combine_observations()
}

fn initialize_wave(
    pattern_propagator: &PatternPropagator,
    target_image_width: u32,
    target_image_height: u32,
) -> Wave {
    info!("initializing wave...");

    let mut wave = Wave {
        width: target_image_width,
        height: target_image_height,
        indices: Vec::new(),
    };

    for i in 0..(wave.width*wave.height) as usize {
        wave.indices.push(Vec::new());
        for j in 0..pattern_propagator.pattern_adjacencies.len() {
            wave.indices[i].push(j);
        }
    }

    wave
}

fn build_propagator(pattern_data: PatternData) -> PatternPropagator {
    PatternPropagator::new(pattern_data)
}

fn observe() {}

fn find_lowest_entropy(wave: &Wave) -> Option<usize> {
    // TODO: speed up with the knowledge of last iteration
    let lowest_entropy_index = None;

    for i in 0..wave.indices.len() {

    }

    lowest_entropy_index
}

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

    pub fn load_image<T: AsRef<Path>>(path: T) -> (u32, u32, Vec<u32>) {
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
                    raw32bit[4 * i],
                    raw32bit[4 * i + 1],
                    raw32bit[4 * i + 2],
                    raw32bit[4 * i + 3],
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

    pub fn euclidean_remainder(dividend: i32, divisor: i32) -> u32 {
        // modulo function but with euclidiean division, see https://en.wikipedia.org/wiki/Modulo#Variants_of_the_definition
        let mut remainder = dividend % divisor;
        if remainder < 0 {
            remainder += divisor.abs();
        }
        remainder as u32
    }
}
