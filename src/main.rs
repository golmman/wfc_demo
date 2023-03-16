use std::path::Path;

use image::GenericImageView;

use model::state::State;
use term2d::model::event::Event;
use term2d::model::image::Image;
use term2d::model::key::Key;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use view::renderer::Renderer;

pub mod controller;
pub mod model;
pub mod view;

struct Controller {
    renderer: Renderer,
    state: State,
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

fn main() {
    let img_raw = load_image_raw("data/flowers.png");
    let img = Image::from(img_raw);

    let renderer = Renderer::new();
    let state = State { frame: 0, img };
    let controller = Controller { renderer, state };
    term2d::run(controller);
}
