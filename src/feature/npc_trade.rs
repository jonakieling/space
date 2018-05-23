use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::{draw_input_state, draw_selection_with_parameters};
use game::{InputState, GameState};
use storage::SelectionStorage;
use objects::Item;
use misc::{TextAlign, Position};

#[derive(PartialEq, Clone)]
pub enum TradeArea {
    NpcInventory,
    NpcStaging,
    PlayerInventory,
    PlayerStaging
}

pub struct Handler {
    player_trade_area: SelectionStorage<Item>,
    npc_trade_area: SelectionStorage<Item>,
    active_trade_area: TradeArea,
    change_state: Option<InputState>
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            player_trade_area: SelectionStorage::new(),
            npc_trade_area: SelectionStorage::new(),
            active_trade_area: TradeArea::PlayerInventory,
            change_state: None
        }
    }

    fn reset_trade_areas(&mut self, data: &mut WorldData) {
        while let Some(item) = self.npc_trade_area.extract_current() {
            data.level.current_npc().unwrap().inventory.insert(item);
        }

        while let Some(item) = self.player_trade_area.extract_current() {
            data.level.player.inventory.insert(item);
        }
    }

    fn draw_trade_area(&self, selection: &SelectionStorage<Item>, ctx: &mut Context, area: TradeArea) -> GameResult<()> {
        let active = area == self.active_trade_area;
        match area {
            TradeArea::NpcInventory => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 180, y: 80 }, TextAlign::Left, active, true)?;
            },
            TradeArea::NpcStaging => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 220, y: 80 }, TextAlign::Right, active, true)?;
            },
            TradeArea::PlayerStaging => {
                draw_selection_with_parameters(&selection, ctx, Position { x: 540, y: 80 }, TextAlign::Left, active, true)?;
            },
            TradeArea::PlayerInventory => {
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
                self.reset_trade_areas(data);
                self.change_state = Some(InputState::World);
                if let Some(npc) = data.level.current_npc() {
                    npc.direction = npc.look_at;
                }
            },
            Keycode::Return => {    
                while let Some(item) = self.npc_trade_area.extract_current() {
                    data.level.player.inventory.insert(item);
                }

                while let Some(item) = self.player_trade_area.extract_current() {
                    data.level.current_npc().unwrap().inventory.insert(item);
                }
            },
            Keycode::Tab => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        let item = data.level.current_npc().unwrap().inventory.extract_current();
                        if item.is_some() {
                            self.npc_trade_area.insert(item.unwrap());
                        }
                    },
                    TradeArea::NpcStaging => {
                        let item = self.npc_trade_area.extract_current();
                        if item.is_some() {
                            data.level.current_npc().unwrap().inventory.insert(item.unwrap());
                        }
                    },
                    TradeArea::PlayerStaging => {
                        let item = self.player_trade_area.extract_current();
                        if item.is_some() {
                            data.level.player.inventory.insert(item.unwrap());
                        }
                    },
                    TradeArea::PlayerInventory => {
                        let item = data.level.player.inventory.extract_current();
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
                        data.level.current_npc().unwrap().inventory.prev();
                    },
                    TradeArea::NpcStaging => {
                        self.npc_trade_area.prev();
                    },
                    TradeArea::PlayerStaging => {
                        self.player_trade_area.prev();
                    },
                    TradeArea::PlayerInventory => {
                        data.level.player.inventory.prev();
                    },
                }
            },
            Keycode::Down => {
                match self.active_trade_area {
                    TradeArea::NpcInventory => {
                        data.level.current_npc().unwrap().inventory.next();
                    },
                    TradeArea::NpcStaging => {
                        self.npc_trade_area.next();
                    },
                    TradeArea::PlayerStaging => {
                        self.player_trade_area.next();
                    },
                    TradeArea::PlayerInventory => {
                        data.level.player.inventory.next();
                    },
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        draw_input_state("Trade", ctx)?;
        let npc_inventory = data.level.current_npc().unwrap().inventory.clone();
        self.draw_trade_area(&npc_inventory, ctx, TradeArea::NpcInventory)?;
        self.draw_trade_area(&self.npc_trade_area, ctx, TradeArea::NpcStaging)?;
        self.draw_trade_area(&self.player_trade_area, ctx, TradeArea::PlayerStaging)?;
        self.draw_trade_area(&data.level.player.inventory, ctx, TradeArea::PlayerInventory)?;

        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context, data: &mut WorldData) -> bool {
        self.reset_trade_areas(data);

        false
    }
}