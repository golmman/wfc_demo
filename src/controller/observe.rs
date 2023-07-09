use crate::model::pattern_propagator::PatternPropagator;
use crate::model::wave::Wave;

pub fn observe(wave: &mut Wave, propagator: &PatternPropagator) -> bool {
    if let Some(i) = find_lowest_entropy_index(wave, propagator) {
        collapse_wave_entry(i, wave, propagator);
        return true;
    }

    false
}

fn collapse_wave_entry(index: usize, wave: &mut Wave, propagator: &PatternPropagator) {
    let mut weighted_pixel_indices = Vec::new();

    for i in 0..wave.indices[index].len() {
        let pi = wave.indices[index][i];
        let weight = propagator.pattern_pixels[pi].weight;

        for _j in 0..weight {
            weighted_pixel_indices.push(pi);
        }
    }

    let k = fastrand::usize(..weighted_pixel_indices.len());
    let chosen_pixel_index = weighted_pixel_indices[k];

    wave.indices[index] = vec![chosen_pixel_index];
    wave.last_index_collapsed = index;
}

fn find_lowest_entropy_index(wave: &Wave, propagator: &PatternPropagator) -> Option<usize> {
    let total_weight = propagator.total_weight as f32;
    let collapsed_neighborhood = get_last_collapsed_neighborhood(wave);

    let mut lowest_entropy_index = None;
    let mut lowest_entropy_value = f32::MAX;

    for &i in &collapsed_neighborhood {
        if wave.indices[i].len() == 1 {
            continue;
        }

        let entropy = calculate_entropy(&wave.indices[i], propagator, total_weight);

        if entropy < lowest_entropy_value {
            lowest_entropy_index = Some(i);
            lowest_entropy_value = entropy;
        }
    }

    if lowest_entropy_index.is_none() {
        lowest_entropy_value = f32::MAX;

        for (i, indices) in wave.indices.iter().enumerate() {
            if indices.len() == 1 {
                continue;
            }

            let entropy = calculate_entropy(indices, propagator, total_weight);

            if entropy < lowest_entropy_value {
                lowest_entropy_index = Some(i);
                lowest_entropy_value = entropy;
            }
        }
    }

    lowest_entropy_index
}

fn calculate_entropy(indices: &[usize], propagator: &PatternPropagator, total_weight: f32) -> f32 {
    let mut entropy = 0.0;
    for &pi in indices {
        let weight = propagator.pattern_pixels[pi].weight as f32;
        let prob = weight / total_weight;
        entropy -= prob * prob.ln();
    }
    entropy
}

fn get_last_collapsed_neighborhood(wave: &Wave) -> Vec<usize> {
    calculate_adjacent_indices(
        wave.width as usize,
        wave.height as usize,
        wave.last_index_collapsed,
    )
}

