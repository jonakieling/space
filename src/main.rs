extern crate ggez;

use std::io::Write;
use std::time::Duration;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::*;
use ggez::event::*;

#[derive(Debug)]
struct Player {
    x: f32,
    vel_x: f32,
    y: f32,
    vel_y: f32,
    speed: f32
}

struct Scene {
    player: Player
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {

        Ok(Scene {
            player: Player {
                x: 0.0,
                vel_x: 0.0,
                y: 0.0,
                vel_y: 0.0,
                speed: 5.0
            }
        })
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {

        self.player.x += self.player.vel_x;
        self.player.y += self.player.vel_y;

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::Up => self.player.vel_y = -self.player.speed,
            Keycode::Down => self.player.vel_y = self.player.speed,
            Keycode::Left => self.player.vel_x = -self.player.speed,
            Keycode::Right => self.player.vel_x = self.player.speed,
            _ => ()
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::Up => self.player.vel_y = 0.0,
            Keycode::Down => self.player.vel_y = 0.0,
            Keycode::Left => self.player.vel_x = 0.0,
            Keycode::Right => self.player.vel_x = 0.0,
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        let player_x = (self.player.x / 20.0).round() * 20.0;
        let player_y = (self.player.y / 20.0).round() * 20.0;
        let rect = graphics::Rect::new(player_x, player_y, 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;

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
