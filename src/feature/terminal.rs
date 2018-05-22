use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app::{draw_input_state, draw_dialog};
use game::{InputState, GameState};
use world::WorldData;
use dialog::DialogAction;

pub struct Handler {
    change_state: Option<InputState>
}

impl Handler {
    pub fn new() -> Handler {
    	Handler {
            change_state: None
        }
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::Handler::new()))
            },
            Some(InputState::Map(feature)) => {
                self.change_state = None;
                Some(Box::new(super::map::Handler::new(feature, data)))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, scene_data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Return => {
                if scene_data.dialog.children.iter().len() > 0 {
                    if let Some(dialog_item) = scene_data.dialog.children.current() {
                        if let Some(ref action) = dialog_item.value.action {
                            match *action {
                                DialogAction::Map(feature) => {
                                    self.change_state = Some(InputState::Map(feature));
                                },
                                _ => { }
                            }
                        }
                    }
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

    fn draw(&mut self, ctx: &mut Context, scene_data: &mut WorldData) -> GameResult<()> {
        draw_input_state("Terminal", ctx)?;

        draw_dialog(&scene_data.dialog, ctx)
    }
}