fn calculate_adjacent_indices(width: usize, height: usize, index: usize) -> Vec<usize> {
    if index >= width * height {
        panic!("index overflow");
    }

    let mut adjacent_indices = Vec::with_capacity(8);

    let row = index / width;
    let col = index % width;

    let has_top_row = row > 0;
    let has_bottom_row = row < height - 1;
    let has_left_col = col > 0;
    let has_right_col = col < width - 1;

    // Top neighbor
    if has_top_row {
        adjacent_indices.push(index - width);

        // Top-left neighbor
        if has_left_col {
            adjacent_indices.push(index - width - 1);
        }

        // Top-right neighbor
        if has_right_col {
            adjacent_indices.push(index - width + 1);
        }
    }

    // Bottom neighbor
    if has_bottom_row {
        adjacent_indices.push(index + width);

        // Bottom-left neighbor
        if has_left_col {
            adjacent_indices.push(index + width - 1);
        }

        // Bottom-right neighbor
        if has_right_col {
            adjacent_indices.push(index + width + 1);
        }
    }

    // Left neighbor
    if has_left_col {
        adjacent_indices.push(index - 1);
    }

    // Right neighbor
    if has_right_col {
        adjacent_indices.push(index + 1);
    }

    adjacent_indices
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::model::pattern_data::PatternData;
    use crate::model::pattern_propagator::PatternPixel;

    use super::*;

    #[test]
    fn it_calculates_adjacent_indices() {
        // Example, width = 5, height = 4
        //  0  1  2  3  4
        //  5  6  7  8  9
        // 10 11 12 13 14
        // 15 16 17 18 19
        let width = 5;
        let height = 4;

        // corners
        let indices = set(calculate_adjacent_indices(width, height, 0));
        assert_eq!(indices, set(vec![1, 5, 6]));
        let indices = set(calculate_adjacent_indices(width, height, 4));
        assert_eq!(indices, set(vec![3, 8, 9]));
        let indices = set(calculate_adjacent_indices(width, height, 15));
        assert_eq!(indices, set(vec![10, 11, 16]));
        let indices = set(calculate_adjacent_indices(width, height, 19));
        assert_eq!(indices, set(vec![13, 14, 18]));

        // edges
        let indices = set(calculate_adjacent_indices(width, height, 1));
        assert_eq!(indices, set(vec![0, 2, 5, 6, 7]));
        let indices = set(calculate_adjacent_indices(width, height, 14));
        assert_eq!(indices, set(vec![8, 9, 13, 18, 19]));
        let indices = set(calculate_adjacent_indices(width, height, 17));
        assert_eq!(indices, set(vec![11, 12, 13, 16, 18]));
        let indices = set(calculate_adjacent_indices(width, height, 5));
        assert_eq!(indices, set(vec![0, 1, 6, 10, 11]));

        // center
        let indices = set(calculate_adjacent_indices(width, height, 6));
        assert_eq!(indices, set(vec![0, 1, 2, 5, 7, 10, 11, 12]));
        let indices = set(calculate_adjacent_indices(width, height, 12));
        assert_eq!(indices, set(vec![6, 7, 8, 11, 13, 16, 17, 18]));
    }

    #[test]
    #[should_panic]
    fn it_panics_when_the_calculation_for_adjacent_indices_overflows() {
        let width = 5;
        let height = 4;
        calculate_adjacent_indices(width, height, 100);
    }

    #[test]
    fn it_finds_the_lowest_entropy_in_an_8_neighborhood() {
        let propagator = create_propagator_with_simple_weights();

        let wave = Wave {
            width: 3,
            height: 2,
            last_index_collapsed: 0,
            indices: vec![
                vec![0],
                vec![2, 2, 2],
                vec![4, 1],
                vec![4, 1, 1],
                vec![1, 1, 1, 1, 1, 1],
                vec![0],
            ],
        };

        // Note that wave elements in the neighborhood all have a summed weight of 6
        // but index 3 wins since it is "densest".
        // Also note that index 2 has lowest (non-singular) global entropy but is not picked
        // since it is not in the neighborhood.
        let lowest_entropy_index = find_lowest_entropy_index(&wave, &propagator);
        assert_eq!(lowest_entropy_index, Some(3));
    }

    #[test]
    fn it_extends_the_lowest_entropy_search_to_the_entire_wave() {
        let propagator = create_propagator_with_simple_weights();

        let wave = Wave {
            width: 3,
            height: 2,
            last_index_collapsed: 0,
            indices: vec![vec![0], vec![1], vec![4, 1], vec![1], vec![1], vec![3, 2]],
        };

        let lowest_entropy_index = find_lowest_entropy_index(&wave, &propagator);
        assert_eq!(lowest_entropy_index, Some(2));
    }

    fn set(v: Vec<usize>) -> HashSet<usize> {
        v.into_iter().collect()
    }

    fn create_pattern_pixel_with_weight(weight: u32) -> PatternPixel {
        PatternPixel {
            color: 0,
            x: 0,
            y: 0,
            weight,
            relationships: Vec::new(),
            colors: Vec::new(),
        }
    }

    fn create_propagator_with_simple_weights() -> PatternPropagator {
        PatternPropagator {
            pattern_data: PatternData {
                image_height: 0,
                image_width: 0,
                pattern_height: 0,
                pattern_width: 0,
                patterns: Vec::new(),
            },
            pattern_pixels: vec![
                create_pattern_pixel_with_weight(1),
                create_pattern_pixel_with_weight(1),
                create_pattern_pixel_with_weight(2),
                create_pattern_pixel_with_weight(3),
                create_pattern_pixel_with_weight(4),
                create_pattern_pixel_with_weight(5),
            ],
            total_weight: 16,
        }
    }
}
