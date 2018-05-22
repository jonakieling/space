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
mod feature;
mod dialog;
mod app;
mod world;

use std::env;
use std::path;
use std::io::Write;

use ggez::{graphics, Context, conf, event::*, GameResult};

use app_state::*;
use world::WorldData;

pub trait GameState {
    fn change_state(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> Option<Box<GameState>> { None }
    
    fn update(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> GameResult<()> { Ok(()) }

    fn draw(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> GameResult<()> { Ok(()) }

    fn key_down_event(&mut self, _ctx: &mut Context, _world: &mut WorldData, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn key_up_event(&mut self, _ctx: &mut Context, _world: &mut WorldData, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn text_input_event(&mut self, _ctx: &mut Context, _world: &mut WorldData, _text: String) { }

    fn quit_event(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> bool { false }
}

fn main() {
    let c = conf::Conf::new();
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

    let game = &mut app::AppContainer {
        state: Box::new(menu::Handler::new().unwrap()),
        world: world::WorldData::new(&mut ctx)
    };

    if let Err(e) = run(ctx, game) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
    }
}
