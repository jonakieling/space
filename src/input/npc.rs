use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, InputState};
use dialog::DialogAction;


pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {

    match keycode {
        Keycode::Escape => {
    		if let Some(npc) = scene.current_npc() {
        	    npc.direction = npc.look_at;
    		}
            scene.input = InputState::World;
        },
        Keycode::Return => {
        	if scene.dialog.children.iter().len() > 0 {
                if let Some(dialog_item) = scene.dialog.children.current() {
                    if let Some(ref action) = dialog_item.value.action {
                        match *action {
                            DialogAction::Trade => {
                                scene.input = InputState::NpcTrade;
                            }
                        }
                    }
                }
        		scene.dialog = scene.dialog.children.current().unwrap().clone();	
        	} else {
	    		if let Some(npc) = scene.current_npc() {
	        	    npc.direction = npc.look_at;
	    		}
            	scene.input = InputState::World;
        	}
        },
        Keycode::Up => {
            scene.dialog.children.prev();
        },
        Keycode::Down => {
            scene.dialog.children.next();
        },
        _ => ()
    }
}