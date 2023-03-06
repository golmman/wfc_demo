use std::path::Path;

use image::GenericImageView;

use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::image::Image;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

struct Controller {
    frame: u32,
    canvas: HalfblockCanvas,
    img: Image,
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

        self.canvas.clear();

        self.canvas.draw_text(
            &Point::new(2, 0),
            &Color {
                fg: Rgba::white(),
                bg: Rgba::transparent(),
            },
            &format!("press 'q' to quit, frame: {}", self.frame),
        );
        self.canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
        self.canvas.draw_image(&Point::new(30, 3), &mut self.img);

        self.canvas.display();

        self.frame += 1;

        true
    }

    fn get_canvas(&mut self) -> &mut HalfblockCanvas {
        &mut self.canvas
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
    let controller = Controller {
        frame: 0,
        canvas: HalfblockCanvas::new(),
        img,
    };
    term2d::run(controller);
}
