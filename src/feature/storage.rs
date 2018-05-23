use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::{draw_input_state, draw_selection_with_parameters};
use game::{InputState, GameState};
use misc::{TextAlign, Position};
use objects::Item;
use storage::SelectionStorage;

#[derive(PartialEq, Clone)]
pub enum StorageArea {
    Inventory,
    Storage
}

pub struct Handler {
    change_state: Option<InputState>,
    active_storage_area: StorageArea
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            change_state: None,
            active_storage_area: StorageArea::Inventory
        }
    }

    fn draw_storage_area(&self, selection: &SelectionStorage<Item>, ctx: &mut Context, area: StorageArea) -> GameResult<()> {
        let active = area == self.active_storage_area;
        match area {
            StorageArea::Storage => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 540, y: 80 }, TextAlign::Left, active, true)?;
            },
            StorageArea::Inventory => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 580, y: 80 }, TextAlign::Right, active, true)?;
            },
        }

        Ok(())
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, _scene_data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::Handler::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Tab => {
                match self.active_storage_area {
                    StorageArea::Storage => {
                        let item = data.level.current_storage().unwrap().content.extract_current();
                        if item.is_some() {
                            data.level.player.inventory.insert(item.unwrap());
                        }
                    },
                    StorageArea::Inventory => {
                        let item = data.level.player.inventory.extract_current();
                        if item.is_some() {
                            data.level.current_storage().unwrap().content.insert(item.unwrap());
                        }
                    },
                }
            },
            Keycode::Right => {
                match self.active_storage_area {
                    StorageArea::Inventory => {
                        self.active_storage_area = StorageArea::Storage;
                    },
                    StorageArea::Storage => {
                        self.active_storage_area = StorageArea::Inventory;
                    },
                }
            },
            Keycode::Left => {
                match self.active_storage_area {
                    StorageArea::Inventory => {
                        self.active_storage_area = StorageArea::Storage;
                    },
                    StorageArea::Storage => {
                        self.active_storage_area = StorageArea::Inventory;
                    },
                }
            },
            Keycode::Up => {
                match self.active_storage_area {
                    StorageArea::Inventory => {
                        data.level.player.inventory.prev();
                    },
                    StorageArea::Storage => {
                        if let Some(current_storage) = data.level.current_storage() {
                            current_storage.content.prev();
                        }
                    },
                }
            },
            Keycode::Down => {
                match self.active_storage_area {
                    StorageArea::Inventory => {
                        data.level.player.inventory.next();
                    },
                    StorageArea::Storage => {
                        if let Some(current_storage) = data.level.current_storage() {
                            current_storage.content.next();
                        }
                    },
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {

        draw_input_state("Storage", ctx)?;

        self.draw_storage_area(&data.level.current_storage().unwrap().content, ctx, StorageArea::Storage)?;
        self.draw_storage_area(&data.level.player.inventory, ctx, StorageArea::Inventory)?;

        Ok(())
    }
            
}