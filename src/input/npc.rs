use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::Scene;
use misc::*;


pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Escape => {
        	if let Some(npc) = scene.current_npc() {
        	    npc.direction = npc.look_at;
        	}
            scene.input = InputState::World;
        },
        _ => ()
    }
}