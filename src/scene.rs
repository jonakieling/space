use std::time::Duration;
use ggez::timer;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event;
use ggez::event::*;

use player::*;
use storage::*;
use objects::*;
use misc::*;
use constants::*;

pub struct Scene {
    movement_timer: Duration,
    pub player: Player,
    pub walls: PositionLevelStorage<Wall>,
    pub doors: PositionLevelStorage<Door>,
    pub terminals: PositionLevelStorage<Terminal>,
    pub circuitry: PositionLevelStorage<Circuitry>,
    terminal_text: graphics::Text,
    input: InputState,
    edit_cursor: Position,
    insight_view: bool,
}

impl Scene {
    pub fn new(_ctx: &mut Context) -> GameResult<Scene> {

        let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
        
        // initialize player and level object storages
        // state and object are loaded seperatly

        let player_position = Position { x: 10, y: 10 };
        let player_direction = Direction::Down;
        let player_front_tile = &player_direction.value() + &player_position;
        let inventory = Box::new(Vec::new());
        let player = Player {
            position: player_position,
            movement: vec![],
            direction: player_direction,
            front_tile: player_front_tile,
            inventory,
            terminal: Box::new(Terminal {
                text: Box::new(String::new()),
                front: Direction::Down
            })
        };

        let walls = <PositionLevelStorage<Wall>>::new();
        let doors = <PositionLevelStorage<Door>>::new();
        let terminals = <PositionLevelStorage<Terminal>>::new();
        let circuitry = <PositionLevelStorage<Circuitry>>::new();
        
        let scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            walls,
            doors,
            terminals,
            circuitry,
            terminal_text: graphics::Text::new(_ctx, "", &font)?,
            input: InputState::World,
            edit_cursor: Position {x: 0, y: 0},
            insight_view: false
        };

