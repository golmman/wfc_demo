use std::path::Path;

pub fn run<T: AsRef<Path>>(path: T) {
    extract_patterns(path);
    build_propagator();

    for i in 0..10 {
        observe();
        propagate();
    }

    combine_observations();
}

fn extract_patterns<T: AsRef<Path>>(path: T) {}

fn build_propagator() {}

fn observe() {}

fn propagate() {}

fn combine_observations() {}

mod helper {
    use std::path::Path;

    use image::GenericImageView;

    fn load_image_raw<T: AsRef<Path>>(path: T) -> (u32, u32, Vec<u8>) {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let raw = img.into_bytes();
        (width, height, raw)
    }
}
