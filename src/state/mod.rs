use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event::*;

use storage::{SelectionStorage, Node};
use dialog::DialogItem;
use objects::Item;
use misc::{Orientation, Position};

pub mod world;
pub mod menu;

pub trait GameState: EventHandler {
    fn change_state(&self, ctx: &mut Context) -> Option<Box<GameState>>;
}

pub struct Game {
    pub state: Box<GameState>
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(scene) = self.state.change_state(ctx) {
            self.state = scene;
        }
        self.state.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.draw(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.state.key_down_event(ctx, keycode, keymod, repeat)
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.state.key_up_event(ctx, keycode, keymod, repeat)
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        self.state.text_input_event(ctx, text)
    }

    fn quit_event(&mut self, ctx: &mut Context) -> bool {
        self.state.quit_event(ctx)
    }
}

fn draw_text(ctx: &mut Context, text: &graphics::Text) -> GameResult<()> {
    graphics::set_color(ctx, graphics::BLACK)?;
    let textbox = graphics::Rect::new(740.0 - text.width() as f32 + 20.0, 20.0, text.width() as f32 + 20.0, 20.0);
    graphics::rectangle(ctx, graphics::DrawMode::Fill, textbox)?;
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), textbox)?;
    graphics::draw(ctx, text, graphics::Point2::new(750.0 - text.width() as f32 + 20.0, 20.0), 0.0)?;

    Ok(())
}

fn draw_dialog(dialog: &Node<DialogItem>, ctx: &mut Context) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let text = graphics::Text::new(ctx, &dialog.value.response, &font)?;

    graphics::set_color(ctx, graphics::BLACK)?;
    let textbox = graphics::Rect::new(300.0, 400.0, text.width() as f32 + 20.0, 20.0);
    graphics::rectangle(ctx, graphics::DrawMode::Fill, textbox)?;
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::draw(ctx, &text, graphics::Point2::new(310.0, 400.0), 0.0)?;
    
    draw_selection_with_parameters(&dialog.children, ctx, Position { x: 300, y: 430 }, Orientation::Right, true)?;

    Ok(())
}

fn draw_selection<T: Clone + ToString>(selection: &SelectionStorage<T>, ctx: &mut Context, cursor: bool) -> GameResult<()> {
    draw_selection_with_parameters(&selection, ctx, Position { x: 760, y: 20 }, Orientation::Left, cursor)?;

    Ok(())
}

fn draw_input_state(state: &str, ctx: &mut Context) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let input_state_text = String::from(state);
    let input_state_graphics = graphics::Text::new(ctx, &input_state_text, &font).unwrap();
    graphics::set_color(ctx, graphics::BLACK)?;
    let input_state_box = graphics::Rect::new(20.0, 20.0, input_state_graphics.width() as f32 + 20.0, 20.0);
    graphics::rectangle(ctx, graphics::DrawMode::Fill, input_state_box)?;
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), input_state_box)?;
    graphics::draw(ctx, &input_state_graphics, graphics::Point2::new(30.0, 20.0), 0.0)?;

    Ok(())
}

pub enum TradeArea {
    Left,
    Right
}

fn draw_trade_area(source: &SelectionStorage<Item>, target: &SelectionStorage<Item>, ctx: &mut Context, area: TradeArea) -> GameResult<()> {
    match area {
        TradeArea::Left => {
            draw_selection_with_parameters(&source, ctx, Position { x: 60, y: 50 }, Orientation::Right, false)?;
            draw_selection_with_parameters(&target, ctx, Position { x: 260, y: 50 }, Orientation::Left, false)?;
        },
        TradeArea::Right => {
            draw_selection_with_parameters(&target, ctx, Position { x: 400, y: 50 }, Orientation::Left, false)?;
            draw_selection_with_parameters(&source, ctx, Position { x: 600, y: 50 }, Orientation::Right, false)?;
        },
    }

    Ok(())
}

fn draw_selection_with_parameters<T: Clone + ToString>(selection: &SelectionStorage<T>, ctx: &mut Context, position: Position, orientation: Orientation, cursor: bool) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let mut inventory_item_position = 0.0;
    let current_item = selection.current_index();
    for (pos, item) in selection.iter().enumerate() {
        let item_text = item.to_string();
        let item_graphics = graphics::Text::new(ctx, &item_text, &font).unwrap();
        let mut offset;
        match orientation {
            Orientation::Left => {
                offset = item_graphics.width() as f32;
            },
            Orientation::Right => {
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