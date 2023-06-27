use std::collections::HashSet;
use std::path::Path;

use crate::controller::load_image::load_image;
use crate::model::image::Image;
use crate::model::pattern_data::Pattern;
use crate::model::pattern_data::PatternData;
use crate::model::image::RawImage;

use log::info;

pub fn extract_patterns(
    image: Image,
    pattern_width: u32,
    pattern_height: u32,
) -> PatternData {
    let mut pattern_set: HashSet<Pattern> = HashSet::new();
    let Image {
        width: image_width,
        height: image_height,
        data: image_data,
    } = image;

    for image_y in 0..image_width {
        for image_x in 0..image_height {
            let mut pixels = vec![0; (pattern_width * pattern_height) as usize];

            for pattern_y in 0..pattern_width {
                for pattern_x in 0..pattern_width {
                    let scan_x = (image_x + pattern_x) % image_width;
                    let scan_y = (image_y + pattern_y) % image_height;
                    let image_index = (image_width * scan_y + scan_x) as usize;
                    let pattern_index = (pattern_width * pattern_y + pattern_x) as usize;
                    pixels[pattern_index] = image_data[image_index];
                }
            }

            let mut pattern = Pattern { pixels, weight: 1 };
            if let Some(p) = pattern_set.get(&pattern) {
                pattern.weight = p.weight + 1;
            }

            pattern_set.replace(pattern);
        }
    }

    let patterns: Vec<Pattern> = pattern_set.into_iter().collect();

    let mut weight_sum = 0;
    for i in 0..patterns.len() {
        weight_sum += patterns[i].weight;
    }

    info!("pattern width: {}", pattern_width);
    info!("pattern height: {}", pattern_height);
    info!("image width: {}", image_width);
    info!("image height: {}", image_height);
    info!("number of unique patterns: {}", patterns.len());
    info!(
        "sum of pattern weights: {} (should equal image_w * image_h)",
        weight_sum
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_patterns() {
        let image = load_image("./data/flowers.png");
        let image_size = image.width * image.height;
        let pattern_data = extract_patterns(image, 3, 3);
        let total_unique_patterns = 76;

        let mut weight_sum = 0;
        for i in 0..pattern_data.patterns.len() {
            weight_sum += pattern_data.patterns[i].weight;
        }

        assert_eq!(pattern_data.patterns.len(), total_unique_patterns);
        assert_eq!(weight_sum, image_size);
    }
}
