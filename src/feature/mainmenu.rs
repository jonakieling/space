use std::fs;

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
    File(String),
    DevShip,
    DevStation,
    Empty
}

impl ToString for SaveType {
    fn to_string(&self) -> String {
        match self {
            &SaveType::Empty => "Empty".to_string(),
            &SaveType::DevShip => "DevShip".to_string(),
            &SaveType::DevStation => "DevStation".to_string(),
            &SaveType::File(ref file) => file.clone(),
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

        if let Ok(dir) = fs::read_dir("saves") {
            for entry in dir {
                if let Ok(entry) = entry {
                    if let Some(extension) = entry.path().extension() {
                        if extension == "tar" {
                            if let Some(file) = entry.path().to_str() {
                                menu.saves.insert(SaveType::File(String::from(file)));
                            }
                        }
                    }
                }
            }
        }

        menu.saves.insert(SaveType::Empty);
        menu.saves.insert(SaveType::DevShip);
        menu.saves.insert(SaveType::DevStation);

    	menu
    }
}

impl GameState for Handler {
    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        let mut state: Option<Box<GameState>> = None;
        if let Some(ref savegame) = self.loading {
            match savegame {
                SaveType::Empty => {
                    savegame::static_levels::empty(data);
                    data.overlay = false;
                    state = Some(Box::new(super::world::Handler::new()));
                },
                SaveType::DevShip => {
                    savegame::static_levels::static_ship_tech(data);
                    data.overlay = false;
                    state = Some(Box::new(super::world::Handler::new()));
                },
                SaveType::DevStation => {
                    savegame::static_levels::static_station_outpost(data);
                    data.overlay = false;
                    state = Some(Box::new(super::world::Handler::new()));
                },
                SaveType::File(savefile) => {
                    savegame::load_scene(data, savefile);
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