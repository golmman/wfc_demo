use model::args::Args;

pub mod controller;
pub mod model;
pub mod view;

fn main() {
    fastrand::seed(7);
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = Args {
        path: "data/flowers.png",
        pattern_width: 3,
        pattern_height: 3,
        target_image_width: 10,
        target_image_height: 10,
    };

    controller::wfc::run(args);

    //let controller = Controller::new();
    //term2d::run(controller);
}
