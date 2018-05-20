use ggez::{Context, GameResult, graphics};
use ggez::event::{Keycode, Mod};

use app_state::{draw_text, draw_input_state};
use app_state::ingame::{SceneData, InputState};
use ingame_state::GameState;

const TERMINAL_LIMIT: usize = 20;

pub struct State {
    terminal_text: Option<String>,
    change_state: Option<InputState>
}

impl State {
    pub fn new() -> State {
    	State {
            terminal_text: None,
            change_state: None
        }
    }

    fn terminal_add_character(&mut self, scene_data: &mut SceneData, text: String) {
        if let Some(current_terminal) = scene_data.terminals.get_mut(scene_data.player.front_tile) {
            if current_terminal.text.len() <= TERMINAL_LIMIT {
                let new_terminal_text = format!("{}{}", current_terminal.text, text);
                current_terminal.text = Box::new(new_terminal_text);

                self.terminal_text = Some(*current_terminal.text.clone());
            }
        }
    }

    fn clear_terminal(&mut self) {
        self.terminal_text = Some("".to_string());
        self.change_state = Some(InputState::World);
    }

    fn terminal_remove_character(&mut self, scene_data: &mut SceneData) {
        if let Some(current_terminal) = scene_data.terminals.get_mut(scene_data.player.front_tile) {
            if current_terminal.text.len() > 0 {
                let text_len = current_terminal.text.len();
                current_terminal.text.split_off(text_len - 1);

                self.terminal_text = Some(*current_terminal.text.clone());
            }
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
            Keycode::Backspace => {
                self.terminal_remove_character(scene_data);
            },
            Keycode::Escape => {
                self.clear_terminal();
            },
            _ => ()
        }
    }

    fn text_input_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, text: String) {
        self.terminal_add_character(scene_data, text);
    }

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {
        draw_input_state("Terminal", ctx)?;
        if let Some(ref text) = self.terminal_text {
            let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
            let text = graphics::Text::new(ctx, text, &font).unwrap();
            draw_text(ctx, &text)?;
        } else {
            if let Some(terminal) = scene_data.current_terminal() {
                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                let text = graphics::Text::new(ctx, &terminal.text, &font).unwrap();
                draw_text(ctx, &text)?;
            }
        }

        Ok(())
    }
}