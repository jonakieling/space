use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::{ingame::SceneData, draw_input_state, ingame::InputState};
use misc::*;
use ingame_state::GameState;

pub struct State {
    change_state: Option<InputState>
}

impl State {
    pub fn new() -> State {
    	State {
            change_state: None
        }
    }

    fn interact_with_npc(&mut self, scene_data: &mut SceneData) {
        if let Some(npc) = scene_data.npc.get_mut(scene_data.player.front_tile) {
            match scene_data.player.direction {
                Direction::Down => npc.direction = Direction::Up,
                Direction::Left => npc.direction = Direction::Right,
                Direction::Up => npc.direction = Direction::Down,
                Direction::Right => npc.direction = Direction::Left,
            }
            scene_data.dialog = npc.dialog.root.clone();
            self.change_state = Some(InputState::Npc);
        }
    }

    fn interact_with_circuitry(&mut self, scene_data: &mut SceneData) {
        if let Some(_) = scene_data.circuitry.get_mut(scene_data.player.front_tile) {
            self.change_state = Some(InputState::Circuitry);
        }
    }

    fn interact_with_storage(&mut self, scene_data: &mut SceneData) {
        if let Some(_) = scene_data.storages.get_mut(scene_data.player.front_tile) {
            self.change_state = Some(InputState::Storage);
        }
    }

    fn interact_with_terminal(&mut self, scene_data: &mut SceneData) {
        if let Some(ref terminal) = scene_data.terminals.get_mut(scene_data.player.front_tile) {
            let terminal_front_tile = &terminal.front.value() + &scene_data.player.front_tile;
            if terminal_front_tile == scene_data.player.position {
                self.change_state = Some(InputState::Terminal);
            }
        }
    }
}

impl GameState for State {

    fn change_state(&mut self) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::State::new()))
            },
            Some(InputState::Edit) => {
                self.change_state = None;
                Some(Box::new(super::edit::State::new()))
            },
            Some(InputState::Menu) => {
                self.change_state = None;
                Some(Box::new(super::menu::State::new()))
            },
            Some(InputState::Npc) => {
                self.change_state = None;
                Some(Box::new(super::npc::State::new()))
            },
            Some(InputState::Terminal) => {
                self.change_state = None;
                Some(Box::new(super::terminal::State::new()))
            },
            Some(InputState::Inventory) => {
                self.change_state = None;
                Some(Box::new(super::inventory::State::new()))
            },
            Some(InputState::Circuitry) => {
                self.change_state = None;
                Some(Box::new(super::circuitry::State::new()))
            },
            Some(InputState::Storage) => {
                self.change_state = None;
                Some(Box::new(super::storage::State::new()))
            },
            _ => None,
        }
    }
    
    fn key_down_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            match keycode {
                Keycode::Left => {
                    scene_data.player.movement(Direction::Left, Direction::Right);
                },
                Keycode::Right => {
                    scene_data.player.movement(Direction::Right, Direction::Left);
                },
                Keycode::Up => {
                    scene_data.player.movement(Direction::Up, Direction::Down);
                },
                Keycode::Down => {
                    scene_data.player.movement(Direction::Down, Direction::Up);
                },
                _ => ()
            }
        } else {
            if let None = scene_data.player.movement.last() {
                match keycode {
                    Keycode::Left => {
                        scene_data.player.movement(Direction::Left, Direction::Right);
                    },
                    Keycode::Right => {
                        scene_data.player.movement(Direction::Right, Direction::Left);
                    },
                    Keycode::Up => {
                        scene_data.player.movement(Direction::Up, Direction::Down);
                    },
                    Keycode::Down => {
                        scene_data.player.movement(Direction::Down, Direction::Up);
                    },
                    _ => ()
                }
            }
        }
    }

    fn key_up_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
                scene_data.player.remove_movement(Direction::Left);
            },
            Keycode::Right => {
                scene_data.player.remove_movement(Direction::Right);
            },
            Keycode::Up => {
                scene_data.player.remove_movement(Direction::Up);
            },
            Keycode::Down => {
                scene_data.player.remove_movement(Direction::Down);
            },
            Keycode::Return => {
                if scene_data.insight_view {
                    self.interact_with_circuitry(scene_data);
                } else {
                    self.interact_with_storage(scene_data);
                    self.interact_with_terminal(scene_data);
                    self.interact_with_npc(scene_data);
                    scene_data.interact_with_door();
                }
            },
            Keycode::I => {
                self.change_state = Some(InputState::Inventory);
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

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {

        if scene_data.insight_view {
            draw_input_state("World insight", ctx)?;
        } else {
            draw_input_state("World", ctx)?;
        }

        Ok(())
    }
}