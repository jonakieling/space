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
        Keycode::Return => {
        	let mut dialog = scene.dialog.clone().unwrap();
        	if dialog.children.iter().len() > 0 {
        		scene.dialog = Some(dialog.children.current().unwrap().clone());	
        	} else {
	    		if let Some(npc) = scene.current_npc() {
	        	    npc.direction = npc.look_at;
	    		}
            	scene.input = InputState::World;
        	}
        },
        Keycode::Up => {
        	if let Some(ref mut dialog) = scene.dialog {
            	dialog.children.prev();
        	}
        },
        Keycode::Down => {
        	if let Some(ref mut dialog) = scene.dialog {
            	dialog.children.next();
        	}
        },
        _ => ()
    }
}