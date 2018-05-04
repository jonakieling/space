use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, MenuOption};
use misc::*;
use level::save_scene;


pub fn key_up_event(scene: &mut Scene, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
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
                MenuOption::Save => {
                    save_scene(scene, "saves/auto-save.tar");
                    scene.input = InputState::World;
                },
                MenuOption::Quit => {
                    ctx.quit().expect("game should have quit");
                },
            }
        },
        _ => ()
    }
}