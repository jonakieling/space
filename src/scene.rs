use std::time::Duration;
use ggez::timer;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event;
use ggez::event::*;
use std::ops::Add;

use player::*;
use storage::*;

// pixel scaling
pub const GRID_SIZE: i32 = 20;
// delay of movement in miliseconds
const MOVEMENT_SPEED: u64 = 290;
// width and height of a level in number of tiles
pub const LEVEL_SIZE: i32 = 40;
const TERMINAL_LIMIT: usize = 20;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn viewport_x(self) -> f32 {
        (self.x * GRID_SIZE) as f32
    }

    pub fn viewport_y(self) -> f32 {
        (self.y * GRID_SIZE) as f32
    }
}

impl<'a> Add for &'a Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Wall {
    
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum DoorStatus {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Door {
    status: DoorStatus
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Terminal {
    text: Box<String>
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum InputState {
    Terminal,
    World
}

pub struct Scene {
    movement_timer: Duration,
    pub player: Player,
    player_front_tile: Position,
    pub walls: PositionLevelStorage<Wall>,
    pub doors: PositionLevelStorage<Door>,
    pub terminals: PositionLevelStorage<Terminal>,
    terminal_text: graphics::Text,
    input: InputState,
}

impl Scene {
    pub fn new(_ctx: &mut Context) -> GameResult<Scene> {

        let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
        
        // initialize player and level object storages
        // state and object can be loaded seperatly

        let player = Player {
            position: Position { x: 10, y: 10 },
            movement: vec![],
            direction: Direction::Down
        };
        let player_front_tile = &player.direction.value() + &player.position;

        let walls = <PositionLevelStorage<Wall>>::new();
        let doors = <PositionLevelStorage<Door>>::new();
        let terminals = <PositionLevelStorage<Terminal>>::new();
        
        let scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            player_front_tile,
            walls,
            doors,
            terminals,
            terminal_text: graphics::Text::new(_ctx, "", &font)?,
            input: InputState::World,
        };

        Ok(scene)
    }

    fn check_player_collision(&self) -> bool {
        let mut found_collision = false;

        if let Some(&Some(_)) = self.walls.get(self.player_front_tile.x, self.player_front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(_)) = self.terminals.get(self.player_front_tile.x, self.player_front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(ref door)) = self.doors.get(self.player_front_tile.x, self.player_front_tile.y) {
            if let DoorStatus::Closed = door.status {
                found_collision = true;
            }
        }

        found_collision
    }

    fn interact_with_door(&mut self) {
        if let Some(&mut Some(ref mut door)) = self.doors.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
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
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.movement_timer += timer::get_delta(_ctx);

        if self.movement_timer > Duration::from_millis(MOVEMENT_SPEED) {
            self.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.player.movement.last() {
                if !self.check_player_collision() {
                    self.player.position = &self.player.position + &current_movement.value();
                }
            };
        }

        self.player_front_tile = &self.player.direction.value() + &self.player.position;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if self.input == InputState::World {
            if !_repeat {
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

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
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
                        self.interact_with_door();

                        // interact_with_terminal
                        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
                            self.input = InputState::Terminal;
                            
                            let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                            self.terminal_text = graphics::Text::new(_ctx, &current_terminal.text, &font).unwrap();
                        }
                    },
                    _ => ()
                }
            },
            InputState::Terminal => {
                match keycode {
                    Keycode::Backspace => {
                        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
                            if current_terminal.text.len() > 0 {
                                let text_len = current_terminal.text.len();
                                current_terminal.text.split_off(text_len - 1);

                                let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                                self.terminal_text = graphics::Text::new(_ctx, &current_terminal.text, &font).unwrap();
                            }
                        }
                    },
                    Keycode::Escape => {
                        let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                        self.terminal_text = graphics::Text::new(_ctx, "", &font).unwrap();
                        self.input = InputState::World;
                    },
                    _ => ()
                }
            },
        }
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _text: String) {
        if self.input == InputState::Terminal {

            if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
                if current_terminal.text.len() <= TERMINAL_LIMIT {
                    let new_terminal_text = format!("{}{}", current_terminal.text, _text);
                    current_terminal.text = Box::new(new_terminal_text);

                    let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                    self.terminal_text = graphics::Text::new(_ctx, &current_terminal.text, &font).unwrap();
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, wall) in self.walls.iter().enumerate() {
            // Match for entity presence
            if let &Some(_) = wall {
                let x = pos as i32 % LEVEL_SIZE;
                let y = pos as i32 / LEVEL_SIZE;
                graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
            }
        }

        for (pos, terminal) in self.terminals.iter().enumerate() {
            // Match for entity presence
            if let &Some(_) = terminal {
                let x = pos as i32 % LEVEL_SIZE;
                let y = pos as i32 / LEVEL_SIZE;
                graphics::set_color(ctx, graphics::BLACK)?;
                graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
                graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
                graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
            }
        }

        for (pos, door_pos) in self.doors.iter().enumerate() {
            // Match for entity presence
            if let &Some(ref door) = door_pos {
                let x = pos as i32 % LEVEL_SIZE;
                let y = pos as i32 / LEVEL_SIZE;
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
            }
        }

        graphics::set_color(ctx, graphics::BLACK)?;

        let player = graphics::Rect::new(self.player.position.viewport_x(), self.player.position.viewport_y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.viewport_x() + 5.0 + (self.player.direction.value().viewport_x() * 0.2), self.player.position.viewport_y() + 5.0 + (self.player.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        graphics::draw(ctx, &self.terminal_text, graphics::Point2::new(320.0, 500.0), 0.0)?;

        graphics::present(ctx);

        Ok(())
    }
}