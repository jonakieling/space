use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use state::{world::InputState, world::SceneData, draw_selection, draw_input_state};
use input::IngameState;

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

impl IngameState for State {

    fn change_state(&mut self) -> Option<Box<IngameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::State::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, scene: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                if let Some(current_circuitry) = scene.current_circuitry() {
                    current_circuitry.parts.prev();
                }
            },
            Keycode::Down => {
                if let Some(current_circuitry) = scene.current_circuitry() {
                    current_circuitry.parts.next();
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, scene: &mut SceneData, ctx: &mut Context) -> GameResult<()> {

        draw_input_state("Circuitry", ctx)?;
        draw_selection(&scene.current_circuitry().unwrap().parts, ctx, true)?;

        if !scene.insight_view {
            let front_index = scene.player.front_tile.to_int();
            if let Some(circuitry) = scene.current_circuitry() {
                circuitry.draw(front_index as i32, ctx)?;
            }
        }

        Ok(())
    }
            
}