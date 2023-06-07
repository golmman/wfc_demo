use crate::model::pattern_data::PatternData;
use crate::model::pattern_propagator::PatternPixel;
use crate::model::pattern_propagator::PatternPropagator2;

pub fn build_propagator(pattern_data: PatternData) -> PatternPropagator2 {
    let propagator = initialize_pixels(pattern_data);

    let propagator = calculate_relationships(propagator);

    propagator
}

fn initialize_pixels(pattern_data: PatternData) -> PatternPropagator2 {
    let mut pattern_pixels = Vec::new();
    let PatternData {
        ref patterns,
        image_height,
        image_width,
        pattern_width,
        pattern_height,
    } = pattern_data;
    let total_weight = pattern_width * pattern_height * image_width * image_height;
    let total_pixels = patterns.len() * (pattern_width * pattern_height) as usize;

    for i in 0..patterns.len() {
        for y in 0..pattern_height {
            for x in 0..pattern_width {
                let j = (pattern_width * y + x) as usize;
                pattern_pixels.push(PatternPixel {
                    color: patterns[i].pixels[j],
                    colors: patterns[i].pixels.clone(),
                    relationships: vec![false; total_pixels],
                    weight: patterns[i].weight,
                    x,
                    y,
                });
            }
        }
    }

    let propagator = PatternPropagator2 {
        pattern_data,
        pattern_pixels,
        total_weight,
    };

    propagator
}

fn calculate_relationships(pattern_propagator: PatternPropagator2) -> PatternPropagator2 {
    pattern_propagator
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::extract_patterns::extract_patterns;

    #[test]
    fn it_initializes_pattern_pixels() {
        let pattern_data = extract_patterns("./data/flowers.png", 3, 3);
        let pattern_propagator = initialize_pixels(pattern_data);
        let pattern_pixels = pattern_propagator.pattern_pixels;

        let mut total_weight = 0;
        for i in 0..pattern_pixels.len() {
            assert_eq!(pattern_pixels[i].relationships.len(), pattern_pixels.len());
            total_weight += pattern_pixels[i].weight;
        }

        assert_eq!(pattern_propagator.total_weight, total_weight);
    }
}
