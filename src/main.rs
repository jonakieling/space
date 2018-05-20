extern crate ggez;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tar;

mod savegame;
mod app_state;
mod storage;
mod player;
mod objects;
mod misc;
mod constants;
mod ingame_state;
mod dialog;

use std::env;
use std::path;
use std::io::Write;

use ggez::{graphics, Context, conf, event::*};

use app_state::*;

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();
    graphics::set_background_color(ctx, graphics::BLACK);

    if let Ok(ref manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        ctx.filesystem.mount(&path, true);

        let mut path = path::PathBuf::from(manifest_dir);
        path.push("saves");
        ctx.filesystem.mount(&path, true);
	}
    
    let menu = menu::Scene::new().unwrap();

    let game = &mut App {
        state: Box::new(menu)
    };

    if let Err(e) = run(ctx, game) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }
}
