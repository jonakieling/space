extern crate ggez;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tar;

mod savegame;
mod game;
mod storage;
mod player;
mod objects;
mod misc;
mod constants;
mod feature;
mod dialog;
mod app;
mod world;

use std::env;
use std::path;
use std::io::Write;

use ggez::{graphics, Context, conf, event::*};

fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.dimensions(768, 512);
    c.window_mode.fullscreen_type(conf::FullscreenType::Desktop);
    c.window_setup = c.window_setup.title("space");
    let mut ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();
    graphics::set_background_color(ctx, graphics::BLACK);
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    if let Ok(ref manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        ctx.filesystem.mount(&path, true);

        let mut path = path::PathBuf::from(manifest_dir);
        path.push("saves");
        ctx.filesystem.mount(&path, true);
	}

    let mut world = world::WorldData::new(&mut ctx);
    let state = Box::new(game::Handler::new(&mut world));
    let app = &mut app::AppContainer {
        state,
        world
    };

    if let Err(e) = run(ctx, app) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }
}
