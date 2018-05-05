extern crate ggez;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tar;

mod level;
mod state;
mod storage;
mod player;
mod objects;
mod misc;
mod constants;
mod input;
mod dialog;

use std::env;
use std::path;
use std::io::Write;

use ggez::{Context, conf, event::*};

use state::*;

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();

    if let Ok(ref manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        ctx.filesystem.mount(&path, true);

        let mut path = path::PathBuf::from(manifest_dir);
        path.push("saves");
        ctx.filesystem.mount(&path, true);
	}
    
    let menu = menu::Scene::new(ctx).unwrap();

    let game = &mut Game {
        state: Box::new(menu)
    };

    if let Err(e) = run(ctx, game) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }
}
