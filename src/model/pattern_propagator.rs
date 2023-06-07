use log::info;

use super::pattern_data::PatternData;

pub const CURSOR_UP_LEFT: &'static str = "\x1b[1F";

pub struct PatternPixel {
    pub color: u32,
    pub colors: Vec<u32>,
    pub relationships: Vec<bool>,
    pub weight: u32,
    pub x: u32,
    pub y: u32,
}

pub struct PatternPropagator2 {
    pub pattern_data: PatternData,
    pub pattern_pixels: Vec<PatternPixel>,
    pub total_weight: u32,
}

impl PatternPropagator2 {
    pub fn get_pixel_index(&self, pattern_index: usize, x: u32, y: u32) -> usize {
        let w = self.pattern_data.pattern_width as usize;
        let p = w * self.pattern_data.pattern_height as usize;
        let x = x as usize;
        let y = y as usize;

        pattern_index * p + y * w + x
    }
}

pub struct PatternAdjacency {
    pub pattern: Vec<u32>,
    pub weight: u32,

    neighbors_allowed: Vec<Vec<bool>>,
}

pub struct PatternPropagator {
    pub pattern_adjacencies: Vec<PatternAdjacency>,
    pub pattern_data: PatternData,
}

// TODO: too much calculation for a 'model' struct
impl PatternPropagator {
    pub fn new(pattern_data: PatternData) -> Self {
        info!("building pattern propagator:");
        info!("  calculating pattern adjacencies...");
        println!();
        let pattern_propagator = Self::calculate_pattern_adjacencies(pattern_data);

        info!("  calculating pattern weights...");
        let pattern_propagator = Self::calculate_pattern_weights(pattern_propagator);

        info!("  compressing propagator...");
        let pattern_propagator = Self::compress_pattern_propagator(pattern_propagator);

        info!(
            "number of patterns after compression: {}",
            pattern_propagator.pattern_adjacencies.len()
        );

        pattern_propagator
    }

    fn calculate_pattern_adjacencies(pattern_data: PatternData) -> Self {
        let mut pattern_adjacencies = Vec::new();

        for i in 0..pattern_data.patterns.len() {
            pattern_adjacencies.push(PatternAdjacency::new(&pattern_data, i));

            println!(
                "{}{}%",
                CURSOR_UP_LEFT,
                100 * (i + 1) / pattern_data.patterns.len()
            );
        }
        println!("{}{}", CURSOR_UP_LEFT, CURSOR_UP_LEFT);

        Self {
            pattern_adjacencies,
            pattern_data,
        }
    }

    fn calculate_pattern_weights(pattern_propagator: Self) -> Self {
        let Self {
            pattern_data,
            mut pattern_adjacencies,
        } = pattern_propagator;

        for i in 0..pattern_data.patterns.len() {
            for j in 0..pattern_data.patterns.len() {
                if i == j {
                    continue;
                }

                if Self::patterns_overlap_(&pattern_data, &pattern_adjacencies, i, j, 0, 0) {
                    pattern_adjacencies[i].weight += 1;
                }
            }
        }

        Self {
            pattern_adjacencies,
            pattern_data,
        }
    }

