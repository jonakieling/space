use ggez::{Context, event::*, GameResult};

use storage::SelectionStorage;
use savegame;
use game::GameState;
use world::WorldData;
use app::draw_selection;

pub struct Handler {
	saves: SelectionStorage<SaveType>,
    loading: Option<SaveType>
}

#[derive(Clone, Debug)]
enum SaveType {
    New,
    Continue
}

impl ToString for SaveType {
    fn to_string(&self) -> String {
        match self {
            &SaveType::New => "New".to_string(),
            &SaveType::Continue => "Continue".to_string(),
        }
    }
}

impl Handler {
    pub fn new(data: &mut WorldData) -> Handler {
        data.overlay = true;
    	let mut menu = Handler {
    		saves: SelectionStorage::new(),
            loading: None
    	};

        menu.saves.insert(SaveType::New);
        menu.saves.insert(SaveType::Continue);

    	menu
    }
}

impl GameState for Handler {
    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        let mut state: Option<Box<GameState>> = None;
        if let Some(ref savegame) = self.loading {
            match savegame {
                SaveType::New => {
                    savegame::static_levels::static_ship_tech(data);
                    data.overlay = false;
                    state = Some(Box::new(super::world::Handler::new()));
                },
                SaveType::Continue => {
                    savegame::load_game(data);
                    data.overlay = false;
                    state = Some(Box::new(super::world::Handler::new()));
                }
            }
        }
        self.loading = None;

        state
    }
    
    fn key_up_event(&mut self, ctx: &mut Context, _world_data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
	    match keycode {
	        Keycode::Up => {
	            self.saves.prev();
	        },
	        Keycode::Down => {
	            self.saves.next();
	        },
            Keycode::Return => {
                self.loading = Some(self.saves.current().unwrap().clone())
            },
            Keycode::Escape => {
                ctx.quit().expect("game should have quit");
            },
	        _ => ()
	    }
    }

    fn draw(&mut self, ctx: &mut Context, _world_data: &mut WorldData) -> GameResult<()> {
        draw_selection(&self.saves, ctx, true, false)
    }
}