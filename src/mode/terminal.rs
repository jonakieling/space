use ggez::Context;
use ggez::event::{Keycode, Mod};

use scene::Scene;

pub fn key_up_event(scene: &mut Scene, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Backspace => {
            scene.terminal_remove_character(ctx);
        },
        Keycode::Escape => {
            scene.clear_terminal(ctx);
        },
        _ => ()
    }
}