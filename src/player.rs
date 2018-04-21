use scene::{Position, Direction};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Player {
    pub position: Position,
    pub movement: Vec<Direction>,
    pub direction: Direction,
    pub front_tile: Position,
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
}