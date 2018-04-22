use std::ops::Add;
use constants::GRID_SIZE;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn viewport_x(self) -> f32 {
        (self.x * GRID_SIZE) as f32
    }

    pub fn viewport_y(self) -> f32 {
        (self.y * GRID_SIZE) as f32
    }
}

impl<'a> Add for &'a Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn value(&self) -> Position {
        match *self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum InputState {
    Terminal,
    World,
    Edit
}