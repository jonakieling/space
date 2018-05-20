use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::{ingame::InputState, ingame::SceneData, draw_selection};
use ingame_state::GameState;
use app_state::ingame::draw_tile;

pub struct State {
    change_state: Option<InputState>
}

impl State {
    pub fn new() -> State {
    	State {
            change_state: None
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
            _ => None,
        }
    }

    fn key_up_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                if let Some(current_circuitry) = scene_data.current_circuitry() {
                    current_circuitry.parts.prev();
                }
            },
            Keycode::Down => {
                if let Some(current_circuitry) = scene_data.current_circuitry() {
                    current_circuitry.parts.next();
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {
        draw_selection(&scene_data.current_circuitry().unwrap().parts, ctx, true, false)?;

        if !scene_data.insight_view {
            let front_index = scene_data.player.front_tile.to_int();
            let camera = scene_data.camera;
            if let Some(circuitry) = scene_data.current_circuitry() {
                draw_tile(ctx, circuitry.tile(), front_index, camera, None)?;
            }
        }

        Ok(())
    }
            
}