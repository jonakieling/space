use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::Scene;
use misc::*;

pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Escape => {
            scene.input = InputState::World;
        },
        Keycode::Up => {
            if let Some(current_circuitry) = scene.current_circuitry() {
                current_circuitry.parts.prev();
            }
        },
        Keycode::Down => {
            if let Some(current_circuitry) = scene.current_circuitry() {
                current_circuitry.parts.next();
            }
        },
        _ => ()
    }
}