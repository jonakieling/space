use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::{draw_selection_with_parameters, draw_input_state};
use app_state::ingame::{SceneData, InputState};
use objects::{Receipe, Item};
use ingame_state::GameState;
use misc::{Position, TextAlign};
use storage::SelectionStorage;

#[derive(PartialEq, Eq)]
enum Mode {
    Inventory,
    Crafting
}

pub struct State {
    craft_area: SelectionStorage<Item>,
    change_state: Option<InputState>,
    mode: Mode
}

impl State {
    pub fn new() -> State {
    	State {
            craft_area:  SelectionStorage::new(),
            change_state: None,
            mode: Mode::Inventory
        }
    }

    pub fn reset_craft_area(&mut self, scene_data: &mut SceneData) {
        while let Some(item) = self.craft_area.extract_current() {
            scene_data.player.inventory.insert(item);
        }
    }
}

impl GameState for State {

    fn change_state(&mut self, _scene_data: &mut SceneData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::State::new()))
            },
            Some(InputState::Log) => {
                self.change_state = None;
                Some(Box::new(super::log::State::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.reset_craft_area(scene_data);
                self.change_state = Some(InputState::World);
            },
            Keycode::I => {
                self.reset_craft_area(scene_data);
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                match self.mode {
                    Mode::Crafting => self.craft_area.prev(),
                    Mode::Inventory => scene_data.player.inventory.prev(),
                };
            },
            Keycode::Down => {
                match self.mode {
                    Mode::Crafting => self.craft_area.next(),
                    Mode::Inventory => scene_data.player.inventory.next(),
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
                        let item = scene_data.player.inventory.extract_current();
                        if item.is_some() {
                            self.craft_area.insert(item.unwrap());
                        }
                    },
                    Mode::Crafting => {
                        let item = self.craft_area.extract_current();
                        if item.is_some() {
                            scene_data.player.inventory.insert(item.unwrap());
                        }
                    }
                }
            },
            Keycode::Return => {
                match self.mode {
                    Mode::Inventory => {
                        match scene_data.player.inventory.current() {
                            Some(Item::Log) => {
                                self.change_state = Some(InputState::Log);
                            },
                            _ => (),
                        }
                    },
                    Mode::Crafting => {
                        let ref crafts = &self.craft_area.storage();
                        let products = Receipe::receipe_match(crafts, &scene_data.receipes);
                        if let Some(receipe) = products.get(0) {
                            self.craft_area.clear();
                            scene_data.player.inventory.insert(receipe.result.clone());
                        }
                    },
                }
            }
            _ => ()
        }
    }

    fn quit_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context) -> bool {
        self.reset_craft_area(scene_data);
        
        false
    }

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {
        let cursor = self.mode == Mode::Inventory;
        draw_selection_with_parameters(&scene_data.player.inventory, ctx, Position {x: 770, y: 20}, TextAlign::Left, cursor, true)?;
        draw_selection_with_parameters(&self.craft_area, ctx, Position {x: 580, y: 20}, TextAlign::Left, !cursor, false)?;

        draw_input_state("Inventory", ctx)?;

        Ok(())
    }
}