extern crate ggez;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tar;
extern crate specs;
#[macro_use]
extern crate specs_derive;

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
mod components;
mod systems;

use std::env;
use std::path;
use std::io::Write;

use ggez::{graphics, conf, event::*, ContextBuilder};

fn main() {
    let screen_height = 512;
    let screen_width = 768;

    let cb = ContextBuilder::new("space", "jonakieling")
        .window_setup(conf::WindowSetup::default().title("space"))
        .window_mode(conf::WindowMode::default().dimensions(screen_width, screen_height));

    let mut ctx = &mut cb.build().unwrap();
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
