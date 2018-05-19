use dialog::DialogAction;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::{ingame::SceneData, draw_input_state, draw_dialog, ingame::InputState};
use ingame_state::GameState;
use misc::Position;

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

    fn change_state(&mut self) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::State::new()))
            },
            Some(InputState::NpcTrade) => {
                self.change_state = None;
                Some(Box::new(super::npc_trade::State::new()))
            },
            _ => None,
        }
    }
    
    fn key_up_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                if let Some(npc) = scene_data.current_npc() {
                    npc.direction = npc.look_at;
                }
                self.change_state = Some(InputState::World);
            },
            Keycode::Return => {
                if scene_data.dialog.children.iter().len() > 0 {
                    if let Some(dialog_item) = scene_data.dialog.children.current() {
                        if let Some(ref action) = dialog_item.value.action {
                            match *action {
                                DialogAction::Trade => {
                                    self.change_state = Some(InputState::NpcTrade);
                                }
                            }
                        }
                    }
                    scene_data.dialog = scene_data.dialog.children.current().unwrap().clone();	
                } else {
                    if let Some(npc) = scene_data.current_npc() {
                        npc.direction = npc.look_at;
                    }
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

    fn draw(&mut self, scene_data: &mut SceneData, _camera: Position, ctx: &mut Context) -> GameResult<()> {
        {
            let current_npc = scene_data.current_npc().unwrap();
            draw_input_state(&current_npc.name, ctx)?;
        }

        draw_dialog(&scene_data.dialog, ctx)
    }

}
