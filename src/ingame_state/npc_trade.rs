use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::{draw_selection_with_parameters, ingame::SceneData, ingame::InputState};
use ingame_state::GameState;
use storage::SelectionStorage;
use objects::Item;
use misc::{Position, TextAlign};

#[derive(PartialEq, Clone)]
pub enum TradeArea {
    NpcInventory,
    NpcStaging,
    PlayerInventory,
    PlayerStaging
}

pub struct State {
    player_trade_area: SelectionStorage<Item>,
    npc_trade_area: SelectionStorage<Item>,
    active_trade_area: TradeArea,
    change_state: Option<InputState>
}

impl State {
    pub fn new() -> State {
    	State {
            player_trade_area: SelectionStorage::new(),
            npc_trade_area: SelectionStorage::new(),
            active_trade_area: TradeArea::PlayerInventory,
            change_state: None
        }
    }

    fn reset_trade_areas(&mut self, scene_data: &mut SceneData) {
        while let Some(item) = self.npc_trade_area.extract_current() {
            scene_data.current_npc().unwrap().inventory.insert(item);
        }

        while let Some(item) = self.player_trade_area.extract_current() {
            scene_data.player.inventory.insert(item);
        }
    }

    fn draw_trade_area(&self, selection: &SelectionStorage<Item>, ctx: &mut Context, area: TradeArea) -> GameResult<()> {
        let active = area == self.active_trade_area;
        match area {
            TradeArea::NpcInventory => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 180, y: 80 }, TextAlign::Left, active)?;
            },
            TradeArea::NpcStaging => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 220, y: 80 }, TextAlign::Right, active)?;
            },
            TradeArea::PlayerStaging => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 540, y: 80 }, TextAlign::Left, active)?;
            },
            TradeArea::PlayerInventory => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 580, y: 80 }, TextAlign::Right, active)?;
            },
        }

        Ok(())
    }
}

impl GameState for State {

    fn change_state(&mut self) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::State::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {

        match keycode {
            Keycode::Escape => {
                self.reset_trade_areas(scene_data);
                self.change_state = Some(InputState::World);
                if let Some(npc) = scene_data.current_npc() {
                    npc.direction = npc.look_at;
                }
            },
            Keycode::Return => {    
                while let Some(item) = self.npc_trade_area.extract_current() {
                    scene_data.player.inventory.insert(item);
                }

                while let Some(item) = self.player_trade_area.extract_current() {
                    scene_data.current_npc().unwrap().inventory.insert(item);
                }
            },
            Keycode::Tab => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        let item = scene_data.current_npc().unwrap().inventory.extract_current();
                        if item.is_some() {
                            self.npc_trade_area.insert(item.unwrap());
                        }
                    },
                    TradeArea::NpcStaging => {
                        let item = self.npc_trade_area.extract_current();
                        if item.is_some() {
                            scene_data.current_npc().unwrap().inventory.insert(item.unwrap());
                        }
                    },
                    TradeArea::PlayerStaging => {
                        let item = self.player_trade_area.extract_current();
                        if item.is_some() {
                            scene_data.player.inventory.insert(item.unwrap());
                        }
                    },
                    TradeArea::PlayerInventory => {
                        let item = scene_data.player.inventory.extract_current();
                        if item.is_some() {
                            self.player_trade_area.insert(item.unwrap());
                        }
                    },
                }
            },
            Keycode::Right => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        self.active_trade_area = TradeArea::NpcStaging;
                    },
                    TradeArea::NpcStaging => {
                        self.active_trade_area = TradeArea::PlayerStaging;
                    },
                    TradeArea::PlayerStaging => {
                        self.active_trade_area = TradeArea::PlayerInventory;
                    },
                    TradeArea::PlayerInventory => {
                        self.active_trade_area = TradeArea::NpcInventory;
                    },
                }
            },
            Keycode::Left => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        self.active_trade_area = TradeArea::PlayerInventory;
                    },
                    TradeArea::NpcStaging => {
                        self.active_trade_area = TradeArea::NpcInventory;
                    },
                    TradeArea::PlayerStaging => {
                        self.active_trade_area = TradeArea::NpcStaging;
                    },
                    TradeArea::PlayerInventory => {
                        self.active_trade_area = TradeArea::PlayerStaging;
                    },
                }
            },
            Keycode::Up => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        scene_data.current_npc().unwrap().inventory.prev();
                    },
                    TradeArea::NpcStaging => {
                        self.npc_trade_area.prev();
                    },
                    TradeArea::PlayerStaging => {
                        self.player_trade_area.prev();
                    },
                    TradeArea::PlayerInventory => {
                        scene_data.player.inventory.prev();
                    },
                }
            },
            Keycode::Down => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        scene_data.current_npc().unwrap().inventory.next();
                    },
                    TradeArea::NpcStaging => {
                        self.npc_trade_area.next();
                    },
                    TradeArea::PlayerStaging => {
                        self.player_trade_area.next();
                    },
                    TradeArea::PlayerInventory => {
                        scene_data.player.inventory.next();
                    },
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, scene_data: &mut SceneData, _camera: Position, ctx: &mut Context) -> GameResult<()> {
        let npc_inventory = scene_data.current_npc().unwrap().inventory.clone();
        self.draw_trade_area(&npc_inventory, ctx, TradeArea::NpcInventory)?;
        self.draw_trade_area(&self.npc_trade_area, ctx, TradeArea::NpcStaging)?;
        self.draw_trade_area(&self.player_trade_area, ctx, TradeArea::PlayerStaging)?;
        self.draw_trade_area(&scene_data.player.inventory, ctx, TradeArea::PlayerInventory)?;

        Ok(())
    }

    fn quit_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context) -> bool {
        self.reset_trade_areas(scene_data);

        false
    }
}