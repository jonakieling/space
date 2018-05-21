use std::collections::HashMap;
use std::f32::consts::{PI, FRAC_PI_2};

use ggez::graphics::{spritebatch::SpriteBatch, get_screen_coordinates};
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event::*;

use storage::{SelectionStorage, Node};
use dialog::DialogItem;
use misc::{TextAlign, Position, Direction};
use world::WorldData;
use super::GameState;
use objects::*;
use constants::*;

pub struct AppContainer {
    pub state: Box<GameState>,
    pub world: WorldData
}

impl EventHandler for AppContainer {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(scene) = self.state.change_state(ctx, &mut self.world) {
            self.state = scene;
        }
        self.state.update(ctx, &mut self.world)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.draw(ctx, &mut self.world)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.state.key_down_event(ctx, &mut self.world, keycode, keymod, repeat)
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.state.key_up_event(ctx, &mut self.world, keycode, keymod, repeat)
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        self.state.text_input_event(ctx, &mut self.world, text)
    }

    fn quit_event(&mut self, ctx: &mut Context) -> bool {
        self.state.quit_event(ctx, &mut self.world)
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum SpriteId {
    Wall,
    Corner,
    Edge,
    Window,
    Floor(FloorType),
    Circuitry,
    Door(DoorStatus),
    Terminal(TerminalType),
    PilotSeat,
    Storage,
    Generator,
    Decoration(DecorationType),
    MapSector,
    MapStation
}

pub fn draw_dialog(dialog: &Node<DialogItem>, ctx: &mut Context) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let text = graphics::Text::new(ctx, &dialog.value.response, &font)?;

    graphics::set_color(ctx, graphics::BLACK)?;
    let textbox = graphics::Rect::new(300.0, 400.0, text.width() as f32 + 20.0, 20.0);
    graphics::rectangle(ctx, graphics::DrawMode::Fill, textbox)?;
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::draw(ctx, &text, graphics::Point2::new(310.0, 400.0), 0.0)?;
    
    draw_selection_with_parameters(&dialog.children, ctx, Position { x: 300, y: 430 }, TextAlign::Right, true, false)?;

    Ok(())
}

pub fn draw_selection<T: Clone + ToString>(selection: &SelectionStorage<T>, ctx: &mut Context, cursor: bool, draw_empty: bool) -> GameResult<()> {
    draw_selection_with_parameters(&selection, ctx, Position { x: 760, y: 20 }, TextAlign::Left, cursor, draw_empty)?;

    Ok(())
}

pub fn draw_input_state(state: &str, ctx: &mut Context) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let input_state_text = String::from(state);
    let input_state_graphics = graphics::Text::new(ctx, &input_state_text, &font).unwrap();
    graphics::set_color(ctx, graphics::BLACK)?;
    let input_state_box = graphics::Rect::new(20.0, 20.0, input_state_graphics.width() as f32 + 20.0, 20.0);
    graphics::rectangle(ctx, graphics::DrawMode::Fill, input_state_box)?;
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::draw(ctx, &input_state_graphics, graphics::Point2::new(30.0, 20.0), 0.0)?;

    Ok(())
}

pub fn draw_selection_with_parameters<T: Clone + ToString>(selection: &SelectionStorage<T>, ctx: &mut Context, position: Position, orientation: TextAlign, cursor: bool, draw_empty: bool) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let mut inventory_item_position = 0.0;
    let current_item = selection.current_index();

    if draw_empty && selection.iter().len() == 0 {
        let empty_text = graphics::Text::new(ctx, "empty", &font).unwrap();
        let offset;
        match orientation {
            TextAlign::Left => {
                offset = empty_text.width() as f32;
            },
            TextAlign::Right => {
                offset = 0.0;
            },
        }
        let empty_text_box = graphics::Rect::new(position.x as f32 - offset, position.y as f32, empty_text.width() as f32 + 20.0, 20.0);

        graphics::set_color(ctx, graphics::WHITE)?;
        graphics::draw(ctx, &empty_text, graphics::Point2::new(position.x as f32 + 11.0 - offset, position.y as f32), 0.0)?;
        if cursor {
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), empty_text_box)?;
        }
    }

    for (pos, item) in selection.iter().enumerate() {
        let item_text = item.to_string();
        let item_graphics = graphics::Text::new(ctx, &item_text, &font).unwrap();
        let mut offset;
        match orientation {
            TextAlign::Left => {
                offset = item_graphics.width() as f32;
            },
            TextAlign::Right => {
                offset = 0.0;
            },
        }
        let inventory_box = graphics::Rect::new(position.x as f32 - offset, position.y as f32 + (inventory_item_position * 25.0), item_graphics.width() as f32 + 20.0, 20.0);
        graphics::set_color(ctx, graphics::BLACK)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, inventory_box)?;
        graphics::set_color(ctx, graphics::WHITE)?;
        if pos == current_item && cursor {
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), inventory_box)?;
        }
        graphics::draw(ctx, &item_graphics, graphics::Point2::new(position.x as f32 + 11.0 - offset, position.y as f32 + (inventory_item_position * 25.0)), 0.0)?;
        inventory_item_position += 1.0;
    }

    Ok(())
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

pub fn get_tile_params(ctx: &mut Context, pos: Position, camera: Position, direction: Option<Direction>) -> graphics::DrawParam {

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

    let params = get_tile_params(ctx, Position::from_int(pos), camera, direction);
    
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
