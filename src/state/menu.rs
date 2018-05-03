use ggez::{graphics, Context, event::*, GameResult};

use storage::SelectionStorage;
use GameState;

pub struct Scene {
	saves: SelectionStorage<String>,

}

impl Scene {
    pub fn new(_ctx: &mut Context) -> GameResult<Scene> {
    	let mut menu = Scene {
    		saves: SelectionStorage::new(),
    	};

    	menu.saves.insert(String::from("dev-level.tar"));

    	Ok(menu)
    }
}

impl GameState for Scene {
    // todo: transitions
}

impl EventHandler for Scene {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> { Ok(()) }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
	    match keycode {
	        Keycode::Up => {
	            self.saves.prev();
	        },
	        Keycode::Down => {
	            self.saves.next();
	        },
	        _ => ()
	    }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        super::draw_selection(&self.saves, ctx, false)?;

        graphics::present(ctx);

        Ok(())
    }
}