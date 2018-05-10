use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, InputState, TradeArea};


pub fn key_up_event(scene: &mut Scene, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {

    match keycode {
        Keycode::Escape => {
            scene.reset_trade_areas();
            scene.input = InputState::World;
            if let Some(npc) = scene.current_npc() {
                npc.direction = npc.look_at;
            }
        },
        Keycode::Return => {    
            while let Some(item) = scene.npc_trade_area.extract_current() {
                scene.player.inventory.insert(item);
            }

            while let Some(item) = scene.player_trade_area.extract_current() {
                scene.current_npc().unwrap().inventory.insert(item);
            }
        },
        Keycode::Tab => {
            match scene.active_trade_area {
                TradeArea::NpcInventory => {
                    let item = scene.current_npc().unwrap().inventory.extract_current();
                    if item.is_some() {
                        scene.npc_trade_area.insert(item.unwrap());
                    }
                },
                TradeArea::NpcStaging => {
                    let item = scene.npc_trade_area.extract_current();
                    if item.is_some() {
                        scene.current_npc().unwrap().inventory.insert(item.unwrap());
                    }
                },
                TradeArea::PlayerStaging => {
                    let item = scene.player_trade_area.extract_current();
                    if item.is_some() {
                        scene.player.inventory.insert(item.unwrap());
                    }
                },
                TradeArea::PlayerInventory => {
                    let item = scene.player.inventory.extract_current();
                    if item.is_some() {
                        scene.player_trade_area.insert(item.unwrap());
                    }
                },
            }
        },
        Keycode::Right => {
            match scene.active_trade_area {
                TradeArea::NpcInventory => {
                    scene.active_trade_area = TradeArea::NpcStaging;
                },
                TradeArea::NpcStaging => {
                    scene.active_trade_area = TradeArea::PlayerStaging;
                },
                TradeArea::PlayerStaging => {
                    scene.active_trade_area = TradeArea::PlayerInventory;
                },
                TradeArea::PlayerInventory => {
                    scene.active_trade_area = TradeArea::NpcInventory;
                },
            }
        },
        Keycode::Left => {
            match scene.active_trade_area {
                TradeArea::NpcInventory => {
                    scene.active_trade_area = TradeArea::PlayerInventory;
                },
                TradeArea::NpcStaging => {
                    scene.active_trade_area = TradeArea::NpcInventory;
                },
                TradeArea::PlayerStaging => {
                    scene.active_trade_area = TradeArea::NpcStaging;
                },
                TradeArea::PlayerInventory => {
                    scene.active_trade_area = TradeArea::PlayerStaging;
                },
            }
        },
        Keycode::Up => {
            match scene.active_trade_area {
                TradeArea::NpcInventory => {
                    scene.current_npc().unwrap().inventory.prev();
                },
                TradeArea::NpcStaging => {
                    scene.npc_trade_area.prev();
                },
                TradeArea::PlayerStaging => {
                    scene.player_trade_area.prev();
                },
                TradeArea::PlayerInventory => {
                    scene.player.inventory.prev();
                },
            }
        },
        Keycode::Down => {
            match scene.active_trade_area {
                TradeArea::NpcInventory => {
                    scene.current_npc().unwrap().inventory.next();
                },
                TradeArea::NpcStaging => {
                    scene.npc_trade_area.next();
                },
                TradeArea::PlayerStaging => {
                    scene.player_trade_area.next();
                },
                TradeArea::PlayerInventory => {
                    scene.player.inventory.next();
                },
            }
        },
        _ => ()
    }
}