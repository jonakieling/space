use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app::{draw_selection_with_parameters, draw_input_state};
use game::{InputState, GameState};
use world::WorldData;
use objects::{Receipe, Item};
use misc::{Position, TextAlign};
use storage::SelectionStorage;
use feature::map::MapFeature;

#[derive(PartialEq, Eq)]
enum Mode {
    Inventory,
    Crafting
}

pub struct Handler {
    craft_area: SelectionStorage<Item>,
    change_state: Option<InputState>,
    mode: Mode
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            craft_area:  SelectionStorage::new(),
            change_state: None,
            mode: Mode::Inventory
        }
    }

    pub fn reset_craft_area(&mut self, data: &mut WorldData) {
        while let Some(item) = self.craft_area.extract_current() {
            data.level.player.inventory.insert(item);
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
            Some(InputState::Map(feature)) => {
                self.change_state = None;
                Some(Box::new(super::map::Handler::new(feature, data)))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.reset_craft_area(data);
                self.change_state = Some(InputState::World);
            },
            Keycode::I => {
                self.reset_craft_area(data);
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                match self.mode {
                    Mode::Crafting => self.craft_area.prev(),
                    Mode::Inventory => data.level.player.inventory.prev(),
                };
            },
            Keycode::Down => {
                match self.mode {
                    Mode::Crafting => self.craft_area.next(),
                    Mode::Inventory => data.level.player.inventory.next(),
                };
            },
            Keycode::Left => {
                match self.mode {
                    Mode::Crafting => self.mode = Mode::Inventory,
                    Mode::Inventory => self.mode = Mode::Crafting,
                }
            },
            Keycode::Right => {
                match self.mode {
                    Mode::Crafting => self.mode = Mode::Inventory,
                    Mode::Inventory => self.mode = Mode::Crafting,
                }
            },
            Keycode::Tab => {
                match self.mode {
                    Mode::Inventory => {
                        let item = data.level.player.inventory.extract_current();
                        if item.is_some() {
                            self.craft_area.insert(item.unwrap());
                        }
                    },
                    Mode::Crafting => {
                        let item = self.craft_area.extract_current();
                        if item.is_some() {
                            data.level.player.inventory.insert(item.unwrap());
                        }
                    }
                }
            },
            Keycode::Return => {
                match self.mode {
                    Mode::Inventory => {
                        match data.level.player.inventory.current() {
                            Some(Item::Navcomp) => {
                                self.change_state = Some(InputState::Map(MapFeature::View));
                            },
                            _ => (),
                        }
                    },
                    Mode::Crafting => {
                        let ref crafts = &self.craft_area.storage();
                        let products = Receipe::receipe_match(crafts, &data.receipes);
                        if let Some(receipe) = products.get(0) {
                            self.craft_area.clear();
                            data.level.player.inventory.insert(receipe.result.clone());
                        }
                    },
                }
            }
            _ => ()
        }
    }

    fn quit_event(&mut self, _ctx: &mut Context, data: &mut WorldData) -> bool {
        self.reset_craft_area(data);
        
        false
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        let cursor = self.mode == Mode::Inventory;
        draw_selection_with_parameters(&data.level.player.inventory, ctx, Position {x: 560, y: 80}, TextAlign::Right, cursor, true)?;
        draw_selection_with_parameters(&self.craft_area, ctx, Position {x: 520, y: 80}, TextAlign::Left, !cursor, false)?;

        draw_input_state("Inventory", ctx)?;

        Ok(())
    }
}