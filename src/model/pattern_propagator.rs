use log::info;

use super::pattern_data::PatternData;

pub struct PatternAdjacency {
    pub pattern: Vec<u32>,
    pub weight: u32,

    neighbors_allowed: Vec<Vec<bool>>,
}

pub struct PatternPropagator {
    pattern_data: PatternData,
    pattern_neighborhoods: Vec<PatternAdjacency>,
}

impl PatternPropagator {
    pub fn new(pattern_data: PatternData) -> Self {
        let mut pattern_neighborhoods = Vec::new();

        for i in 0..pattern_data.patterns.len() {
            pattern_neighborhoods.push(PatternAdjacency::new(&pattern_data, i));
        }

        info!(
            "number of adjacencies per pattern: {} (should equal patterns * (2*pattern_w-1) * (2*pattern_h-1))",
            pattern_neighborhoods[0].neighbors_allowed.len()
                * pattern_neighborhoods[0].neighbors_allowed[0].len()
        );

        Self {
            pattern_data,
            pattern_neighborhoods,
        }
    }
}

impl PatternAdjacency {
    pub fn new(pattern_data: &PatternData, self_index: usize) -> Self {
        let PatternData {
            patterns,
            pattern_width,
            pattern_height,
            ..
        } = pattern_data;

        let total_offsets = (2 * pattern_width - 1) * (2 * pattern_height - 1);
        let pattern = patterns[self_index].clone();
        let weight = 1;
        let mut neighbors_allowed = Vec::new();

        let left = 1 - *pattern_width as i32;
        let right = *pattern_width as i32;
        let top = 1 - *pattern_height as i32;
        let bottom = *pattern_height as i32;

        for pattern_index in 0..patterns.len() {
            neighbors_allowed.push(vec![false; total_offsets as usize]);
            for y in top..bottom {
                for x in left..right {
                    let offset_x = x + *pattern_width as i32 - 1;
                    let offset_y = y + *pattern_height as i32 - 1;
                    let offset_index = (*pattern_width as i32 * offset_y + offset_x) as usize;
                    neighbors_allowed[pattern_index][offset_index] =
                        Self::is_overlapping_match(&pattern_data, self_index, pattern_index, x, y);
                }
            }
        }

        Self {
            pattern,
            weight,
            neighbors_allowed,
        }
    }

    fn is_overlapping_match(
        pattern_data: &PatternData,
        this_pattern_index: usize,
        that_pattern_index: usize,
        that_pattern_x: i32,
        that_pattern_y: i32,
    ) -> bool {
        true
    }
}
