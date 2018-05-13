use ggez::{GameResult, Context, event::Keycode, event::Mod};
use app_state::ingame::SceneData;

pub mod edit;
pub mod terminal;
pub mod world;
pub mod inventory;
pub mod circuitry;
pub mod menu;
pub mod npc;
pub mod npc_trade;
pub mod log;
pub mod storage;

pub trait GameState {
    fn change_state(&mut self) -> Option<Box<GameState>> { None }
    
    fn update(&mut self, _scene_data: &mut SceneData) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _scene_data: &mut SceneData, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(&mut self, _scene_data: &mut SceneData, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn key_up_event(&mut self, _scene_data: &mut SceneData, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn text_input_event(&mut self, _scene_data: &mut SceneData, _ctx: &mut Context, _text: String) { }

    fn quit_event(&mut self, _scene_data: &mut SceneData, _ctx: &mut Context) -> bool { false }
}