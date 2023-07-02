use std::path::Path;

use image::GenericImageView;

use crate::model::image::Image;
use crate::model::image::RawImage;

pub fn load_image<T: AsRef<Path>>(path: T) -> Image {
    let (width, height, raw32bit) = load_image_raw(path);
    let mut data = vec![0; (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let i = (width * y + x) as usize;
            let rgba = (raw32bit[4 * i] as u32)
                | (raw32bit[4 * i + 1] as u32) << 8
                | (raw32bit[4 * i + 2] as u32) << 16
                | (raw32bit[4 * i + 3] as u32) << 24;

            data[i] = rgba;
        }
    }

    Image {
        width,
        height,
        data,
    }
}

pub fn load_image_raw<T: AsRef<Path>>(path: T) -> RawImage {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    let raw = img.into_bytes();
    (width, height, raw)
}
