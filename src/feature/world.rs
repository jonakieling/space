use ggez::Context;
use ggez::event::{Keycode, Mod};

use world::WorldData;
use game::{InputState, GameState};
use feature::map::MapFeature;
use misc::*;
use objects::*;
use savegame::static_levels::*;

pub struct Handler {
    change_state: Option<InputState>
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            change_state: None
        }
    }

    fn interact_with_door(&mut self, data: &mut WorldData) {
        let mut load_static_ship = false;
        let mut load_static_station = false;
        if let Some(door) = data.level.doors.get_mut(data.level.player.front_tile) {
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
	            DoorType::Exit(Location::Ship(ref ship_id)) => {
                    if ship_id == "Tech 2.1" {
                        load_static_ship = true;
                    }
                },
	            DoorType::Exit(Location::Station(ref station_id)) => {
                    if station_id == "Mun" {
                        load_static_station = true;
                    }
                },
	            DoorType::Exit(Location::Space) => { },
            }
        }

        if load_static_ship {
            static_ship_tech(data);
        } else if load_static_station {
            static_station_outpost(data);
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
            data.dialog = npc.dialog.clone();
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

    fn interact_with_terminal(&mut self, data: &mut WorldData) {
        if let Some(ref terminal) = data.level.terminals.get_mut(data.level.player.front_tile) {
            let terminal_front_tile = &terminal.front.value() + &data.level.player.front_tile;
            if terminal_front_tile == data.level.player.position {
                self.change_state = Some(InputState::Terminal);
                data.dialog = terminal.dialog.clone();
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
                Some(Box::new(super::npc::Handler::new()))
            },
            Some(InputState::Terminal) => {
                self.change_state = None;
                Some(Box::new(super::terminal::Handler::new()))
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
                if data.insight_view {
                    self.interact_with_circuitry(data);
                } else {
                    self.interact_with_storage(data);
                    self.interact_with_terminal(data);
                    self.interact_with_npc(data);
                    self.interact_with_door(data);
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
                self.change_state = Some(InputState::Menu);
            },
            Keycode::Insert => {
                self.change_state = Some(InputState::Edit);
            },
            _ => ()
        }
    }
}