use std::fs;

use ggez::{graphics, Context, event::*, GameResult};

use storage::SelectionStorage;
use GameState;
use level;
use state::world;

pub struct Scene {
	saves: SelectionStorage<String>,
    loading: Option<String>
}

impl Scene {
    pub fn new(_ctx: &mut Context) -> GameResult<Scene> {
    	let mut menu = Scene {
    		saves: SelectionStorage::new(),
            loading: None
    	};

        for entry in fs::read_dir("levels")? {
            let dir = entry?;
            if let Some(extension) = dir.path().extension() {
                if extension == "tar" {
                    menu.saves.insert(String::from(dir.path().to_str().unwrap()));
                }
            }
        }

        for entry in fs::read_dir("saves")? {
            let dir = entry?;
            if let Some(extension) = dir.path().extension() {
                if extension == "tar" {
                    menu.saves.insert(String::from(dir.path().to_str().unwrap()));
                }
            }
        }

        menu.saves.insert("empty".to_string());

    	Ok(menu)
    }
}

impl GameState for Scene {
    fn change_state(&self, ctx: &mut Context) -> Option<Box<GameState>> {
        if let Some(ref savegame) = self.loading {
            if savegame == "empty" {
                let mut world = world::Scene::new(ctx).unwrap();
                level::static_levels::static_level0(&mut world);
                Some(Box::new(world))
            } else {
                let mut world = world::Scene::new(ctx).unwrap();
                level::load_scene(&mut world, savegame);
                Some(Box::new(world))
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

        super::draw_selection(&self.saves, ctx, true)?;

        graphics::present(ctx);

        Ok(())
    }
}