extern crate ggez;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tar;

mod save;
mod scene;
mod storage;
mod player;
mod objects;
mod misc;
mod constants;

use std::io::Write;
use ggez::{Context, conf, event};

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();
    let scene = &mut scene::Scene::new(ctx).unwrap();

    save::load_scene(scene);

    if let Err(e) = event::run(ctx, scene) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }

    save::save_scene(scene);
}
