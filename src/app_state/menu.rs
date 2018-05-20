use std::fs;

use ggez::{graphics, Context, event::*, GameResult};

use storage::SelectionStorage;
use AppState;
use savegame;
use app_state::ingame;

pub struct Scene {
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

impl Scene {
    pub fn new() -> GameResult<Scene> {
    	let mut menu = Scene {
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

impl AppState for Scene {
    fn change_state(&self, ctx: &mut Context) -> Option<Box<AppState>> {
        if let Some(ref savegame) = self.loading {
            match savegame {
                SaveType::Empty => {
                    let mut world = ingame::Scene::new(ctx).unwrap();
                    savegame::static_levels::empty(&mut world.data);
                    Some(Box::new(world))
                },
                SaveType::DevShip => {
                    let mut world = ingame::Scene::new(ctx).unwrap();
                    savegame::static_levels::static_ship_tech(&mut world.data);
                    Some(Box::new(world))
                },
                SaveType::DevStation => {
                    let mut world = ingame::Scene::new(ctx).unwrap();
                    savegame::static_levels::static_station_outpost(&mut world.data);
                    Some(Box::new(world))
                },
                SaveType::File(savefile) => {
                    let mut world = ingame::Scene::new(ctx).unwrap();
                    savegame::load_scene(&mut world.data, savefile);
                    Some(Box::new(world))
                },
            }
        } else {
            None
        }
    }
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
            Keycode::Return => {
                self.loading = Some(self.saves.current().unwrap().clone())
            },
	        _ => ()
	    }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        super::draw_selection(&self.saves, ctx, true, false)?;

        graphics::present(ctx);

        Ok(())
    }
}