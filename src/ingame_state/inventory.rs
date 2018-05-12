use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use app_state::draw_selection_with_parameters;
use app_state::ingame::{SceneData, InputState};
use objects::{Receipe, Item};
use ingame_state::GameState;
use misc::{Position, TextAlign};
use storage::SelectionStorage;

pub struct State {
    craft_area: SelectionStorage<Item>,
    change_state: Option<InputState>
}

impl State {
    pub fn new() -> State {
    	State {
            craft_area:  SelectionStorage::new(),
            change_state: None
        }
    }

    pub fn reset_craft_area(&mut self, scene_data: &mut SceneData) {
        while let Some(item) = self.craft_area.extract_current() {
            scene_data.player.inventory.insert(item);
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
            _ => None,
        }
    }

    fn key_up_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.reset_craft_area(scene_data);
                self.change_state = Some(InputState::World);
            },
            Keycode::I => {
                self.reset_craft_area(scene_data);
                self.change_state = Some(InputState::World);
            },
            Keycode::Up => {
                scene_data.player.inventory.prev();
            },
            Keycode::Down => {
                scene_data.player.inventory.next();
            },
            Keycode::Tab => {
                let item = scene_data.player.inventory.extract_current();
                if item.is_some() {
                    self.craft_area.insert(item.unwrap());
                }
            },
            Keycode::Return => {
                let ref crafts = &self.craft_area.storage();
                let products = Receipe::receipe_match(crafts, &scene_data.receipes);
                if let Some(receipe) = products.get(0) {
                    self.craft_area.clear();
                    scene_data.player.inventory.insert(receipe.result.clone());
                }
            }
            _ => ()
        }
    }

    fn quit_event(&mut self, scene_data: &mut SceneData, _ctx: &mut Context) -> bool {
        self.reset_craft_area(scene_data);
        
        false
    }

    fn draw(&mut self, scene_data: &mut SceneData, ctx: &mut Context) -> GameResult<()> {
        draw_selection_with_parameters(&scene_data.player.inventory, ctx, Position {x: 770, y: 20}, TextAlign::Left, true)?;
        draw_selection_with_parameters(&self.craft_area, ctx, Position {x: 580, y: 20}, TextAlign::Left, false)?;

        Ok(())
    }
}