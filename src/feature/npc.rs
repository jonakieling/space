use dialog::DialogAction;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::{draw_input_state, draw_dialog};
use game::{InputState, GameState};

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
                if data.dialog.children.iter().len() > 0 {
                    if let Some(dialog_item) = data.dialog.children.current() {
                        if let Some(ref action) = dialog_item.value.action {
                            match *action {
                                DialogAction::Trade => {
                                    self.change_state = Some(InputState::NpcTrade);
                                },
                                DialogAction::Map(_) => { }
                            }
                        }
                    }
                    data.dialog = data.dialog.children.current().unwrap().clone();	
                } else {
                    if let Some(npc) = data.level.current_npc() {
                        npc.direction = npc.look_at;
                    }
                    self.change_state = Some(InputState::World);
                }
            },
            Keycode::Up => {
                data.dialog.children.prev();
            },
            Keycode::Down => {
                data.dialog.children.next();
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        {
            let current_npc = data.level.current_npc().unwrap();
            draw_input_state(&current_npc.name, ctx)?;
        }

        draw_dialog(&data.dialog, ctx)
    }

}
