use log::info;

use super::pattern_data::PatternData;

#[derive(Clone)]
pub struct PatternPixel {
    pub color: u32,
    pub colors: Vec<u32>,
    pub relationships: Vec<bool>,
    pub weight: u32,
    pub x: u32,
    pub y: u32,
}

pub struct PatternPropagator {
    pub pattern_data: PatternData,
    pub pattern_pixels: Vec<PatternPixel>,
    pub total_weight: u32,
}
