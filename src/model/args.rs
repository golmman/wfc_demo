use std::path::Path;

pub struct Args<T: AsRef<Path>> {
    pub path: T,
    pub pattern_width: u32,
    pub pattern_height: u32,
    pub target_image_width: u32,
    pub target_image_height: u32,
}
