use std::hash::Hash;
use std::hash::Hasher;

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

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.pixels == other.pixels
    }
}
impl Eq for Pattern {}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pixels.hash(state);
    }
}
