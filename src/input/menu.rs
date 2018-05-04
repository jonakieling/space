use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, MenuOption};
use misc::*;


pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Escape => {
            scene.input = InputState::World;
        },
        Keycode::Up => {
            scene.menu.prev();
        },
        Keycode::Down => {
            scene.menu.next();
        },
        Keycode::Return => {
            match *scene.menu.current().unwrap() {
                MenuOption::Save => println!("saving"),
                MenuOption::Quit => println!("quitting"),
            }
        },
        _ => ()
    }
}