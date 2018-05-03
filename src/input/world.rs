use std::time::Duration;

use ggez::Context;
use ggez::event::{Keycode, Mod};

use constants::MOVEMENT_SPEED;
use state::world::Scene;
use misc::*;

pub fn key_down_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
	if !repeat {
        scene.movement_timer = Duration::from_millis(MOVEMENT_SPEED);

        match keycode {
            Keycode::Left => {
                scene.player.movement(Direction::Left, Direction::Right);
            },
            Keycode::Right => {
                scene.player.movement(Direction::Right, Direction::Left);
            },
            Keycode::Up => {
                scene.player.movement(Direction::Up, Direction::Down);
            },
            Keycode::Down => {
                scene.player.movement(Direction::Down, Direction::Up);
            },
            _ => ()
        }
    } else {
        if let None = scene.player.movement.last() {
            match keycode {
                Keycode::Left => {
                    scene.player.movement(Direction::Left, Direction::Right);
                },
                Keycode::Right => {
                    scene.player.movement(Direction::Right, Direction::Left);
                },
                Keycode::Up => {
                    scene.player.movement(Direction::Up, Direction::Down);
                },
                Keycode::Down => {
                    scene.player.movement(Direction::Down, Direction::Up);
                },
                _ => ()
            }
        }
    }
}

pub fn key_up_event(scene: &mut Scene, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
	match keycode {
        Keycode::Left => {
            scene.player.remove_movement(Direction::Left);
        },
        Keycode::Right => {
            scene.player.remove_movement(Direction::Right);
        },
        Keycode::Up => {
            scene.player.remove_movement(Direction::Up);
        },
        Keycode::Down => {
            scene.player.remove_movement(Direction::Down);
        },
        Keycode::Return => {
            if scene.insight_view {
                scene.interact_with_circuitry();
            } else {
                scene.interact_with_door();
                scene.interact_with_terminal(ctx);
            }
        },
        Keycode::I => {
            scene.input = InputState::Inventory;
        },
        Keycode::Insert => {
            scene.input = InputState::Edit;
            scene.edit_selection = scene.get_edit_selection();
        },
        _ => ()
    }
}