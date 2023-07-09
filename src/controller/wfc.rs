use std::path::Path;

use crate::model::args::Args;
use crate::model::image::RawImage;
use crate::model::pattern_propagator::PatternPropagator;
use crate::model::wave::Wave;

use super::build_propagator::build_propagator;
use super::extract_patterns::extract_patterns;
use super::initialize_wave::initialize_wave;
use super::load_image::load_image;
use super::load_image::load_image_raw;
use super::observe::observe;

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
    let mut wave = initialize_wave(&pattern_propagator, target_image_width, target_image_height);

    //for i in 0..10 {
    //    if !observe(&mut wave, &pattern_propagator) {
    //        break;
    //    }
    //    propagate();
    //}

    while observe(&mut wave, &pattern_propagator) {
        propagate();
    }

    combine_observations()
}

fn propagate() {}

fn combine_observations() -> RawImage {
    load_image_raw("data/flowers.png")
}
