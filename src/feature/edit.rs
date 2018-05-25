use ggez::graphics::get_screen_coordinates;
use ggez::{Context, GameResult, graphics};
use ggez::event::{Keycode, Mod, LSHIFTMOD};

use world::WorldData;
use app::draw_selection;
use game::{InputState, GameState};
use misc::Direction;
use objects::*;
use storage::{SelectionStorage, Node};
use misc::Position;
use constants::GRID_SIZE;

pub struct Handler {
    edit_cursor: Position,
    edit_selection: SelectionStorage<String>,
    change_state: Option<InputState>
}

impl Handler {
    pub fn new(init: Position) -> Handler {
    	Handler {
            edit_cursor: init,
            edit_selection: SelectionStorage::new(),
            change_state: None
        }
    }

    fn get_edit_selection(&mut self, data: &mut WorldData) -> SelectionStorage<String> {
        let mut selection_storage: SelectionStorage<String> = SelectionStorage::new();
        if let Some(_) = data.level.walls.get(self.edit_cursor) {
            selection_storage.insert("Wall".to_string());
        }
        
        if let Some(_) = data.level.doors.get(self.edit_cursor) {
            selection_storage.insert("Door".to_string());
        }
        
        if let Some(_) = data.level.terminals.get(self.edit_cursor) {
            selection_storage.insert("Terminal".to_string());
        }
        
        if let Some(_) = data.level.circuitry.get(self.edit_cursor) {
            selection_storage.insert("Circuitry".to_string());
        }
        
        if let Some(_) = data.level.generators.get(self.edit_cursor) {
            selection_storage.insert("Generator".to_string());
        }
        
        if let Some(npc) = data.level.npc.get(self.edit_cursor) {
            selection_storage.insert(npc.name.clone());
        }
        
        if let Some(_) = data.level.storages.get(self.edit_cursor) {
            selection_storage.insert("Storage".to_string());
        }

        if self.edit_cursor.x == data.level.player.position.x && self.edit_cursor.y == data.level.player.position.y {
            selection_storage.insert("Player".to_string());
        }

        selection_storage
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
    
    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, keymod: Mod, _repeat: bool) {

        self.edit_selection = self.get_edit_selection(data);
        
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Left => {
                self.edit_cursor = &self.edit_cursor + &Direction::Left.value();
                self.edit_selection = self.get_edit_selection(data);
            },
            Keycode::Right => {
                self.edit_cursor = &self.edit_cursor + &Direction::Right.value();
                self.edit_selection = self.get_edit_selection(data);
            },
            Keycode::Up => {
                self.edit_cursor = &self.edit_cursor + &Direction::Up.value();
                self.edit_selection = self.get_edit_selection(data);
            },
            Keycode::Down => {
                self.edit_cursor = &self.edit_cursor + &Direction::Down.value();
                self.edit_selection = self.get_edit_selection(data);
            },
            Keycode::Delete => {
                data.level.walls.remove(self.edit_cursor);
                data.level.doors.remove(self.edit_cursor);
                data.level.terminals.remove(self.edit_cursor);
                data.level.circuitry.remove(self.edit_cursor);
                data.level.generators.remove(self.edit_cursor);
                data.level.storages.remove(self.edit_cursor);
                data.level.floor.remove(self.edit_cursor);
                data.level.update_power();
            },
            Keycode::W => {
                data.level.walls.insert(self.edit_cursor, Wall { variant: WallType::Wall, face: Direction::Right});
            },
            Keycode::C => {
                let mut parts = SelectionStorage::new();
                parts.insert(Item::PowerConductor);
                data.level.circuitry.insert(self.edit_cursor, Circuitry {parts, variant: CircuitryType::Inactive});
                data.level.update_power();
            },
            Keycode::G => {
                data.level.generators.insert(self.edit_cursor, Generator { face: Direction::Down });
                data.level.update_power();
            },
            Keycode::S => {
                data.level.storages.insert(self.edit_cursor, Storage { content: SelectionStorage::new(), face: Direction::Down });
            },
            Keycode::D => {
                data.level.doors.insert(self.edit_cursor, Door { status: DoorStatus::Closed, variant: DoorType::Passage, face: Direction::Down});
            },
            Keycode::T => {
                data.level.terminals.insert(self.edit_cursor, Terminal { variant: TerminalType::Intercomm, dialog: Node::new(), front: Direction::Down});
            },
            Keycode::Tab => {
                if let Some(ref mut generator) = data.level.generators.get_mut(self.edit_cursor) {
                    match generator.face {
                        Direction::Up => {
                            generator.face = Direction::Right;
                        },
                        Direction::Right => {
                            generator.face = Direction::Down;
                        },
                        Direction::Down => {
                            generator.face = Direction::Left;
                        },
                        Direction::Left => {
                            generator.face = Direction::Up;
                        },
                    }
                }
                if let Some(ref mut storage) = data.level.storages.get_mut(self.edit_cursor) {
                    match storage.face {
                        Direction::Up => {
                            storage.face = Direction::Right;
                        },
                        Direction::Right => {
                            storage.face = Direction::Down;
                        },
                        Direction::Down => {
                            storage.face = Direction::Left;
                        },
                        Direction::Left => {
                            storage.face = Direction::Up;
                        },
                    }
                }
                if let Some(ref mut wall) = data.level.walls.get_mut(self.edit_cursor) {
                    if keymod == LSHIFTMOD {
                        match wall.variant {
                            WallType::Corner => {
                                wall.variant = WallType::Edge;
                            },
                            WallType::Edge => {
                                wall.variant = WallType::Wall;
                            },
                            WallType::Wall => {
                                wall.variant = WallType::Window;
                            },
                            WallType::Window => {
                                wall.variant = WallType::Corner;
                            }
                        }
                    } else {
                        match wall.face {
                            Direction::Up => {
                                wall.face = Direction::Right;
                            },
                            Direction::Right => {
                                wall.face = Direction::Down;
                            },
                            Direction::Down => {
                                wall.face = Direction::Left;
                            },
                            Direction::Left => {
                                wall.face = Direction::Up;
                            },
                        }
                    }
                }
                if let Some(ref mut door) = data.level.doors.get_mut(self.edit_cursor) {
                    if keymod == LSHIFTMOD {
                        match door.status {
                            DoorStatus::Open => {
                                door.status = DoorStatus::Closed;
                            },
                            DoorStatus::Closed => {
                                door.status = DoorStatus::Open;
                            }
                        }
                    } else {
                        match door.face {
                            Direction::Up => {
                                door.face = Direction::Right;
                            },
                            Direction::Right => {
                                door.face = Direction::Down;
                            },
                            Direction::Down => {
                                door.face = Direction::Left;
                            },
                            Direction::Left => {
                                door.face = Direction::Up;
                            },
                        }
                    }
                }
                if let Some(ref mut terminal) = data.level.terminals.get_mut(self.edit_cursor) {
                    if keymod == LSHIFTMOD {
                        match terminal.variant {
                            TerminalType::Intercomm => {
                                terminal.variant = TerminalType::ShipConsole;
                            },
                            TerminalType::ShipConsole => {
                                terminal.variant = TerminalType::Intercomm;
                            },
                            _ => { }
                        }
                    } else {
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
                }
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        draw_selection(&self.edit_selection, ctx, false, false)?;

        graphics::set_color(ctx, graphics::Color{r: 0.2, g: 0.8, b: 0.2, a: 1.0,})?;

        let viewport_pos = self.edit_cursor.viewport(data.camera);

        let sceen_horizontal_center = get_screen_coordinates(ctx).w / 2.0 - (GRID_SIZE / 2) as f32;
        let sceen_vertical_center = get_screen_coordinates(ctx).h / 2.0 - (GRID_SIZE / 2) as f32;
        let edit_cursor = graphics::Rect::new(
            viewport_pos.x as f32 + sceen_horizontal_center,
            viewport_pos.y as f32 + sceen_vertical_center,
            GRID_SIZE as f32,
            GRID_SIZE as f32
        );
        graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), edit_cursor)?;

        Ok(())
    }
}

