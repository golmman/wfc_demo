use controller::Controller;

pub mod controller;
pub mod model;
pub mod view;

fn main() {
    let controller = Controller::new();
    term2d::run(controller);
}
