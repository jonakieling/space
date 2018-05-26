use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app::{draw_input_state, draw_dialog};
use game::{InputState, GameState};
use world::WorldData;
use dialog::{DialogAction, DialogItem};
use storage::Node;

pub struct Handler {
    change_state: Option<InputState>,
    dialog: Node<DialogItem>
}

impl Handler {
    pub fn new(data: &mut WorldData) -> Handler {
    	Handler {
            change_state: None,
            dialog: data.level.current_terminal().unwrap().dialog.clone()
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

    fn key_up_event(&mut self, _ctx: &mut Context, _data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Return => {
                if self.dialog.children.iter().len() > 0 {
                    if let Some(dialog_item) = self.dialog.children.current() {
                        if let Some(ref action) = dialog_item.value.action {
                            match *action {
                                DialogAction::Map(feature) => {
                                    self.change_state = Some(InputState::Map(feature));
                                },
                                _ => { }
                            }
                        }
                    }
                    self.dialog = self.dialog.children.current().unwrap().clone();	
                } else {
                    self.change_state = Some(InputState::World);
                }
            },
            Keycode::Up => {
                self.dialog.children.prev();
            },
            Keycode::Down => {
                self.dialog.children.next();
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, _data: &mut WorldData) -> GameResult<()> {
        draw_input_state("Terminal", ctx)?;

        draw_dialog(&self.dialog, ctx)
    }
}