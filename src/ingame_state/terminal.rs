use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::{draw_input_state, draw_dialog};
use app_state::ingame::{SceneData, InputState};
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
            Keycode::Return => {
                if scene_data.dialog.children.iter().len() > 0 {
                    scene_data.dialog = scene_data.dialog.children.current().unwrap().clone();	
                } else {
                    self.change_state = Some(InputState::World);
                }
            },
            Keycode::Up => {
                scene_data.dialog.children.prev();
            },
            Keycode::Down => {
                scene_data.dialog.children.next();
            },
            _ => ()
        }
    }

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {
        draw_input_state("Terminal", ctx)?;

        draw_dialog(&scene_data.dialog, ctx)
    }
}