extern crate ggez;

use std::io::Write;
use std::time::Duration;
use std::collections::HashMap;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::event::*;

#[derive(Debug)]
enum CurrentDirection {
    Up,
    Down,
    Left,
    Right
}

struct Direction {
    x: f32,
    y: f32
}

impl CurrentDirection {
    fn current(&self) -> Direction {
        match *self {
            CurrentDirection::Up => Direction { x: 0.0, y: -1.0 },
            CurrentDirection::Down => Direction { x: 0.0, y: 1.0 },
            CurrentDirection::Left => Direction { x: -1.0, y: 0.0 },
            CurrentDirection::Right => Direction { x: 1.0, y: 0.0 },
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    direction: CurrentDirection
}

struct Wall {
    
}

struct Scene {
    player: Player,
    walls: HashMap<(i32, i32), Wall>
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {
        let player = Player {
            x: 200,
            y: 200,
            direction: CurrentDirection::Down
        };

        let mut walls: HashMap<(i32, i32), Wall> = HashMap::new();
        walls.insert((20,20), Wall {});
        walls.insert((60,60), Wall {});
        walls.insert((100,100), Wall {});
        walls.insert((140,140), Wall {});

        let scene = Scene {
            player,
            walls
        };

        Ok(scene)
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if !_repeat {
            match keycode {
                Keycode::Left => {
                },
                Keycode::Right => {
                },
                Keycode::Up => {
                },
                Keycode::Down => {
                },
                _ => ()
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
            },
            Keycode::Right => {
            },
            Keycode::Up => {
            },
            Keycode::Down => {
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for pos in self.walls.keys() {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(pos.0 as f32, pos.1 as f32, 20.0, 20.0))?;
        }

        let player = graphics::Rect::new(self.player.x as f32, self.player.y as f32, 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.x as f32 + (self.player.direction.current().x * 7.0), self.player.y as f32 + (self.player.direction.current().y * 7.0), 10.0, 10.0);
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
