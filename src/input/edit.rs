use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, InputState};
use misc::*;
use objects::*;
use storage::*;

pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
        Keycode::Escape => {
            scene.input = InputState::World;
        },
        Keycode::Left => {
            scene.edit_cursor = &scene.edit_cursor + &Direction::Left.value();
            scene.edit_selection = scene.get_edit_selection();
        },
        Keycode::Right => {
            scene.edit_cursor = &scene.edit_cursor + &Direction::Right.value();
            scene.edit_selection = scene.get_edit_selection();
        },
        Keycode::Up => {
            scene.edit_cursor = &scene.edit_cursor + &Direction::Up.value();
            scene.edit_selection = scene.get_edit_selection();
        },
        Keycode::Down => {
            scene.edit_cursor = &scene.edit_cursor + &Direction::Down.value();
            scene.edit_selection = scene.get_edit_selection();
        },
        Keycode::Delete => {
            scene.walls.remove(scene.edit_cursor);
            scene.doors.remove(scene.edit_cursor);
            scene.terminals.remove(scene.edit_cursor);
            scene.circuitry.remove(scene.edit_cursor);
            scene.generators.remove(scene.edit_cursor);
            scene.update_power();
        },
        Keycode::W => {
            scene.walls.insert(scene.edit_cursor, Wall {});
        },
        Keycode::C => {
            scene.circuitry.insert(scene.edit_cursor, Circuitry {parts: SelectionStorage::new(), powered: false});
            scene.update_power();
        },
        Keycode::G => {
            scene.generators.insert(scene.edit_cursor, Generator {});
            scene.update_power();
        },
        Keycode::D => {
            scene.doors.insert(scene.edit_cursor, Door { status: DoorStatus::Closed});
        },
        Keycode::T => {
            scene.terminals.insert(scene.edit_cursor, Terminal { text: Box::new(String::new()), front: Direction::Down});
        },
        Keycode::Tab => {
            if let Some(ref mut door) = scene.doors.get_mut(scene.edit_cursor) {
                match door.status {
                    DoorStatus::Open => {
                        door.status = DoorStatus::Closed;
                    },
                    DoorStatus::Closed => {
                        door.status = DoorStatus::Open;
                    }
                }
            }
            if let Some(ref mut terminal) = scene.terminals.get_mut(scene.edit_cursor) {
                match terminal.front {
                    Direction::Up => {
                        terminal.front = Direction::Right;
                    },
                    Direction::Right => {
                        terminal.front = Direction::Down;
                    },
                    Direction::Down => {
                        terminal.front = Direction::Left;
                    },
                    Direction::Left => {
                        terminal.front = Direction::Up;
                    },
                }
            }
        },
        _ => ()
    }
}