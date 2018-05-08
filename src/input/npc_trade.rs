use ggez::Context;
use ggez::event::{Keycode, Mod};

use state::world::{Scene, InputState};
use state::TradeArea;


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
            match scene.active_trade_area {
                TradeArea::LeftSource => {
                    let item = scene.current_npc().unwrap().inventory.extract_current();
                    if item.is_some() {
                        scene.npc_trade_area.insert(item.unwrap());
                    }
                },
                TradeArea::LeftTarget => {
                    let item = scene.npc_trade_area.extract_current();
                    if item.is_some() {
                        scene.current_npc().unwrap().inventory.insert(item.unwrap());
                    }
                },
                TradeArea::RightTarget => {
                    let item = scene.player_trade_area.extract_current();
                    if item.is_some() {
                        scene.player.inventory.insert(item.unwrap());
                    }
                },
                TradeArea::RightSource => {
                    let item = scene.player.inventory.extract_current();
                    if item.is_some() {
                        scene.player_trade_area.insert(item.unwrap());
                    }
                },
            }
        },
        Keycode::Right => {
            match scene.active_trade_area {
                TradeArea::LeftSource => {
                    scene.active_trade_area = TradeArea::LeftTarget;
                },
                TradeArea::LeftTarget => {
                    scene.active_trade_area = TradeArea::RightTarget;
                },
                TradeArea::RightTarget => {
                    scene.active_trade_area = TradeArea::RightSource;
                },
                TradeArea::RightSource => {
                    scene.active_trade_area = TradeArea::LeftSource;
                },
            }
        },
        Keycode::Left => {
            match scene.active_trade_area {
                TradeArea::LeftSource => {
                    scene.active_trade_area = TradeArea::RightSource;
                },
                TradeArea::LeftTarget => {
                    scene.active_trade_area = TradeArea::LeftSource;
                },
                TradeArea::RightTarget => {
                    scene.active_trade_area = TradeArea::LeftTarget;
                },
                TradeArea::RightSource => {
                    scene.active_trade_area = TradeArea::RightTarget;
                },
            }
        },
        Keycode::Up => {
            match scene.active_trade_area {
                TradeArea::LeftSource => {
                    scene.current_npc().unwrap().inventory.prev();
                },
                TradeArea::LeftTarget => {
                    scene.npc_trade_area.prev();
                },
                TradeArea::RightTarget => {
                    scene.player_trade_area.prev();
                },
                TradeArea::RightSource => {
                    scene.player.inventory.prev();
                },
            }
        },
        Keycode::Down => {
            match scene.active_trade_area {
                TradeArea::LeftSource => {
                    scene.current_npc().unwrap().inventory.next();
                },
                TradeArea::LeftTarget => {
                    scene.npc_trade_area.next();
                },
                TradeArea::RightTarget => {
                    scene.player_trade_area.next();
                },
                TradeArea::RightSource => {
                    scene.player.inventory.next();
                },
            }
        },
        _ => ()
    }
}