    // TODO: split into smaller functions
    fn compress_pattern_propagator(pattern_propagator: Self) -> Self {
        let total_uncompressed_patterns = pattern_propagator.pattern_data.patterns.len();
        let mut skip_index = vec![false; total_uncompressed_patterns];
        let mut index_map = Self::build_index_map(total_uncompressed_patterns);

        let mut pattern_adjacencies = Vec::new();
        let mut pattern_data = PatternData {
            image_height: pattern_propagator.pattern_data.image_height,
            image_width: pattern_propagator.pattern_data.image_width,
            pattern_height: pattern_propagator.pattern_data.pattern_height,
            pattern_width: pattern_propagator.pattern_data.pattern_width,
            patterns: Vec::new(),
        };

        // calculate weights and index_map
        for i in 0..total_uncompressed_patterns {
            if skip_index[i] {
                continue;
            }

            pattern_adjacencies.push(PatternAdjacency {
                neighbors_allowed: Vec::new(),
                pattern: Vec::new(),
                weight: 0,
            });

            let k = pattern_adjacencies.len() - 1;
            index_map[i] = k;

            for j in i + 1..total_uncompressed_patterns {
                if skip_index[j] {
                    continue;
                }

                if pattern_propagator.patterns_overlap(i, j, 0, 0) {
                    skip_index[j] = true;
                    index_map[j] = k;
                    pattern_adjacencies[k].weight += 1;
                }
            }
        }

        let inverted_index_map = Self::invert_index_map(index_map);

        // TODO: verify/test if this is correct
        // copy pattern and neighbors_allowed according to index_map
        for i in 0..pattern_adjacencies.len() {
            let k = inverted_index_map[i];
            pattern_adjacencies[i].pattern =
                pattern_propagator.pattern_adjacencies[k].pattern.clone();

            for j in 0..pattern_adjacencies.len() {
                let l = inverted_index_map[j];
                pattern_adjacencies[i]
                    .neighbors_allowed
                    .push(pattern_propagator.pattern_adjacencies[k].neighbors_allowed[l].clone());
            }
        }

        // copy pattern_data according to index_map
        for i in 0..pattern_adjacencies.len() {
            let k = inverted_index_map[i];
            pattern_data
                .patterns
                .push(pattern_propagator.pattern_data.patterns[k].clone());
        }

        Self {
            pattern_adjacencies,
            pattern_data,
        }
    }

    fn patterns_overlap_(
        pattern_data: &PatternData,
        pattern_adjacencies: &Vec<PatternAdjacency>,
        this_pattern_index: usize,
        that_pattern_index: usize,
        that_pattern_x: i32,
        that_pattern_y: i32,
    ) -> bool {
        let PatternData {
            pattern_width,
            pattern_height,
            ..
        } = *pattern_data;

        let offset_x = that_pattern_x + pattern_width as i32 - 1;
        let offset_y = that_pattern_y + pattern_height as i32 - 1;
        let offset_index = (pattern_width as i32 * offset_y + offset_x) as usize;

        pattern_adjacencies[this_pattern_index].neighbors_allowed[that_pattern_index][offset_index]
    }

    pub fn patterns_overlap(
        &self,
        this_pattern_index: usize,
        that_pattern_index: usize,
        that_pattern_x: i32,
        that_pattern_y: i32,
    ) -> bool {
        Self::patterns_overlap_(
            &self.pattern_data,
            &self.pattern_adjacencies,
            this_pattern_index,
            that_pattern_index,
            that_pattern_x,
            that_pattern_y,
        )
    }

    fn build_index_map(size: usize) -> Vec<usize> {
        let mut index_map = vec![0 as usize; size];

        for i in 0..size {
            index_map[i] = i;
        }

        index_map
    }

    fn invert_index_map(index_map: Vec<usize>) -> Vec<usize> {
        let mut inverted_index_map = Vec::new();

        for i in 0..index_map.len() {
            let l = inverted_index_map.len();
            if index_map[i] == l {
                inverted_index_map.push(i);
            }
        }

        inverted_index_map
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
            pattern: pattern.pixels,
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
        let x = that_pattern_x;
        let y = that_pattern_y;
        let w = pattern_data.pattern_width as i32;
        let h = pattern_data.pattern_height as i32;
        let this_pattern = &pattern_data.patterns[this_pattern_index];
        let that_pattern = &pattern_data.patterns[that_pattern_index];

        let overlap_width = if x <= 0 { w + x } else { w - x };
        let overlap_height = if y <= 0 { h + y } else { h - y };
        let this_left = if x <= 0 { 0 } else { x };
        let this_top = if y <= 0 { 0 } else { y };
        let that_left = if x <= 0 { -x } else { 0 };
        let that_top = if y <= 0 { -y } else { 0 };

        for v in 0..overlap_height {
            for u in 0..overlap_width {
                let this_u = this_left + u;
                let this_v = this_top + v;
                let that_u = that_left + u;
                let that_v = that_top + v;
                let this_index = (w * this_v + this_u) as usize;
                let that_index = (w * that_v + that_u) as usize;

                if this_pattern.pixels[this_index] != that_pattern.pixels[that_index] {
                    return false;
                }
            }
        }

        true
    }
}
