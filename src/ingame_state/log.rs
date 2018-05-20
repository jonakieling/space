use ggez::{Context, GameResult, graphics};
use ggez::event::{Keycode, Mod};

use app_state::{draw_text, draw_input_state, draw_selection_with_parameters};
use app_state::ingame::{SceneData, InputState};
use ingame_state::GameState;
use misc::{Position, TextAlign};
use player::LogEntry;

const LOG_TITLE_LIMIT: usize = 10;
const LOG_MESSAGE_LIMIT: usize = 20;

enum Mode {
    Title,
    Message,
    Store
}

pub struct State {
    log_title: String,
    log_message: String,
    change_state: Option<InputState>,
    mode: Mode
}

impl State {
    pub fn new() -> State {
    	State {
            log_title: "".to_string(),
            log_message: "".to_string(),
            change_state: None,
            mode: Mode::Title
        }
    }

    fn add_character(&mut self, text: String) {
        match self.mode {
            Mode::Title => {
                if self.log_title.len() <= LOG_TITLE_LIMIT {
                    let text = format!("{}{}", self.log_title, text);
                    self.log_title = text;
                }
            },
            Mode::Message => {
                if self.log_message.len() <= LOG_MESSAGE_LIMIT {
                    let text = format!("{}{}", self.log_message, text);
                    self.log_message = text;
                }
            },
            _ => (),
        }
    }

    fn remove_character(&mut self) {
        match self.mode {
            Mode::Title => {
                let text_len = self.log_title.len();
                if text_len > 0 {
                    self.log_title.split_off(text_len - 1);
                }
            },
            Mode::Message => {
                let text_len = self.log_message.len();
                if text_len > 0 {
                    self.log_message.split_off(text_len - 1);
                }
            },
            _ => (),
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
                self.remove_character();
            },
            Keycode::Up => {
                scene_data.player.log.prev();
            },
            Keycode::Down => {
                scene_data.player.log.next();
            },
            Keycode::Return => {
                match self.mode {
                    Mode::Title => self.mode = Mode::Message,
                    Mode::Message => self.mode = Mode::Store,
                    Mode::Store => {
                        if self.log_title != "".to_string() {
                            let log_entry = LogEntry {
                                title: self.log_title.clone(),
                                message: self.log_message.clone()
                            };
                            scene_data.player.log.insert(log_entry);
                            self.log_title = "".to_string();
                            self.log_message = "".to_string();
                            self.mode = Mode::Title;
                        } else {
                            self.mode = Mode::Title;
                        }
                    },
                }
            },
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Tab => {
                match self.mode {
                    Mode::Title => self.mode = Mode::Message,
                    Mode::Message => self.mode = Mode::Store,
                    Mode::Store => self.mode = Mode::Title,
                }
            },
            _ => ()
        }
    }

    fn text_input_event(&mut self, _scene_data: &mut SceneData, _ctx: &mut Context, text: String) {
        self.add_character(text);
    }

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {

        draw_selection_with_parameters(&scene_data.player.log, ctx, Position {x: 770, y: 45}, TextAlign::Left, true, false)?;

        let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();

        if let Some(ref log_entry) = scene_data.player.log.current() {
            graphics::set_color(ctx, graphics::BLACK)?;
            let text = graphics::Text::new(ctx, &log_entry.message, &font).unwrap();
            let textbox = graphics::Rect::new(770.0 - text.width() as f32, 20.0, text.width() as f32 + 20.0, 20.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, textbox)?;
            graphics::set_color(ctx, graphics::WHITE)?;
            graphics::draw(ctx, &text, graphics::Point2::new(780.0 - text.width() as f32, 20.0), 0.0)?;
        }
        
        match self.mode {
            Mode::Title => {
                draw_input_state("Log Title", ctx)?;
                let text = graphics::Text::new(ctx, &self.log_title, &font).unwrap();
                draw_text(ctx, &text)?;
            },
            Mode::Message => {
                draw_input_state("Log Message", ctx)?;
                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                let text = graphics::Text::new(ctx, &self.log_message, &font).unwrap();
                draw_text(ctx, &text)?;
            },
            Mode::Store => {
                draw_input_state("Log Store", ctx)?;
                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                let text = graphics::Text::new(ctx, "store", &font).unwrap();
                draw_text(ctx, &text)?;
            },
        }

        Ok(())
    }
}