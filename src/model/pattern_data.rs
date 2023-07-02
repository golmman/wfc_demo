pub struct PatternData {
    pub image_height: u32,
    pub image_width: u32,
    pub pattern_height: u32,
    pub pattern_width: u32,
    pub patterns: Vec<Pattern>,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    // TODO: rename to colors to match wording in PatternPixel
    pub pixels: Vec<u32>,
    pub weight: u32,
}

impl PatternData {
    pub fn get_pixel_index(&self, pattern_index: usize, x: u32, y: u32) -> usize {
        let p = pattern_index;
        let x = x as usize;
        let y = y as usize;
        let w = self.pattern_width as usize;
        let h = self.pattern_height as usize;

        let s1 = w;
        let s2 = w * h;

        p * s2 + y * s1 + x
    }

    pub fn get_relationship_index(
        &self,
        pattern_index: usize,
        x: u32,
        y: u32,
        u: u32,
        v: u32,
    ) -> usize {
        let p = pattern_index;
        let x = x as usize;
        let y = y as usize;
        let u = u as usize;
        let v = v as usize;
        let w = self.pattern_width as usize;
        let h = self.pattern_height as usize;

        let s1 = w;
        let s2 = s1 * h;
        let s3 = s2 * w;
        let s4 = s3 * h;

        p * s4 + y * s3 + x * s2 + v * s1 + u
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_relationship_indices() {
        let w = 5;
        let h = 3;
        let p = 8;

        let pattern_data = PatternData {
            image_height: 0,
            image_width: 0,
            pattern_height: h,
            pattern_width: w,
            patterns: Vec::new(),
        };

        let mut indices = Vec::new();
        for i in 0..p {
            for y in 0..h {
                for x in 0..w {
                    for v in 0..h {
                        for u in 0..w {
                            let index = pattern_data.get_relationship_index(i, x, y, u, v);
                            indices.push(index);
                        }
                    }
                }
            }
        }

        assert_eq!(indices.len(), p * (w * h * w * h) as usize);
        for i in 0..indices.len() {
            assert_eq!(indices[i], i);
        }
    }
}
