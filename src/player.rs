use ggez::GameResult;
use ggez::Context;
use ggez::graphics;

use misc::{Position, Direction};
use objects::{Item, Terminal};
use storage::SelectionStorage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub position: Position,
    pub movement: Vec<Direction>,
    pub direction: Direction,
    pub front_tile: Position,
    pub inventory: SelectionStorage<Item>,
    pub terminal: Box<Terminal>,
}

impl Player {
    pub fn movement(&mut self, direction: Direction, reverse: Direction) {
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

    pub fn remove_movement(&mut self, direction: Direction) {
        let mut remove_indicies: Vec<usize> = vec![];
        for (index, movement) in self.movement.iter().enumerate() {
            if movement == &direction {
                remove_indicies.push(index);
            }
        }
        for remove_index in remove_indicies.iter() {
            self.movement.remove(*remove_index);
        }

        if let Some(&resulting_movement) = self.movement.last() {
            self.direction = resulting_movement;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::BLACK)?;
        let player = graphics::Rect::new(self.position.viewport_x(), self.position.viewport_y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.position.viewport_x() + 5.0 + (self.direction.value().viewport_x() * 0.2), self.position.viewport_y() + 5.0 + (self.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;
        Ok(())
    }
}