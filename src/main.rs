extern crate ggez;

use std::io::Write;
use std::time::Duration;
use std::collections::HashMap;
use std::ops::Add;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::event::*;

const GRID_SIZE: i32 = 20;

#[derive(PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn x(self) -> f32 {
        (self.x * GRID_SIZE) as f32
    }

    fn y(self) -> f32 {
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

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn value(&self) -> Position {
        match *self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}

struct Player {
    position: Position,
    movement: Vec<Direction>,
    direction: Direction
}

impl Player {
    fn movement(&mut self, direction: Direction, reverse: Direction) {
        if let Some(&current_movement) = self.movement.last() {
            if current_movement == reverse {
                self.remove_movement(current_movement);
            } else {
                if current_movement == self.direction {
                    self.movement.push(direction);
                }
            }    
        } else {
            if direction == self.direction {
                self.movement.push(direction);
            }
        }

        self.direction = direction;
    }

    fn remove_movement(&mut self, direction: Direction) {
        let mut remove_indicies: Vec<usize> = vec![];
        for (index, movement) in self.movement.iter().enumerate() {
            if movement == &direction {
                remove_indicies.push(index);
            }
        }
        for remove_index in remove_indicies.iter() {
            self.movement.remove(*remove_index);
        }
    }
}

struct Wall {
    
}

struct Scene {
    movement_timer: Duration,
    player: Player,
    walls: HashMap<(i32, i32), Wall>
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {

        let player = Player {
            position: Position { x: 10, y: 10 },
            movement: vec![],
            direction: Direction::Down
        };

        let mut walls: HashMap<(i32, i32), Wall> = HashMap::new();
        walls.insert((1,1), Wall {});
        walls.insert((3,3), Wall {});
        walls.insert((5,5), Wall {});
        walls.insert((6,6), Wall {});

        let scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            walls
        };

        Ok(scene)
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.movement_timer += _dt;

        if self.movement_timer > Duration::from_millis(120) {
            self.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.player.movement.last() {
                self.player.position = &self.player.position + &current_movement.value();
            };
        }

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if !_repeat {
            self.movement_timer = Duration::from_millis(120);

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
                        self.player.movement.push(Direction::Left);
                    },
                    Keycode::Right => {
                        self.player.movement.push(Direction::Right);
                    },
                    Keycode::Up => {
                        self.player.movement.push(Direction::Up);
                    },
                    Keycode::Down => {
                        self.player.movement.push(Direction::Down);
                    },
                    _ => ()
                }
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut key_direction = Direction::Up;
        match keycode {
            Keycode::Left => {
                key_direction = Direction::Left
            },
            Keycode::Right => {
                key_direction = Direction::Right
            },
            Keycode::Up => {
                key_direction = Direction::Up
            },
            Keycode::Down => {
                key_direction = Direction::Down
            },
            _ => ()
        }

        self.player.remove_movement(key_direction);

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for pos in self.walls.keys() {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((pos.0 * GRID_SIZE) as f32, (pos.1 * GRID_SIZE) as f32, 20.0, 20.0))?;
        }

        let player = graphics::Rect::new(self.player.position.x(), self.player.position.y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.x() + (self.player.direction.value().x() * 0.2), self.player.position.y() + (self.player.direction.value().y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("space", "ggez", c).unwrap();
    let scene = &mut Scene::new(ctx).unwrap();

    match event::run(ctx, scene) {
        Ok(()) => (),
        Err(e) => {
            writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
            std::process::exit(1);
        }
    }
}
