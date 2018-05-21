use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use world::WorldData;
use app::draw_selection;
use app_state::ingame::InputState;
use GameState;
use storage::SelectionStorage;

#[derive(Debug, Clone)]
pub enum MenuOption {
    Quit,
    Menu
}

impl ToString for MenuOption {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub struct Handler {
    menu: SelectionStorage<MenuOption>,
    change_state: Option<InputState>
}

impl Handler {
    pub fn new() -> Handler {
        let mut menu = SelectionStorage::new();
        menu.insert(MenuOption::Menu);
        menu.insert(MenuOption::Quit);
    	Handler { menu, change_state: None }
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, _scene_data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                Some(Box::new(super::world::Handler::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, scene_data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                self.menu.prev();
            },
            Keycode::Down => {
                self.menu.next();
            },
            Keycode::Return => {
                match *self.menu.current().unwrap() {
                    MenuOption::Quit => {
                        ctx.quit().expect("game should have quit");
                    },
                    MenuOption::Menu => {
                        scene_data.main_menu = true;
                    },
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, _scene_data: &mut WorldData) -> GameResult<()> {
        draw_selection(&self.menu, ctx, true, false)?;

        Ok(())
    }
}