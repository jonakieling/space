use std::fmt::Debug;

use ggez::GameResult;
use ggez::Context;
use ggez::graphics;

use storage::SelectionStorage;

pub mod world;
pub mod menu;

fn draw_selection<T: Clone + Debug>(selection: &SelectionStorage<T>, ctx: &mut Context, cursor: bool) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let mut inventory_item_position = 0.0;
    let current_item = selection.current_index();
    for (pos, item) in selection.iter().enumerate() {
        let item_text = format!("{:?}", item);
        let item_graphics = graphics::Text::new(ctx, &item_text, &font).unwrap();
        let inventory_box = graphics::Rect::new(760.0 - (item_graphics.width() as f32), 20.0 + (inventory_item_position * 25.0), item_graphics.width() as f32 + 20.0, 20.0);
        graphics::set_color(ctx, graphics::BLACK)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, inventory_box)?;
        graphics::set_color(ctx, graphics::WHITE)?;
        if pos == current_item && cursor {
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), inventory_box)?;
        }
        graphics::draw(ctx, &item_graphics, graphics::Point2::new(771.0 - item_graphics.width() as f32, 20.0 + (inventory_item_position * 25.0)), 0.0)?;
        inventory_item_position += 1.0;
    }

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