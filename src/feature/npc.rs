use dialog::DialogAction;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::{draw_input_state, draw_dialog};
use game::{InputState, GameState};
use storage::Node;
use dialog::DialogItem;

pub struct Handler {
    change_state: Option<InputState>,
    dialog: Node<DialogItem>
}

impl Handler {
    pub fn new(data: &mut WorldData) -> Handler {
    	Handler {
            change_state: None,
            dialog: data.level.current_npc().unwrap().dialog.clone()
        }
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, _scene_data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::Handler::new()))
            },
            Some(InputState::NpcTrade) => {
                self.change_state = None;
                Some(Box::new(super::npc_trade::Handler::new()))
            },
            _ => None,
        }
    }
    
    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                if let Some(npc) = data.level.current_npc() {
                    npc.direction = npc.look_at;
                }
                self.change_state = Some(InputState::World);
            },
            Keycode::Return => {
                if self.dialog.children.iter().len() > 0 {
                    if let Some(dialog_item) = self.dialog.children.current() {
                        if let Some(ref action) = dialog_item.value.action {
                            match *action {
                                DialogAction::Trade => {
                                    self.change_state = Some(InputState::NpcTrade);
                                },
                                DialogAction::Map(_) => { }
                            }
                        }
                    }
                    self.dialog = self.dialog.children.current().unwrap().clone();	
                } else {
                    if let Some(npc) = data.level.current_npc() {
                        npc.direction = npc.look_at;
                    }
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

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        {
            let current_npc = data.level.current_npc().unwrap();
            draw_input_state(&current_npc.name, ctx)?;
        }

        draw_dialog(&self.dialog, ctx)
    }

}
