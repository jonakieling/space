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

#[derive(PartialEq, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32
}

impl<'a> Add for &'a Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn value(&self) -> Vec2 {
        match *self {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Right => Vec2 { x: 1, y: 0 },
        }
    }
}

struct Player {
    position: Vec2,
    direction: Direction
}

struct Wall {
    
}

struct Scene {
    movement: Vec<Vec2>,
    player: Player,
    walls: HashMap<(i32, i32), Wall>
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {

        let player = Player {
            position: Vec2 { x: 200, y: 200 },
            direction: Direction::Down
        };

        let mut walls: HashMap<(i32, i32), Wall> = HashMap::new();
        walls.insert((20,20), Wall {});
        walls.insert((60,60), Wall {});
        walls.insert((100,100), Wall {});
        walls.insert((140,140), Wall {});

        let scene = Scene {
            movement: vec![],
            player,
            walls
        };

        Ok(scene)
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {

        if let Some(current_movement) = self.movement.last() {
            self.player.position = &self.player.position + current_movement;
        };

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if !_repeat {
            match keycode {
                Keycode::Left => {
                    self.movement.push(Direction::Left.value());
                },
                Keycode::Right => {
                    self.movement.push(Direction::Right.value());
                },
                Keycode::Up => {
                    self.movement.push(Direction::Up.value());
                },
                Keycode::Down => {
                    self.movement.push(Direction::Down.value());
                },
                _ => ()
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut key_direction = Vec2 { x: 0, y: 0 };
        match keycode {
            Keycode::Left => {
                key_direction = Direction::Left.value()
            },
            Keycode::Right => {
                key_direction = Direction::Right.value()
            },
            Keycode::Up => {
                key_direction = Direction::Up.value()
            },
            Keycode::Down => {
                key_direction = Direction::Down.value()
            },
            _ => ()
        }

        let mut remove_indicies: Vec<usize> = vec![];
        for (index, movement) in self.movement.iter().enumerate() {
            if movement == &key_direction {
                remove_indicies.push(index);
            }
        }
        for remove_index in remove_indicies.iter() {
            self.movement.remove(*remove_index);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for pos in self.walls.keys() {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(pos.0 as f32, pos.1 as f32, 20.0, 20.0))?;
        }

        let player = graphics::Rect::new(self.player.position.x as f32, self.player.position.y as f32, 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.x as f32 + (self.player.direction.value().x as f32 * 7.0), self.player.position.y as f32 + (self.player.direction.value().y as f32 * 7.0), 10.0, 10.0);
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
