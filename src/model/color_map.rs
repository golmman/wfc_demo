use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RawRgba(pub u8, pub u8, pub u8, pub u8);

pub struct ColorMap {
    pub colors: [RawRgba; 256],
    pub indices: HashMap<RawRgba, u8>,
}

impl ColorMap {
    pub fn new() -> Self {
        Self {
            colors: [RawRgba(0, 0, 0, 0); 256],
            indices: HashMap::new(),
        }
    }
}

pub struct CompressedImage {
    pub color_map: ColorMap,
    pub data: Vec<u8>,
    pub height: u32,
    pub width: u32,
}

impl CompressedImage {
    pub fn new(width: u32, height: u32) -> Self {
        let color_map = ColorMap::new();
        let data = vec![0; (width * height) as usize];

        Self {
            color_map,
            data,
            height,
            width,
        }
    }
}