        Ok(scene)
    }

    fn check_player_collision(&self) -> bool {
        let mut found_collision = false;

        if let Some(&Some(_)) = self.walls.get(self.player.front_tile.x, self.player.front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(_)) = self.terminals.get(self.player.front_tile.x, self.player.front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(ref door)) = self.doors.get(self.player.front_tile.x, self.player.front_tile.y) {
            if let DoorStatus::Closed = door.status {
                found_collision = true;
            }
        }

        found_collision
    }

    fn interact_with_door(&mut self) {
        if let Some(&mut Some(ref mut door)) = self.doors.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            match door.status {
                DoorStatus::Closed => {
                    door.status = DoorStatus::Open;
                    println!("door opened");
                },
                DoorStatus::Open => {
                    door.status = DoorStatus::Closed;
                    println!("door closed");
                },
            }
        }
    }

    fn interact_with_circuitry(&mut self) {
        if let Some(&mut Some(ref mut current_circuitry)) = self.circuitry.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            println!("accessing circuitry");
            for part in current_circuitry.parts.iter() {
                println!("{:?}", part);
            }
        }
    }

    fn interact_with_terminal(&mut self, ctx: &mut Context) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            let terminal_front_tile = &self.player.front_tile + &current_terminal.front.value();
            if terminal_front_tile == self.player.position {
                self.input = InputState::Terminal;
                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            } else {
                println!("this is not the front of the terminal");
            }
        }
    }

    fn clear_terminal(&mut self, ctx: &mut Context) {
        let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
        self.terminal_text = graphics::Text::new(ctx, "", &font).unwrap();
        self.input = InputState::World;
    }

    fn terminal_remove_character(&mut self, ctx: &mut Context) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            if current_terminal.text.len() > 0 {
                let text_len = current_terminal.text.len();
                current_terminal.text.split_off(text_len - 1);

                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            }
        }
    }

    fn terminal_add_character(&mut self, ctx: &mut Context, text: String) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            if current_terminal.text.len() <= TERMINAL_LIMIT {
                let new_terminal_text = format!("{}{}", current_terminal.text, text);
                current_terminal.text = Box::new(new_terminal_text);

                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            }
        }
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.movement_timer += timer::get_delta(ctx);

        if self.movement_timer > Duration::from_millis(MOVEMENT_SPEED) {
            self.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.player.movement.last() {
                if !self.check_player_collision() {
                    self.player.position = &self.player.position + &current_movement.value();
                }
            };
        }

        self.player.front_tile = &self.player.direction.value() + &self.player.position;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if self.input == InputState::World {
            if !repeat {
                self.movement_timer = Duration::from_millis(MOVEMENT_SPEED);

                match keycode {
                    Keycode::Left => {
                        self.player.movement(Direction::Left, Direction::Right);
                    },
                    Keycode::Right => {
                        self.player.movement(Direction::Right, Direction::Left);
                    },
                    Keycode::Up => {
                        self.player.movement(Direction::Up, Direction::Down);
                    },
                    Keycode::Down => {
                        self.player.movement(Direction::Down, Direction::Up);
                    },
                    Keycode::LCtrl => {
                        self.insight_view = true;
                    }
                    _ => ()
                }
            } else {
                if let None = self.player.movement.last() {
                    match keycode {
                        Keycode::Left => {
                            self.player.movement(Direction::Left, Direction::Right);
                        },
                        Keycode::Right => {
                            self.player.movement(Direction::Right, Direction::Left);
                        },
                        Keycode::Up => {
                            self.player.movement(Direction::Up, Direction::Down);
                        },
                        Keycode::Down => {
                            self.player.movement(Direction::Down, Direction::Up);
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match self.input {
            InputState::World => {
                match keycode {
                    Keycode::Left => {
                        self.player.remove_movement(Direction::Left);
                    },
                    Keycode::Right => {
                        self.player.remove_movement(Direction::Right);
                    },
                    Keycode::Up => {
                        self.player.remove_movement(Direction::Up);
                    },
                    Keycode::Down => {
                        self.player.remove_movement(Direction::Down);
                    },
                    Keycode::Return => {
                        if self.insight_view {
                            self.interact_with_circuitry();
                        } else {
                            self.interact_with_door();
                            self.interact_with_terminal(ctx);
                        }
                    },
                    Keycode::I => {
                        println!("player inventory:");
                        for item in self.player.inventory.iter() {
                            println!("{:?}", item);
                        }
                    },
                    Keycode::Insert => {
                        self.input = InputState::Edit;
                    },
                    Keycode::LCtrl => {
                        self.insight_view = false;
                    },
                    _ => ()
                }
            },
            InputState::Terminal => {
                match keycode {
                    Keycode::Backspace => {
                        self.terminal_remove_character(ctx);
                    },
                    Keycode::Escape => {
                        self.clear_terminal(ctx);
                    },
                    _ => ()
                }
            },
            InputState::Edit => {
                match keycode {
                    Keycode::Escape => {
                        self.input = InputState::World;
                    },
                    Keycode::Left => {
                        self.edit_cursor = &self.edit_cursor + &Direction::Left.value();
                    },
                    Keycode::Right => {
                        self.edit_cursor = &self.edit_cursor + &Direction::Right.value();
                    },
                    Keycode::Up => {
                        self.edit_cursor = &self.edit_cursor + &Direction::Up.value();
                    },
                    Keycode::Down => {
                        self.edit_cursor = &self.edit_cursor + &Direction::Down.value();
                    },
                    Keycode::Delete => {
                        self.walls.remove(self.edit_cursor.x, self.edit_cursor.y);
                        self.doors.remove(self.edit_cursor.x, self.edit_cursor.y);
                        self.terminals.remove(self.edit_cursor.x, self.edit_cursor.y);
                        self.circuitry.remove(self.edit_cursor.x, self.edit_cursor.y);
                    },
                    Keycode::W => {
                        self.walls.insert(self.edit_cursor.x, self.edit_cursor.y, Wall {});
                    },
                    Keycode::C => {
                        self.circuitry.insert(self.edit_cursor.x, self.edit_cursor.y, Circuitry {parts: Box::new(Vec::new())});
                    },
                    Keycode::D => {
                        self.doors.insert(self.edit_cursor.x, self.edit_cursor.y, Door { status: DoorStatus::Closed});
                    },
                    Keycode::T => {
                        self.terminals.insert(self.edit_cursor.x, self.edit_cursor.y, Terminal { text: Box::new(String::new()), front: Direction::Down});
                    },
                    Keycode::Tab => {
                        if let Some(&mut Some(ref mut door)) = self.doors.get_mut(self.edit_cursor.x, self.edit_cursor.y) {
                            match door.status {
                                DoorStatus::Open => {
                                    door.status = DoorStatus::Closed;
                                },
                                DoorStatus::Closed => {
                                    door.status = DoorStatus::Open;
                                }
                            }
                        }
                        if let Some(&mut Some(ref mut terminal)) = self.terminals.get_mut(self.edit_cursor.x, self.edit_cursor.y) {
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
        }
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        if self.input == InputState::Terminal {
            self.terminal_add_character(ctx, text);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, wall) in self.walls.iter().enumerate() {
            if let &Some(_) = wall {
                draw_wall(pos as i32, ctx)?;
            }
        }

        for (pos, terminal) in self.terminals.iter().enumerate() {
            if let &Some(ref current_terminal) = terminal {
                draw_terminal(pos as i32, &current_terminal.front, ctx)?;
            }
        }

        for (pos, item) in self.doors.iter().enumerate() {
            if let &Some(ref door) = item {
                draw_door(door, pos as i32, ctx)?;
            }
        }

        if self.insight_view {
            for (pos, circuitry) in self.circuitry.iter().enumerate() {
                if let &Some(_) = circuitry {
                    draw_circuitry(pos as i32, ctx)?;
                }
            }
        }

        graphics::set_color(ctx, graphics::BLACK)?;
        let player = graphics::Rect::new(self.player.position.viewport_x(), self.player.position.viewport_y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.viewport_x() + 5.0 + (self.player.direction.value().viewport_x() * 0.2), self.player.position.viewport_y() + 5.0 + (self.player.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        if self.input == InputState::Terminal {
            graphics::set_color(ctx, graphics::BLACK)?;
            let console = graphics::Rect::new(260.0, 500.0, self.terminal_text.width() as f32 + 20.0, 20.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, console)?;
            graphics::set_color(ctx, graphics::WHITE)?;
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), console)?;
            graphics::draw(ctx, &self.terminal_text, graphics::Point2::new(270.0, 500.0), 0.0)?;
        }

        if self.input == InputState::Edit {
            graphics::set_color(ctx, graphics::Color{r: 0.2, g: 0.8, b: 0.2, a: 1.0,})?;
            let player = graphics::Rect::new(self.edit_cursor.viewport_x(), self.edit_cursor.viewport_y(), 21.0, 21.0);
            graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), player)?;
        }

        graphics::present(ctx);

        Ok(())
    }
}

fn draw_wall(pos: i32, ctx: &mut Context) -> GameResult<()> {
    let x = pos % LEVEL_SIZE;
    let y = pos / LEVEL_SIZE;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;

    Ok(())
}

fn draw_circuitry(pos: i32, ctx: &mut Context) -> GameResult<()> {
    let x = pos % LEVEL_SIZE;
    let y = pos / LEVEL_SIZE;
    graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 0.1,})?;
    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 3.0, (y * GRID_SIZE) as f32 + 3.0, 15.0, 15.0))?;
    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 5.0, (y * GRID_SIZE) as f32 + 5.0, 11.0, 11.0))?;

    Ok(())
}

