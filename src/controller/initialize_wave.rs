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
    let last_index_collapsed = fastrand::usize(..indices.len());

    info!("  done, took {} ms", now.elapsed().as_millis());
    info!("  wave width: {}", target_image_width);
    info!("  wave height: {}", target_image_height);

    Wave {
        width: target_image_width,
        height: target_image_height,
        indices,
        last_index_collapsed,
    }
}

#[cfg(test)]
mod tests {
    use crate::model::pattern_data::PatternData;
    use crate::model::pattern_propagator::PatternPixel;

    use super::*;

    #[test]
    fn it_initializes_the_wave() {
        let propagator = PatternPropagator {
            pattern_data: PatternData {
                image_height: 0,
                image_width: 0,
                pattern_height: 0,
                pattern_width: 0,
                patterns: Vec::new(),
            },
            pattern_pixels: vec![
                PatternPixel {
                    color: 0,
                    y: 0,
                    x: 0,
                    weight: 0,
                    colors: Vec::new(),
                    relationships: Vec::new(),
                };
                10
            ],
            total_weight: 1,
        };

        let wave = initialize_wave(&propagator, 7, 7);

        assert_eq!(wave.indices[0][3], 3);
        assert_eq!(wave.indices[4][3], 3);
        assert_eq!(wave.indices[6][3], 3);
        assert_eq!(wave.indices[8][3], 3);
        assert_eq!(wave.indices[48][3], 3);

        assert_eq!(wave.indices[0][7], 7);
        assert_eq!(wave.indices[4][7], 7);
        assert_eq!(wave.indices[6][7], 7);
        assert_eq!(wave.indices[8][7], 7);
        assert_eq!(wave.indices[48][7], 7);
    }
}
