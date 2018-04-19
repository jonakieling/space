extern crate ggez;
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tar;

use tar::{Builder, Archive};

use std::fs;
use std::fs::File;
use std::io::{Write};
use std::time::Duration;
use std::ops::Add;
use std::slice;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::event::*;

// pixel scaling
const GRID_SIZE: i32 = 20;
// delay of movement in miliseconds
const MOVEMENT_SPEED: u64 = 290;
// width and height of a level in number of tiles
const LEVEL_SIZE: i32 = 40;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn viewport_x(self) -> f32 {
        (self.x * GRID_SIZE) as f32
    }

    fn viewport_y(self) -> f32 {
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
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn value(&self) -> Position {
        match *self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Player {
    position: Position,
    movement: Vec<Direction>,
    direction: Direction
}

impl Player {
    fn movement(&mut self, direction: Direction, reverse: Direction) {
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

    fn remove_movement(&mut self, direction: Direction) {
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

#[derive(Clone)]
struct PositionLevelStorage<T: Clone> {
    storage: Vec<Option<Box<T>>>
}

impl<T: Clone> PositionLevelStorage<T> {
    fn new() -> PositionLevelStorage<T> {
        PositionLevelStorage {
            storage: vec![None; (LEVEL_SIZE * LEVEL_SIZE) as usize]
        }
    }
    fn get(&self, x: i32, y: i32) -> Option<&Option<Box<T>>> {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
        let position = x + y * LEVEL_SIZE;
            match self.storage.get(position as usize) {
                Some(item) => Some(item),
                None => None
            }
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Option<Box<T>>> {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
        let position = x + y * LEVEL_SIZE;
            match self.storage.get_mut(position as usize) {
                Some(item) => Some(item),
                None => None
            }
        } else {
            None
        }
    }

    fn insert(&mut self, x: i32, y: i32, item: T) {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
            let position = x + y * LEVEL_SIZE;
            self.storage.remove(position as usize);
            self.storage.insert(position as usize, Some(Box::new(item)));
        }
    }

    fn iter(&self) -> slice::Iter<Option<Box<T>>> {
        self.storage.iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
struct Wall {
    
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
enum DoorStatus {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
struct Door {
    status: DoorStatus
}

struct Scene {
    movement_timer: Duration,
    player: Player,
    walls: PositionLevelStorage<Wall>,
    doors: PositionLevelStorage<Door>
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {

        // initialize player and level object storages
        // state and object can be loaded seperatly

        let player = Player {
            position: Position { x: 10, y: 10 },
            movement: vec![],
            direction: Direction::Down
        };

        let walls = <PositionLevelStorage<Wall>>::new();
        let doors = <PositionLevelStorage<Door>>::new();
        let scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            walls,
            doors
        };

        Ok(scene)
    }

    fn check_player_collision(&self, direction: Direction) -> bool {
        let mut found_collision = false;
        let position = &direction.value() + &self.player.position;
        match self.walls.get(position.x, position.y) {
            // Match for Vec access
            Some(item) => {
                // Match for entity presence
                match *item {
                    None => (),
                    _ => {
                        found_collision = true;
                    }
                }
                
            },
            None => ()
        }

        match self.doors.get(position.x, position.y) {
            // Match for Vec access
            Some(item) => {
                // Match for entity presence
                match *item {
                    None => (),
                    Some(ref door) => {
                        match door.status {
                            DoorStatus::Closed => {
                                found_collision = true;
                            },
                            DoorStatus::Open => (),
                        }
                    }
                }
                
            },
            None => ()
        }

        found_collision
    }

    fn interact_with_door(&mut self) {
        let position = &self.player.direction.value() + &self.player.position;

        // Match for Vec access
        if let Some(item) = self.doors.get_mut(position.x, position.y) {
            // Match for entity presence
            if let &mut Some(ref mut door) = item {
                    match door.status {
                        DoorStatus::Closed => {
                            door.status = DoorStatus::Open;
                        },
                        DoorStatus::Open => {
                            door.status = DoorStatus::Closed;
                        },
                    }
            }
        }
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.movement_timer += _dt;

        if self.movement_timer > Duration::from_millis(MOVEMENT_SPEED) {
            self.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.player.movement.last() {
                if !self.check_player_collision(current_movement) {
                    self.player.position = &self.player.position + &current_movement.value();
                }
            };
        }

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if !_repeat {
            self.movement_timer = Duration::from_millis(MOVEMENT_SPEED);

            match keycode {
                Keycode::Left => {
                    self.player.movement(Direction::Left, Direction::Right);
                },
                Keycode::Right => {
                    self.player.movement(Direction::Right, Direction::Left);
                },
                Keycode::Up => {
                    self.player.movement(Direction::Up, Direction::Down);
                },
                Keycode::Down => {
                    self.player.movement(Direction::Down, Direction::Up);
                },
                _ => ()
            }
        } else {
            if let None = self.player.movement.last() {
                match keycode {
                    Keycode::Left => {
                        self.player.movement(Direction::Left, Direction::Right);
                    },
                    Keycode::Right => {
                        self.player.movement(Direction::Right, Direction::Left);
                    },
                    Keycode::Up => {
                        self.player.movement(Direction::Up, Direction::Down);
                    },
                    Keycode::Down => {
                        self.player.movement(Direction::Down, Direction::Up);
                    },
                    _ => ()
                }
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
                self.player.remove_movement(Direction::Left);
            },
            Keycode::Right => {
                self.player.remove_movement(Direction::Right);
            },
            Keycode::Up => {
                self.player.remove_movement(Direction::Up);
            },
            Keycode::Down => {
                self.player.remove_movement(Direction::Down);
            },
            Keycode::E => {
                self.interact_with_door();
            },
            _ => ()
        }


    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, wall) in self.walls.iter().enumerate() {
            // Match for entity presence
            match *wall {
                Some(_) => {
                    let x = pos as i32 % LEVEL_SIZE;
                    let y = pos as i32 / LEVEL_SIZE;
                    graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
                },
                None => (),
            }
        }

        for (pos, door_pos) in self.doors.iter().enumerate() {
            // Match for entity presence
            match *door_pos {
                Some(ref door) => {
                    let x = pos as i32 % LEVEL_SIZE;
                    let y = pos as i32 / LEVEL_SIZE;
                    match door.status {
                        DoorStatus::Open => {
                            graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
                            graphics::rectangle(ctx, graphics::DrawMode::Line, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
                        },
                        DoorStatus::Closed => {
                            graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
                            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
                        },
                    }
                },
                None => (),
            }
        }

        graphics::set_color(ctx, graphics::BLACK)?;

        let player = graphics::Rect::new(self.player.position.viewport_x(), self.player.position.viewport_y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.viewport_x() + (self.player.direction.value().viewport_x() * 0.2), self.player.position.viewport_y() + (self.player.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();
    let scene = &mut Scene::new(ctx).unwrap();

    load_scene(scene);

    match event::run(ctx, scene) {
        Ok(()) => (),
        Err(e) => {
            writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
            std::process::exit(1);
        }
    }

    save_scene(scene);
}

// https://github.com/alexcrichton/tar-rs
fn save_scene(scene: &Scene) {
    fs::create_dir("level0").unwrap();

    let bytes: Vec<u8> = bincode::serialize(&scene.player).unwrap();
    File::create("level0/player.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_walls: Vec<(i32, i32, Wall)> = vec![];
    for (pos, item) in scene.walls.iter().enumerate() {
        if let Some(ref wall) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_walls.push((x, y, *wall.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_walls).unwrap();
    File::create("level0/walls.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_doos: Vec<(i32, i32, Door)> = vec![];
    for (pos, item) in scene.doors.iter().enumerate() {
        if let Some(ref door) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_doos.push((x, y, *door.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_doos).unwrap();
    File::create("level0/doors.bin").unwrap().write_all(&bytes).unwrap();

    let file = File::create("level0.tar").unwrap();
    let mut a = Builder::new(file);
    a.append_dir_all("level0", "level0").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("level0").unwrap();
}

// https://github.com/alexcrichton/tar-rs
fn load_scene(scene: &mut Scene) {
    let file = File::open("level0.tar").unwrap();
    let mut a = Archive::new(file);

    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        match file.path().unwrap().file_stem().unwrap().to_str().unwrap() as &str {
            "walls" => {
                let mut level_walls: Vec<(i32, i32, Wall)> = bincode::deserialize_from(file).unwrap();
                for wall in level_walls {
                    scene.walls.insert(wall.0, wall.1, wall.2);
                }
            },
            "doors" => {
                let mut level_doors: Vec<(i32, i32, Door)> = bincode::deserialize_from(file).unwrap();
                for door in level_doors {
                    scene.doors.insert(door.0, door.1, door.2);
                }
            },
            "player" => {
                let mut level_player: Player = bincode::deserialize_from(file).unwrap();
                scene.player = level_player;
            }
            _ => (),
        }
    }
}
