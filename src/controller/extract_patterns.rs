use std::collections::HashMap;
use std::time::Instant;

use crate::model::image::Image;
use crate::model::pattern_data::Pattern;
use crate::model::pattern_data::PatternData;

use log::info;

pub fn extract_patterns(image: Image, pattern_width: u32, pattern_height: u32) -> PatternData {
    info!("extracting patterns...");
    let now = Instant::now();

    let mut pattern_index_map: HashMap<Vec<u32>, usize> = HashMap::new();
    let mut patterns: Vec<Pattern> = Vec::new();
    let Image {
        width: image_width,
        height: image_height,
        data: image_data,
    } = image;

    for image_y in 0..image_height {
        for image_x in 0..image_width {
            let mut pixels = vec![0; (pattern_width * pattern_height) as usize];

            for pattern_y in 0..pattern_height {
                for pattern_x in 0..pattern_width {
                    let scan_x = (image_x + pattern_x) % image_width;
                    let scan_y = (image_y + pattern_y) % image_height;
                    let image_index = (image_width * scan_y + scan_x) as usize;
                    let pattern_index = (pattern_width * pattern_y + pattern_x) as usize;
                    pixels[pattern_index] = image_data[image_index];
                }
            }

            if let Some(i) = pattern_index_map.get(&pixels) {
                patterns[*i].weight += 1;
            } else {
                pattern_index_map.insert(pixels.clone(), patterns.len());
                patterns.push(Pattern { pixels, weight: 1 });
            }
        }
    }

    let mut weight_sum = 0;
    for i in 0..patterns.len() {
        weight_sum += patterns[i].weight;
    }

    info!("  done, took {} ms", now.elapsed().as_millis());
    info!("  pattern width: {}", pattern_width);
    info!("  pattern height: {}", pattern_height);
    info!("  image width: {}", image_width);
    info!("  image height: {}", image_height);
    info!("  number of unique patterns: {}", patterns.len());
    info!(
        "  sum of pattern weights: {} (should equal image_w * image_h)",
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
    use crate::controller::load_image::load_image;

    use super::*;

    #[test]
    fn it_extracts_patterns() {
        let image = load_image("./data/flowers.png");
        let image_size = image.width * image.height;
        let pattern_data = extract_patterns(image, 3, 3);
        let total_unique_patterns = 92;

        let mut weight_sum = 0;
        for i in 0..pattern_data.patterns.len() {
            weight_sum += pattern_data.patterns[i].weight;
        }

        assert_eq!(pattern_data.patterns.len(), total_unique_patterns);
        assert_eq!(weight_sum, image_size);
    }

    #[test]
    fn it_extracts_patterns_in_proper_order() {
        let pattern_width = 3;
        let pattern_height = 2;
        let image = Image {
            width: 4,
            height: 3,
            #[rustfmt::skip]
            data: vec![
                0, 1, 2, 3,
                4, 5, 6, 7,
                8, 9, 10, 11,
            ],
        };

        let pattern_data = extract_patterns(image, pattern_width, pattern_height);

        assert_eq!(pattern_data.patterns[0].pixels, vec![0, 1, 2, 4, 5, 6]);
        assert_eq!(pattern_data.patterns[1].pixels, vec![1, 2, 3, 5, 6, 7]);
        assert_eq!(pattern_data.patterns[2].pixels, vec![2, 3, 0, 6, 7, 4]);
        assert_eq!(pattern_data.patterns[3].pixels, vec![3, 0, 1, 7, 4, 5]);

        assert_eq!(pattern_data.patterns[4].pixels, vec![4, 5, 6, 8, 9, 10]);
        assert_eq!(pattern_data.patterns[5].pixels, vec![5, 6, 7, 9, 10, 11]);
        assert_eq!(pattern_data.patterns[6].pixels, vec![6, 7, 4, 10, 11, 8]);
        assert_eq!(pattern_data.patterns[7].pixels, vec![7, 4, 5, 11, 8, 9]);

        assert_eq!(pattern_data.patterns[8].pixels, vec![8, 9, 10, 0, 1, 2]);
        assert_eq!(pattern_data.patterns[9].pixels, vec![9, 10, 11, 1, 2, 3]);
        assert_eq!(pattern_data.patterns[10].pixels, vec![10, 11, 8, 2, 3, 0]);
        assert_eq!(pattern_data.patterns[11].pixels, vec![11, 8, 9, 3, 0, 1]);
    }

    #[test]
    fn it_extracts_patterns_without_duplicates() {
        let pattern_width = 3;
        let pattern_height = 2;
        let image = Image {
            width: 4,
            height: 3,
            #[rustfmt::skip]
            data: vec![
                0, 1, 2, 3,
                4, 5, 6, 7,
                8, 9, 10, 11,
            ],
        };

        let pattern_data = extract_patterns(image, pattern_width, pattern_height);
        assert_eq!(pattern_data.patterns.len(), 12);
        for i in 0..pattern_data.patterns.len() {
            assert_eq!(pattern_data.patterns[i].weight, 1);
        }
    }

    #[test]
    fn it_extracts_patterns_with_all_duplicates() {
        let pattern_width = 3;
        let pattern_height = 2;
        let image = Image {
            width: 4,
            height: 3,
            #[rustfmt::skip]
            data: vec![
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        };

        let pattern_data = extract_patterns(image, pattern_width, pattern_height);
        assert_eq!(pattern_data.patterns.len(), 1);
        for i in 0..pattern_data.patterns.len() {
            assert_eq!(pattern_data.patterns[i].weight, 12);
        }
    }

    #[test]
    fn it_extracts_patterns_with_half_duplicates() {
        let pattern_width = 3;
        let pattern_height = 2;
        let image = Image {
            width: 4,
            height: 3,
            #[rustfmt::skip]
            data: vec![
                0, 1, 0, 1,
                2, 3, 2, 3,
                4, 5, 4, 5,
            ],
        };

        let pattern_data = extract_patterns(image, pattern_width, pattern_height);
        assert_eq!(pattern_data.patterns.len(), 6);
        for i in 0..pattern_data.patterns.len() {
            assert_eq!(pattern_data.patterns[i].weight, 2);
        }
    }
}