fn draw_door(door: &Door, pos: i32, ctx: &mut Context) -> GameResult<()> {
    let x = pos % LEVEL_SIZE;
    let y = pos / LEVEL_SIZE;
    match door.status {
        DoorStatus::Open => {
            graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
            graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
        },
        DoorStatus::Closed => {
            graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
        },
    }

    Ok(())
}

fn draw_terminal(pos: i32, direction: &Direction, ctx: &mut Context) -> GameResult<()> {
    let x = pos % LEVEL_SIZE;
    let y = pos / LEVEL_SIZE;
    graphics::set_color(ctx, graphics::BLACK)?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
    graphics::set_color(ctx, graphics::Color{r: 0.5, g: 0.8, b: 0.5, a: 1.0,})?;
    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
    match *direction {
        Direction::Up => {
            let front = graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 3.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
        },
        Direction::Down => {
            let front = graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32 + (direction.value().y as f32 * 17.0), 21.0, 4.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
            
        },
        Direction::Right => {
            let front = graphics::Rect::new((x * GRID_SIZE) as f32 + (direction.value().x as f32 * 17.0), (y * GRID_SIZE) as f32 + (direction.value().y as f32), 4.0, 21.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
            
        },
        Direction::Left => {
            let front = graphics::Rect::new((x * GRID_SIZE) as f32 + (direction.value().x as f32), (y * GRID_SIZE) as f32 + (direction.value().y as f32), 4.0, 21.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
            
        },
    }

    Ok(())
}