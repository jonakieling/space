use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, InputState};

pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Escape => {
            scene.reset_craft_area();
            scene.input = InputState::World;
        },
        Keycode::I => {
            scene.reset_craft_area();
            scene.input = InputState::World;
        },
        Keycode::Up => {
            scene.player.inventory.prev();
        },
        Keycode::Down => {
            scene.player.inventory.next();
        },
        Keycode::Tab => {
            let item = scene.player.inventory.extract_current();
            if item.is_some() {
                scene.craft_area.insert(item.unwrap());
            }
        },
        _ => ()
    }
}