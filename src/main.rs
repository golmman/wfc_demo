use controller::Controller;

pub mod controller;
pub mod model;
pub mod view;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    controller::wfc::run("data/flowers.png", 3, 3);

    //let controller = Controller::new();
    //term2d::run(controller);
}
