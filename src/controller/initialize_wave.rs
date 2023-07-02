use std::time::Instant;

use log::info;

use crate::model::pattern_propagator::PatternPropagator;
use crate::model::wave::Wave;

pub fn initialize_wave(
    pattern_propagator: &PatternPropagator,
    target_image_width: u32,
    target_image_height: u32,
) -> Wave {
    info!("initializing wave...");
    let now = Instant::now();

    let mut pixel_indices = Vec::new();
    for i in 0..pattern_propagator.pattern_pixels.len() {
        pixel_indices.push(i);
    }
    let indices = vec![pixel_indices; (target_image_width * target_image_height) as usize];

    info!("  done, took {} ms", now.elapsed().as_millis());
    info!("  wave width: {}", target_image_width);
    info!("  wave height: {}", target_image_height);

    Wave {
        width: target_image_width,
        height: target_image_height,
        indices,
    }
}
