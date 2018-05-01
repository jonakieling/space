extern crate ggez;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tar;

mod level;
mod scene;
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

use scene::*;

enum GameState {
    World,
    Menu
}

struct Game {
    world: world::Scene,
    menu: menu::Scene,
    state: GameState
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            GameState::World => self.world.update(ctx),
            GameState::Menu => self.menu.update(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            GameState::World => self.world.draw(ctx),
            GameState::Menu => self.menu.draw(ctx),
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match self.state {
            GameState::World => self.world.key_down_event(ctx, keycode, keymod, repeat),
            GameState::Menu => self.menu.key_down_event(ctx, keycode, keymod, repeat),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match self.state {
            GameState::World => self.world.key_up_event(ctx, keycode, keymod, repeat),
            GameState::Menu => self.menu.key_up_event(ctx, keycode, keymod, repeat),
        }
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        match self.state {
            GameState::World => self.world.text_input_event(ctx, text),
            GameState::Menu => self.menu.text_input_event(ctx, text),
        }
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
    
    let mut world = world::Scene::new(ctx).unwrap();
    let menu = menu::Scene::new(ctx).unwrap();

    level::load_scene(&mut world, "dev-level.tar");

    let game = &mut Game {
        world,
        menu,
        state: GameState::World
    };

    if let Err(e) = run(ctx, game) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }

    level::save_scene(&mut game.world, "dev-level.tar");
}
