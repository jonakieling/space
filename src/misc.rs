use std::ops::{Add, Sub};
use constants::{GRID_SIZE, LEVEL_SIZE};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {

    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x,
            y
        }
    }
    
    pub fn viewport(self, camera: Position) -> Position {
        Position {
            x: (self.x - camera.x) * GRID_SIZE,
            y: (self.y - camera.y) * GRID_SIZE
        }
    }

    pub fn viewport_x(self, camera: Position) -> f32 {
        ((self.x - camera.x) * GRID_SIZE) as f32
    }

    pub fn viewport_y(self, camera: Position) -> f32 {
        ((self.y - camera.y) * GRID_SIZE) as f32
    }

    pub fn dist(self, other: &Position) -> f32 {
        ((other.x - self.x).pow(2) as f32 + (other.y - self.y).pow(2) as f32).sqrt()
    }

    pub fn to_int(self) -> i32 {
        self.x + self.y * LEVEL_SIZE
    }

    pub fn x_up(self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y
        }
    }

    pub fn x_down(self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y
        }
    }

    pub fn y_up(self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1
        }
    }

    pub fn y_down(self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1
        }
    }

    pub fn from_int(pos: i32) -> Position {
        Position {
            x: pos % LEVEL_SIZE,
            y: pos / LEVEL_SIZE
        }
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

impl<'a> Sub for &'a Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y
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

pub enum TextAlign {
    Left,
    Right
}