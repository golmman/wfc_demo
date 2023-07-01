use std::time::Instant;

use log::info;

use crate::model::{pattern_propagator::PatternPropagator, wave::Wave};

pub fn initialize_wave(
    pattern_propagator: &PatternPropagator,
    target_image_width: u32,
    target_image_height: u32,
) -> Wave {
    info!("initializing wave...");
    let now = Instant::now();

    let mut wave = Wave {
        width: target_image_width,
        height: target_image_height,
        indices: Vec::new(),
    };

    for i in 0..(wave.width * wave.height) as usize {
        wave.indices.push(Vec::new());
        for j in 0..pattern_propagator.pattern_pixels.len() {
            wave.indices[i].push(j);
        }
    }

    info!("  done, took {} ms", now.elapsed().as_millis());
    info!("  wave width: {}", wave.width);
    info!("  wave height: {}", wave.height);
    wave
}
