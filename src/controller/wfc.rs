use std::path::Path;

use crate::model::args::Args;
use crate::model::image::RawImage;
use crate::model::wave::Wave;

use super::build_propagator::build_propagator;
use super::extract_patterns::extract_patterns;
use super::initialize_wave::initialize_wave;
use super::load_image::load_image;
use super::load_image::load_image_raw;

pub fn run<T: AsRef<Path>>(args: Args<T>) -> RawImage {
    let Args {
        path,
        pattern_width,
        pattern_height,
        target_image_width,
        target_image_height,
    } = args;

    let image = load_image(path);

    let pattern_data = extract_patterns(image, pattern_width, pattern_height);
    let pattern_propagator = build_propagator(pattern_data);
    let wave = initialize_wave(&pattern_propagator, target_image_width, target_image_height);

    for i in 0..10 {
        observe();
        propagate();
    }

    combine_observations()
}

fn observe() {}

fn find_lowest_entropy(wave: &Wave) -> Option<usize> {
    // TODO: speed up with the knowledge of last iteration
    let lowest_entropy_index = None;

    for i in 0..wave.indices.len() {}

    lowest_entropy_index
}

fn propagate() {}

fn combine_observations() -> RawImage {
    load_image_raw("data/flowers.png")
}
