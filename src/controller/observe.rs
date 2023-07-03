use crate::model::pattern_propagator::PatternPropagator;
use crate::model::wave::Wave;

pub fn observe(wave: &mut Wave, propagator: &PatternPropagator) {
    //
}

fn find_lowest_entropy(wave: &Wave) -> Option<usize> {
    // TODO: speed up with the knowledge of last iteration
    let lowest_entropy_index = None;

    for i in 0..wave.indices.len() {}

    lowest_entropy_index
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

    fn set(v: Vec<usize>) -> HashSet<usize> {
        v.into_iter().collect()
    }
}
