use ggez::GameResult;
use ggez::Context;
use ggez::graphics;

use misc::{Position, Direction};
use objects::{Item, Terminal, draw_tile};
use storage::SelectionStorage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LogEntry {
	pub title: String,
	pub message: String
}

impl ToString for LogEntry {
    fn to_string(&self) -> String {
        self.title.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub position: Position,
    pub movement: Vec<Direction>,
    pub direction: Direction,
    pub front_tile: Position,
    pub inventory: SelectionStorage<Item>,
    pub terminal: Box<Terminal>,
    pub log:SelectionStorage<LogEntry>
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
		let image_src;
		match self.direction {
			Direction::Up => {
				image_src = "/char-back.png";
			},
			Direction::Down => {
				image_src = "/char-front.png";
			},
			Direction::Left => {
				image_src = "/char-left.png";
			},
			Direction::Right => {
				image_src = "/char-right.png";
			}
		}

		let dst = graphics::Point2::new(self.position.viewport_x(), self.position.viewport_y());
        draw_tile(ctx, image_src, dst, None)
    }
}