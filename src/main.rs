extern crate ggez;

use std::io::Write;
use std::time::Duration;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::event::*;

#[derive(Debug)]
struct Actor {
    x: f32,
    vel_x: f32,
    y: f32,
    vel_y: f32,
    speed_x: f32,
    speed_y: f32,
    direction: Vec<f32>
}

struct Scene {
    player: Actor,
    actors: Vec<Actor>
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {
        let player = Actor {
            x: 200.0,
            vel_x: 0.0,
            y: 200.0,
            vel_y: 0.0,
            speed_x: 2.0,
            speed_y: 2.0,
            direction: vec![0.0, 1.0]
        };

        let wall1 = Actor {
            x: 0.0,
            vel_x: 0.0,
            y: 0.0,
            vel_y: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            direction: vec![0.0, 1.0]
        };
        let wall2 = Actor {
            x: 40.0,
            y: 40.0,
            direction: vec![0.0, 1.0],
            ..wall1
        };
        let wall3 = Actor {
            x: 80.0,
            y: 80.0,
            direction: vec![0.0, 1.0],
            ..wall1
        };
        let wall4 = Actor {
            x: 120.0,
            y: 120.0,
            direction: vec![0.0, 1.0],
            ..wall1
        };
        
        let scene = Scene {
            player: player,
            actors: vec![wall1, wall2, wall3, wall4]
        };

        Ok(scene)
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {

        self.player.x += if (self.player.vel_x * self.player.direction[0]).abs() >= 0.0 { self.player.vel_x } else { 0.0 };
        self.player.y += if (self.player.vel_y * self.player.direction[1]).abs() >= 0.0 { self.player.vel_y } else { 0.0 };

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if !_repeat {
            match keycode {
                Keycode::Left => {
                    self.player.vel_x -= self.player.speed_x;
                    self.player.direction[0] = -1.0;
                    self.player.direction[1] = 0.0;
                },
                Keycode::Right => {
                    self.player.vel_x += self.player.speed_x;
                    self.player.direction[0] = 1.0;
                    self.player.direction[1] = 0.0;
                },
                Keycode::Up => {
                    self.player.vel_y -= self.player.speed_y;
                    self.player.direction[0] = 0.0;
                    self.player.direction[1] = -1.0;
                },
                Keycode::Down => {
                    self.player.vel_y += self.player.speed_y;
                    self.player.direction[0] = 0.0;
                    self.player.direction[1] = 1.0;
                },
                _ => ()
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
                self.player.vel_x += self.player.speed_x
            },
            Keycode::Right => {
                self.player.vel_x -= self.player.speed_x
            },
            Keycode::Up => {
                self.player.vel_y += self.player.speed_y
            },
            Keycode::Down => {
                self.player.vel_y -= self.player.speed_y
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        let player_x = (self.player.x / 20.0).floor() * 20.0;
        let player_y = (self.player.y / 20.0).floor() * 20.0;
        
        for actor in self.actors.iter() {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(actor.x, actor.y, 20.0, 20.0))?;
        }

        let player = graphics::Rect::new(player_x, player_y, 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(player_x + (self.player.direction[0] * 7.0), player_y + (self.player.direction[1] * 7.0), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    println!("{:?}", c);
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
