use std::fs;

use ggez::{graphics, Context, event::*, GameResult};

use storage::SelectionStorage;
use GameState;
use savegame;
use app_state::ingame;
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
    pub fn new() -> GameResult<Handler> {
    	let mut menu = Handler {
    		saves: SelectionStorage::new(),
            loading: None
    	};

        for entry in fs::read_dir("saves")? {
            let dir = entry?;
            if let Some(extension) = dir.path().extension() {
                if extension == "tar" {
                    menu.saves.insert(SaveType::File(String::from(dir.path().to_str().unwrap())));
                }
            }
        }

        menu.saves.insert(SaveType::Empty);
        menu.saves.insert(SaveType::DevShip);
        menu.saves.insert(SaveType::DevStation);

    	Ok(menu)
    }
}

impl GameState for Handler {
    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        if let Some(ref savegame) = self.loading {
            match savegame {
                SaveType::Empty => {
                    let mut world = ingame::Handler::new();
                    data.clear();
                    savegame::static_levels::empty(data);
                    Some(Box::new(world))
                },
                SaveType::DevShip => {
                    let mut world = ingame::Handler::new();
                    data.clear();
                    savegame::static_levels::static_ship_tech(data);
                    Some(Box::new(world))
                },
                SaveType::DevStation => {
                    let mut world = ingame::Handler::new();
                    data.clear();
                    savegame::static_levels::static_station_outpost(data);
                    Some(Box::new(world))
                },
                SaveType::File(savefile) => {
                    let mut world = ingame::Handler::new();
                    data.clear();
                    savegame::load_scene(data, savefile);
                    Some(Box::new(world))
                },
            }
        } else {
            None
        }
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
        graphics::clear(ctx);

        draw_selection(&self.saves, ctx, true, false)?;

        graphics::present(ctx);

        Ok(())
    }
}