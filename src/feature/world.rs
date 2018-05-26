use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use game::{InputState, GameState};
use feature::map::MapFeature;
use misc::*;
use app::draw_dialog;
use objects::*;
use savegame::*;
use storage::{Node, SelectionStorage};
use dialog::DialogItem;

pub struct Handler {
    change_state: Option<InputState>,
    dialog: Option<Node<DialogItem>>
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            change_state: None,
            dialog: None
        }
    }

    fn interact_with_door(&mut self, data: &mut WorldData, powered: bool) {
        let mut location = None;

        if let Some(door) = data.level.doors.get_mut(data.level.player.front_tile) {
            if powered {
                match &door.variant {
                    DoorType::Passage => {
                        match door.status {
                            DoorStatus::Closed => {
                                door.status = DoorStatus::Open;
                            },
                            DoorStatus::Open => {
                                door.status = DoorStatus::Closed;
                            },
                        }
                    },
                    DoorType::Exit(new_location) => {
                        location = Some(new_location.clone());
                    }
                }
            } else {
                self.dialog = Some(Node {
                    value: DialogItem {
                        text: "".to_string(),
                        response: "Needs to be powered".to_string(),
                        action: None
                    },
                    children: SelectionStorage::new()
                });
            }
        }

        if let Some(ref location) = location {
            save_location(data);
            load_location(data, location);
        }
    }

    fn interact_with_npc(&mut self, data: &mut WorldData) {
        if let Some(npc) = data.level.npc.get_mut(data.level.player.front_tile) {
            match data.level.player.direction {
                Direction::Down => npc.direction = Direction::Up,
                Direction::Left => npc.direction = Direction::Right,
                Direction::Up => npc.direction = Direction::Down,
                Direction::Right => npc.direction = Direction::Left,
            }
            self.change_state = Some(InputState::Npc);
        }
    }

    fn interact_with_circuitry(&mut self, data: &mut WorldData) {
        if let Some(_) = data.level.circuitry.get_mut(data.level.player.front_tile) {
            self.change_state = Some(InputState::Circuitry);
        }
    }

    fn interact_with_storage(&mut self, data: &mut WorldData) {
        if let Some(_) = data.level.storages.get_mut(data.level.player.front_tile) {
            self.change_state = Some(InputState::Storage);
        }
    }

    fn interact_with_terminal(&mut self, data: &mut WorldData, powered: bool) {
        if let Some(ref terminal) = data.level.terminals.get_mut(data.level.player.front_tile) {
            if powered {
                let terminal_front_tile = &terminal.front.value() + &data.level.player.front_tile;
                if terminal_front_tile == data.level.player.position {
                    self.change_state = Some(InputState::Terminal);
                }
            } else {
                self.dialog = Some(Node {
                    value: DialogItem {
                        text: "".to_string(),
                        response: "Needs to be powered".to_string(),
                        action: None
                    },
                    children: SelectionStorage::new()
                });
            }
        }
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::Handler::new()))
            },
            Some(InputState::Edit) => {
                self.change_state = None;
                Some(Box::new(super::edit::Handler::new(data.level.player.position)))
            },
            Some(InputState::Menu) => {
                self.change_state = None;
                Some(Box::new(super::menu::Handler::new()))
            },
            Some(InputState::Npc) => {
                self.change_state = None;
                Some(Box::new(super::npc::Handler::new(data)))
            },
            Some(InputState::Terminal) => {
                self.change_state = None;
                Some(Box::new(super::terminal::Handler::new(data)))
            },
            Some(InputState::Inventory) => {
                self.change_state = None;
                Some(Box::new(super::inventory::Handler::new()))
            },
            Some(InputState::Circuitry) => {
                self.change_state = None;
                Some(Box::new(super::circuitry::Handler::new()))
            },
            Some(InputState::Storage) => {
                self.change_state = None;
                Some(Box::new(super::storage::Handler::new()))
            },
            Some(InputState::Map(feature)) => {
                self.change_state = None;
                Some(Box::new(super::map::Handler::new(feature, data)))
            },
            _ => None,
        }
    }
    
    fn key_down_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            match keycode {
                Keycode::Left => {
                    data.level.player.movement(Direction::Left, Direction::Right);
                },
                Keycode::Right => {
                    data.level.player.movement(Direction::Right, Direction::Left);
                },
                Keycode::Up => {
                    data.level.player.movement(Direction::Up, Direction::Down);
                },
                Keycode::Down => {
                    data.level.player.movement(Direction::Down, Direction::Up);
                },
                _ => ()
            }
        } else {
            if let None = data.level.player.movement.last() {
                match keycode {
                    Keycode::Left => {
                        data.level.player.movement(Direction::Left, Direction::Right);
                    },
                    Keycode::Right => {
                        data.level.player.movement(Direction::Right, Direction::Left);
                    },
                    Keycode::Up => {
                        data.level.player.movement(Direction::Up, Direction::Down);
                    },
                    Keycode::Down => {
                        data.level.player.movement(Direction::Down, Direction::Up);
                    },
                    _ => ()
                }
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
                data.level.player.remove_movement(Direction::Left);
            },
            Keycode::Right => {
                data.level.player.remove_movement(Direction::Right);
            },
            Keycode::Up => {
                data.level.player.remove_movement(Direction::Up);
            },
            Keycode::Down => {
                data.level.player.remove_movement(Direction::Down);
            },
            Keycode::Return => {
                match self.dialog {
                    Some(_) => self.dialog = None,
                    None => {
                        if data.insight_view {
                            self.interact_with_circuitry(data);
                        } else {
                            let mut powered = false;
                            if let Some(circuitry) = data.level.current_circuitry() {
                                if circuitry.powered() {
                                    powered = true;
                                }
                            }
                            
                            self.interact_with_terminal(data, powered);
                            self.interact_with_door(data, powered);
                            self.interact_with_npc(data);
                            self.interact_with_storage(data);
                        }
                    },
                }
            },
            Keycode::I => {
                self.change_state = Some(InputState::Inventory);
            },
            Keycode::M => {
                if data.level.player.has(Item::Navcomp) {
                    self.change_state = Some(InputState::Map(MapFeature::View));
                }
            },
            Keycode::Escape => {
                match self.dialog {
                    Some(_) => self.dialog = None,
                    None => self.change_state = Some(InputState::Menu),
                }
            },
            Keycode::Insert => {
                self.change_state = Some(InputState::Edit);
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, _data: &mut WorldData) -> GameResult<()> {
        if let Some(ref dialog) = self.dialog {
            draw_dialog(dialog, ctx)?;
        }

        Ok(())
    }
}