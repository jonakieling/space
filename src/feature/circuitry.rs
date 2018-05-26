use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::{draw_selection_with_parameters, draw_tile};
use game::{InputState, GameState};
use misc::{Position, TextAlign};
use objects::Item;

#[derive(PartialEq, Eq)]
enum Mode {
    Inventory,
    Circuitry
}

pub struct Handler {
    change_state: Option<InputState>,
    mode: Mode
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            change_state: None,
            mode: Mode::Circuitry
        }
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, _data: &mut WorldData) -> Option<Box<GameState>> {
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
            Keycode::Up => {
                match self.mode {
                    Mode::Circuitry => data.level.current_circuitry().unwrap().parts.prev(),
                    Mode::Inventory => data.level.player.inventory.prev(),
                };
            },
            Keycode::Down => {
                match self.mode {
                    Mode::Circuitry => data.level.current_circuitry().unwrap().parts.next(),
                    Mode::Inventory => data.level.player.inventory.next(),
                };
            },
            Keycode::Left => {
                match self.mode {
                    Mode::Circuitry => self.mode = Mode::Inventory,
                    Mode::Inventory => self.mode = Mode::Circuitry,
                }
            },
            Keycode::Right => {
                match self.mode {
                    Mode::Circuitry => self.mode = Mode::Inventory,
                    Mode::Inventory => self.mode = Mode::Circuitry,
                }
            },
            Keycode::Tab => {
                match self.mode {
                    Mode::Inventory => {
                        let mut current = None;
                        if let Some(item) = data.level.player.inventory.current() {
                            current = Some(item.clone());
                        }
                        if let Some(item) = current {
                            if item == Item::PowerConductor {
                                let item = data.level.player.inventory.extract_current();
                                if item.is_some() {
                                    &data.level.current_circuitry().unwrap().parts.insert(item.unwrap());
                                    data.level.update_power();
                                }
                            }
                        }
                    },
                    Mode::Circuitry => {
                        let item = data.level.current_circuitry().unwrap().parts.extract_current();
                        if let Some(item) = item {    
                            data.level.player.inventory.insert(item.clone());
                            data.level.update_power();
                        }
                    }
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        let cursor = self.mode == Mode::Inventory;
        draw_selection_with_parameters(&data.level.player.inventory, ctx, Position {x: 580, y: 80}, TextAlign::Right, cursor, true)?;
        draw_selection_with_parameters(&data.level.current_circuitry().unwrap().parts, ctx, Position {x: 540, y: 80}, TextAlign::Left, !cursor, true)?;

        if !data.insight_view {
            let front_index = data.level.player.front_tile.to_int();
            let camera = data.camera;
            if let Some(circuitry) = data.level.current_circuitry() {
                draw_tile(ctx, circuitry.tile(), front_index, camera, None)?;
            }
        }

        Ok(())
    }
            
}