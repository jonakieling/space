extern crate ggez;
extern crate serde;
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

use std::env;
use std::path;
use std::io::Write;

use ggez::{Context, conf, event::*, GameResult};

use state::*;

trait GameState: EventHandler {
    // add code here
}

struct Game {
    state: Box<GameState>
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.draw(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.state.key_down_event(ctx, keycode, keymod, repeat)
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.state.key_up_event(ctx, keycode, keymod, repeat)
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        self.state.text_input_event(ctx, text)
    }

    fn quit_event(&mut self, ctx: &mut Context) -> bool {
        self.state.quit_event(ctx)
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        ctx.filesystem.mount(&path, true);
	}
    
    // let menu = menu::Scene::new(ctx).unwrap();
    let mut world = world::Scene::new(ctx).unwrap();
    level::load_scene(&mut world, "auto-save.tar");

    let game = &mut Game {
        state: Box::new(world)
    };

    if let Err(e) = run(ctx, game) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }
}
