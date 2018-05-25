use std::time::Duration;

use ggez::timer::get_delta;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event::*;

use app::*;
use objects::*;
use misc::*;
use constants::*;
use world::WorldData;
use feature::{*, map::MapFeature};

pub trait GameState {
    fn change_state(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> Option<Box<GameState>> { None }
    
    fn update(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> GameResult<()> { Ok(()) }

    fn draw(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> GameResult<()> { Ok(()) }

    fn key_down_event(&mut self, _ctx: &mut Context, _world: &mut WorldData, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn key_up_event(&mut self, _ctx: &mut Context, _world: &mut WorldData, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn text_input_event(&mut self, _ctx: &mut Context, _world: &mut WorldData, _text: String) { }

    fn quit_event(&mut self, _ctx: &mut Context, _world: &mut WorldData) -> bool { false }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputState {
    Terminal,
    World,
    Edit,
    Inventory,
    Circuitry,
    Menu,
    Mainmenu,
    Npc,
    NpcTrade,
    Storage,
    Map(MapFeature)
}

pub struct Handler {
    pub current_ingame_state: Box<GameState>
}

impl Handler {
    pub fn new(data: &mut WorldData) -> Handler {
        Handler {
            current_ingame_state: Box::new(mainmenu::Handler::new(data))
        }
    }
}

impl GameState for Handler {
    fn update(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {

        if let Some(state) = self.current_ingame_state.change_state(ctx, data) {
            self.current_ingame_state = state;
        }

        data.movement_timer += get_delta(ctx);

        if data.movement_timer > Duration::from_millis(MOVEMENT_DURATION) {
            if let Some(&current_movement) = data.level.player.movement.last() {
                if !data.level.check_player_collision(&current_movement) {
                    data.movement_timer = Duration::from_millis(0);
                    data.level.player.position = &data.level.player.position + &current_movement.value();
                }
            };
        }

        data.level.player.front_tile = &data.level.player.direction.value() + &data.level.player.position;

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

        false
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        graphics::clear(ctx);

        if !data.overlay {
            data.camera = data.level.player.position;
            if data.level.backdrop != "" {
                graphics::set_color(ctx, graphics::Color{r: 1.0, g: 1.0, b: 1.0, a: 0.25})?;
                let mut backdrop = graphics::Image::new(ctx, &data.level.backdrop)?;
                backdrop.set_filter(graphics::FilterMode::Nearest);

                // this is a convention for levels now (got stuck when setting up static levels via functions)
                let backdrop_pos = Position {
                    x: 1,
                    y: 1
                };
                let mut p = get_tile_params(ctx, backdrop_pos, data.camera, None);
                // override with grid size scaling since backdrops are smaller scale (1 pixel = 1 tile)
                p.scale = graphics::Point2::new(GRID_SIZE as f32, GRID_SIZE as f32);
                graphics::draw_ex(
                    ctx,
                    &backdrop,
                    p,
                )?;
            }

            graphics::set_color(ctx, graphics::BLACK)?;

            for (pos, item) in data.level.floor.iter().enumerate() {
                if let Some(floor) = item {
                    let p = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, None);
                    match floor.variant {
                        FloorType::Regular => add_sprite(&mut data.sprites, SpriteId::Floor(FloorType::Regular), p),
                        FloorType::Light => add_sprite(&mut data.sprites, SpriteId::Floor(FloorType::Light), p)
                    };
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Floor(FloorType::Regular))?;
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Floor(FloorType::Light))?;

            for (pos, item) in data.level.walls.iter().enumerate() {
                if let Some(wall) = item {
                    let p = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, Some(wall.face));
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

            for (pos, terminal) in data.level.terminals.iter().enumerate() {
                if let Some(current_terminal) = terminal {
                    let p = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, Some(current_terminal.front));
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

            for (pos, item) in data.level.pilot_seats.iter().enumerate() {
                if let Some(pilot_seat) = item {
                    let p = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, Some(pilot_seat.front));
                    add_sprite(&mut data.sprites, SpriteId::PilotSeat, p);
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::PilotSeat)?;

            for (pos, item) in data.level.doors.iter().enumerate() {
                if let Some(door) = item {
                    let p = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, Some(door.face));
                    add_sprite(&mut data.sprites, SpriteId::Door(door.status), p);
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Door(DoorStatus::Closed))?;
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Door(DoorStatus::Open))?;

            for (pos, item) in data.level.generators.iter().enumerate() {
                if item.is_some() {
                    let params = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, None);
                    add_sprite(&mut data.sprites, SpriteId::Generator, params);
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Generator)?;

            for (pos, item) in data.level.storages.iter().enumerate() {
                if item.is_some() {
                    let params = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, None);
                    add_sprite(&mut data.sprites, SpriteId::Storage, params);
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Storage)?;

            for (pos, item) in data.level.decorations.iter().enumerate() {
                if let Some(deco) = item {
                    let p = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, Some(deco.face));
                    add_sprite(&mut data.sprites, SpriteId::Decoration(deco.variant), p);
                }
            }
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Decoration(DecorationType::Display))?;
            draw_spritebatch(ctx, &mut data.sprites, SpriteId::Decoration(DecorationType::Panel))?;

            if data.insight_view {
                for (pos, item) in data.level.circuitry.iter().enumerate() {
                    if let Some(circuitry) = item {
                        let params = get_tile_params(ctx, Position::from_int(pos as i32), data.camera, None);
                        add_sprite(&mut data.sprites, SpriteId::Circuitry(circuitry.variant.clone()), params);
                    }
                }
                draw_spritebatch(ctx, &mut data.sprites, SpriteId::Circuitry(CircuitryType::Powered))?;
                draw_spritebatch(ctx, &mut data.sprites, SpriteId::Circuitry(CircuitryType::Inactive))?;
            }

            for (pos, npc) in data.level.npc.iter().enumerate() {
                if let Some(npc) = npc {
                    draw_tile(ctx, npc.tile(), pos as i32, data.camera, None)?;
                }
            }

            draw_tile(ctx, data.level.player.tile(), data.level.player.position.to_int(), data.camera, None)?;
        }

        self.current_ingame_state.draw(ctx, data)?;

        graphics::present(ctx);

        Ok(())
    }
}
