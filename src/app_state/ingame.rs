use std::time::Duration;
use std::collections::HashMap;
use std::f32::consts::{PI, FRAC_PI_2};

use ggez::timer::get_delta;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, get_screen_coordinates};
use ggez::event::*;

use objects::*;
use misc::*;
use constants::*;
use world::WorldData;
use world::SpriteId;
use savegame::save_scene;
use ingame_state::*;
use GameState;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputState {
    Terminal,
    World,
    Edit,
    Inventory,
    Circuitry,
    Menu,
    Npc,
    NpcTrade,
    Storage
}

pub struct Handler {
    pub current_ingame_state: Box<GameState>
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            current_ingame_state: Box::new(world::Handler::new())
        }
    }
}

impl GameState for Handler {
    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        if data.main_menu {
            data.main_menu = false;
            save_scene(&data, "saves/auto-save.tar");
            let menu = super::menu::Handler::new().unwrap();
            Some(Box::new(menu))
        } else {
            None
        }
    }

    fn update(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {

        if let Some(state) = self.current_ingame_state.change_state(ctx, data) {
            self.current_ingame_state = state;
        }

        data.movement_timer += get_delta(ctx);

        if data.movement_timer > Duration::from_millis(MOVEMENT_DURATION) {
            if let Some(&current_movement) = data.player.movement.last() {
                if !data.check_player_collision(&current_movement) {
                    data.movement_timer = Duration::from_millis(0);
                    data.player.position = &data.player.position + &current_movement.value();
                }
            };
        }

        data.player.front_tile = &data.player.direction.value() + &data.player.position;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::LCtrl => {
                data.insight_view = true;
            },
            _ => ()
        }

        self.current_ingame_state.key_down_event(ctx, data, keycode, _keymod, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::LCtrl => {
                data.insight_view = false;
            },
            _ => ()
        }

        self.current_ingame_state.key_up_event(ctx, data, keycode, _keymod, _repeat);
    }

    fn text_input_event(&mut self, ctx: &mut Context, data: &mut WorldData, text: String) {
        self.current_ingame_state.text_input_event(ctx, data, text);
    }

    fn quit_event(&mut self, ctx: &mut Context, data: &mut WorldData) -> bool {
        self.current_ingame_state.quit_event(ctx, data);

        save_scene(data, "saves/auto-save.tar");

        false
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        data.camera = data.player.position;

        graphics::clear(ctx);

        if data.backdrop != "" {
            graphics::set_color(ctx, graphics::Color{r: 1.0, g: 1.0, b: 1.0, a: 0.25})?;
            let mut backdrop = graphics::Image::new(ctx, &data.backdrop)?;
            backdrop.set_filter(graphics::FilterMode::Nearest);

            // this is a convention for levels now (got stuck when setting up static levels via functions)
            let backdrop_pos = Position {
                x: 1,
                y: 1
            };
            let mut p = get_tile_params(ctx, backdrop_pos.to_int(), data.camera, None);
            // override with grid size scaling since backdrops are smaller scale (1 pixel = 1 tile)
            p.scale = graphics::Point2::new(GRID_SIZE as f32, GRID_SIZE as f32);
            graphics::draw_ex(
                ctx,
                &backdrop,
                p,
            )?;
        }

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, item) in data.floor.iter().enumerate() {
            if let Some(floor) = item {
                let p = get_tile_params(ctx, pos as i32, data.camera, None);
                match floor.variant {
                    FloorType::Regular => add_sprite(&mut data.sprites, SpriteId::Floor(FloorType::Regular), p),
                    FloorType::Light => add_sprite(&mut data.sprites, SpriteId::Floor(FloorType::Light), p)
                };
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Floor(FloorType::Regular))?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Floor(FloorType::Light))?;

        for (pos, item) in data.walls.iter().enumerate() {
            if let Some(wall) = item {
                let p = get_tile_params(ctx, pos as i32, data.camera, Some(wall.face));
                match wall.variant {
                    WallType::Wall => add_sprite(&mut data.sprites, SpriteId::Wall, p),
                    WallType::Corner => add_sprite(&mut data.sprites, SpriteId::Corner, p),
                    WallType::Edge => add_sprite(&mut data.sprites, SpriteId::Edge, p),
                    WallType::Window => add_sprite(&mut data.sprites, SpriteId::Window, p),
                };
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Wall)?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Corner)?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Edge)?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Window)?;

        for (pos, terminal) in data.terminals.iter().enumerate() {
            if let Some(current_terminal) = terminal {
                let p = get_tile_params(ctx, pos as i32, data.camera, Some(current_terminal.front));
                match current_terminal.variant {
                    TerminalType::Intercomm => {
                        add_sprite(&mut data.sprites, SpriteId::Terminal(TerminalType::Intercomm), p);
                    },
                    TerminalType::ShipConsole => {
                        add_sprite(&mut data.sprites, SpriteId::Terminal(TerminalType::ShipConsole), p);
                    },
                    TerminalType::Hud => ()
                };
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Terminal(TerminalType::Intercomm))?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Terminal(TerminalType::ShipConsole))?;

        for (pos, item) in data.pilot_seats.iter().enumerate() {
            if let Some(pilot_seat) = item {
                let p = get_tile_params(ctx, pos as i32, data.camera, Some(pilot_seat.front));
                add_sprite(&mut data.sprites, SpriteId::PilotSeat, p);
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::PilotSeat)?;

        for (pos, item) in data.doors.iter().enumerate() {
            if let Some(door) = item {
                let p = get_tile_params(ctx, pos as i32, data.camera, Some(door.face));
                match door.status {
                    DoorStatus::Open => add_sprite(&mut data.sprites, SpriteId::Door(DoorStatus::Open), p),
                    DoorStatus::Closed => add_sprite(&mut data.sprites, SpriteId::Door(DoorStatus::Closed), p)
                };
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Door(DoorStatus::Closed))?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Door(DoorStatus::Open))?;

        for (pos, item) in data.generators.iter().enumerate() {
            if item.is_some() {
                let params = get_tile_params(ctx, pos as i32, data.camera, None);
                add_sprite(&mut data.sprites, SpriteId::Generator, params);
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Generator)?;

        for (pos, item) in data.storages.iter().enumerate() {
            if item.is_some() {
                let params = get_tile_params(ctx, pos as i32, data.camera, None);
                add_sprite(&mut data.sprites, SpriteId::Storage, params);
            }
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::Storage)?;

        if data.insight_view {
            for (pos, item) in data.circuitry.iter().enumerate() {
                if item.is_some() {
                    let params = get_tile_params(ctx, pos as i32, data.camera, None);
                    add_sprite(&mut data.sprites, SpriteId::Circuitry, params);
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Circuitry)?;
        }

        for (pos, npc) in data.npc.iter().enumerate() {
            if let Some(npc) = npc {
                draw_tile(ctx, npc.tile(), pos as i32, data.camera, None)?;
            }
        }

        draw_tile(ctx, data.player.tile(), data.player.position.to_int(), data.camera, None)?;

        self.current_ingame_state.draw(ctx, data)?;

        graphics::present(ctx);

        Ok(())
    }
}

pub fn draw_spritebatch(ctx: &mut Context, sprites: &mut HashMap<SpriteId, SpriteBatch>, sprite_id: SpriteId) -> GameResult<()> {
    graphics::set_color(ctx, graphics::WHITE)?;
    let params = graphics::DrawParam {
        dest: graphics::Point2::new(0.0, 0.0),
        ..Default::default()
    };
    if let Some(spritebatch) = sprites.get_mut(&sprite_id) {
        graphics::draw_ex(ctx, spritebatch, params)?;
        spritebatch.clear();
    }

    Ok(())
}

pub fn get_tile_params(ctx: &mut Context, pos: i32, camera: Position, direction: Option<Direction>) -> graphics::DrawParam {
    let pos = Position {
        x: pos % LEVEL_SIZE,
        y: pos / LEVEL_SIZE
    };

    let viewport_pos = pos.viewport(camera);

    let sceen_horizontal_center = get_screen_coordinates(ctx).w / 2.0 - (GRID_SIZE / 2) as f32;
    let sceen_vertical_center = get_screen_coordinates(ctx).h / 2.0 - (GRID_SIZE / 2) as f32;
    let dst = graphics::Point2::new(viewport_pos.x as f32 + sceen_horizontal_center, viewport_pos.y as f32 + sceen_vertical_center);

    let mut tile_dst = dst;
    let rotation;
    match direction {
        Some(Direction::Up) => {
            rotation = PI;
            tile_dst = graphics::Point2::new(dst.x + GRID_SIZE as f32, dst.y + GRID_SIZE as f32);
        },
        Some(Direction::Down) => {
            rotation = 0.0;
        },
        Some(Direction::Left) => {
            rotation = FRAC_PI_2;
            tile_dst = graphics::Point2::new(tile_dst.x + GRID_SIZE as f32, tile_dst.y);
        },
        Some(Direction::Right) => {
            rotation = 3.0 * FRAC_PI_2;
            tile_dst = graphics::Point2::new(tile_dst.x, tile_dst.y + GRID_SIZE as f32);
        },
        _ => {
            rotation = 0.0;
        }
    }

    graphics::DrawParam {
        dest: tile_dst,
        rotation: rotation,
        scale: graphics::Point2::new(PIXEL_SCALE as f32, PIXEL_SCALE as f32),
        ..Default::default()
    }
}

pub fn draw_tile(ctx: &mut Context, tile_src: &str, pos: i32, camera: Position, direction: Option<Direction>) -> GameResult<()> {
		
    graphics::set_color(ctx, graphics::WHITE)?;
    let mut storage_image = graphics::Image::new(ctx, tile_src)?;
    storage_image.set_filter(graphics::FilterMode::Nearest);

    let params = get_tile_params(ctx, pos, camera, direction);
    
    graphics::draw_ex(
        ctx,
        &storage_image,
        params,
    )?;

    Ok(())
}

pub fn add_sprite(sprites: &mut HashMap<SpriteId, SpriteBatch>, sprite_id: SpriteId, params: graphics::DrawParam) {
    if let Some(spritebatch) = sprites.get_mut(&sprite_id) {
        spritebatch.add(params);
    }
}
