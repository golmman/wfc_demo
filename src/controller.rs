use std::path::Path;

use image::GenericImageView;

use term2d::model::event::Event;
use term2d::model::image::Image;
use term2d::model::key::Key;
use term2d::view::canvas::halfblock::HalfblockCanvas;

use crate::model::args::Args;
use crate::model::state::State;
use crate::view::renderer::Renderer;

pub mod wfc;

pub struct Controller {
    renderer: Renderer,
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        let args = Args {
            path: "data/flowers.png",
            pattern_width: 3,
            pattern_height: 3,
            target_image_width: 100,
            target_image_height: 100,
        };

        let img_raw = wfc::run(args);
        let img = Image::from(img_raw);

        let renderer = Renderer::new();
        let state = State { frame: 0, img };
        Self { renderer, state }
    }
}

impl term2d::controller::Controller<HalfblockCanvas> for Controller {
    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,
                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {}
        }

        self.renderer.display(&self.state);

        self.state.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.renderer.canvas
    }
}

fn load_image_raw<T: AsRef<Path>>(path: T) -> (u32, u32, Vec<u8>) {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    let raw = img.into_bytes();
    (width, height, raw)
}
