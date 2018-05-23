use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::{draw_selection, draw_tile};
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

    fn change_state(&mut self, _ctx: &mut Context, _data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::Handler::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                if let Some(current_circuitry) = data.level.current_circuitry() {
                    current_circuitry.parts.prev();
                }
            },
            Keycode::Down => {
                if let Some(current_circuitry) = data.level.current_circuitry() {
                    current_circuitry.parts.next();
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        draw_selection(&data.level.current_circuitry().unwrap().parts, ctx, true, false)?;

        if !data.insight_view {
            let front_index = data.level.player.front_tile.to_int();
            let camera = data.camera;
            if let Some(circuitry) = data.level.current_circuitry() {
                draw_tile(ctx, circuitry.tile(), front_index, camera, None)?;
            }
        }

        Ok(())
    }
            
}