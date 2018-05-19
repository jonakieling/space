use ggez::{Context, GameResult, graphics};
use ggez::event::{Keycode, Mod};

use app_state::{ingame::InputState, ingame::SceneData, draw_selection};
use ingame_state::GameState;
use misc::Direction;
use objects::*;
use storage::SelectionStorage;
use misc::Position;

pub struct State {
    edit_cursor: Position,
    edit_selection: SelectionStorage<String>,
    change_state: Option<InputState>
}

impl State {
    pub fn new() -> State {
    	State {
            edit_cursor: Position {x: 0, y: 0},
            edit_selection: SelectionStorage::new(),
            change_state: None
        }
    }

    fn get_edit_selection(&mut self, scene_data: &mut SceneData) -> SelectionStorage<String> {
        let mut selection_storage: SelectionStorage<String> = SelectionStorage::new();
        if let Some(_) = scene_data.walls.get(self.edit_cursor) {
            selection_storage.insert("Wall".to_string());
        }
        
        if let Some(_) = scene_data.doors.get(self.edit_cursor) {
            selection_storage.insert("Door".to_string());
        }
        
        if let Some(_) = scene_data.terminals.get(self.edit_cursor) {
            selection_storage.insert("Terminal".to_string());
        }
        
        if let Some(_) = scene_data.circuitry.get(self.edit_cursor) {
            selection_storage.insert("Circuitry".to_string());
        }
        
        if let Some(_) = scene_data.generators.get(self.edit_cursor) {
            selection_storage.insert("Generator".to_string());
        }
        
        if let Some(npc) = scene_data.npc.get(self.edit_cursor) {
            selection_storage.insert(npc.name.clone());
        }
        
        if let Some(_) = scene_data.storages.get(self.edit_cursor) {
            selection_storage.insert("Storage".to_string());
        }

        if self.edit_cursor.x == scene_data.player.position.x && self.edit_cursor.y == scene_data.player.position.y {
            selection_storage.insert("Player".to_string());
        }

        selection_storage
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

        self.edit_selection = self.get_edit_selection(scene_data);
        
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Left => {
                self.edit_cursor = &self.edit_cursor + &Direction::Left.value();
                self.edit_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Right => {
                self.edit_cursor = &self.edit_cursor + &Direction::Right.value();
                self.edit_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Up => {
                self.edit_cursor = &self.edit_cursor + &Direction::Up.value();
                self.edit_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Down => {
                self.edit_cursor = &self.edit_cursor + &Direction::Down.value();
                self.edit_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Delete => {
                scene_data.walls.remove(self.edit_cursor);
                scene_data.doors.remove(self.edit_cursor);
                scene_data.terminals.remove(self.edit_cursor);
                scene_data.circuitry.remove(self.edit_cursor);
                scene_data.generators.remove(self.edit_cursor);
                scene_data.storages.remove(self.edit_cursor);
                scene_data.update_power();
            },
            Keycode::W => {
                scene_data.walls.insert(self.edit_cursor, Wall { variant: WallType::Wall, face: Direction::Right});
            },
            Keycode::C => {
                scene_data.circuitry.insert(self.edit_cursor, Circuitry {parts: SelectionStorage::new(), powered: false});
                scene_data.update_power();
            },
            Keycode::G => {
                scene_data.generators.insert(self.edit_cursor, Generator { face: Direction::Down });
                scene_data.update_power();
            },
            Keycode::S => {
                scene_data.storages.insert(self.edit_cursor, Storage { content: SelectionStorage::new(), face: Direction::Down });
            },
            Keycode::D => {
                scene_data.doors.insert(self.edit_cursor, Door { status: DoorStatus::Closed, face: Direction::Down});
            },
            Keycode::T => {
                scene_data.terminals.insert(self.edit_cursor, Terminal { variant: TerminalType::Intercomm, text: Box::new(String::new()), front: Direction::Down});
            },
            Keycode::Tab => {
                if let Some(ref mut door) = scene_data.doors.get_mut(self.edit_cursor) {
                    match door.status {
                        DoorStatus::Open => {
                            door.status = DoorStatus::Closed;
                        },
                        DoorStatus::Closed => {
                            door.status = DoorStatus::Open;
                        }
                    }
                }
                if let Some(ref mut terminal) = scene_data.terminals.get_mut(self.edit_cursor) {
                    match terminal.front {
                        Direction::Up => {
                            terminal.front = Direction::Right;
                        },
                        Direction::Right => {
                            terminal.front = Direction::Down;
                        },
                        Direction::Down => {
                            terminal.front = Direction::Left;
                        },
                        Direction::Left => {
                            terminal.front = Direction::Up;
                        },
                    }
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, _scene_data: &mut SceneData, camera: Position, ctx: &mut Context) -> GameResult<()> {
        draw_selection(&self.edit_selection, ctx, false, false)?;

        graphics::set_color(ctx, graphics::Color{r: 0.2, g: 0.8, b: 0.2, a: 1.0,})?;
        let edit_cursor = graphics::Rect::new(self.edit_cursor.viewport_x(camera), self.edit_cursor.viewport_y(camera), 21.0, 21.0);
        graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), edit_cursor)?;

        Ok(())
    }
}

