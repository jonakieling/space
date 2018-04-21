extern crate ggez;
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tar;

use tar::{Builder, Archive};

use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::ops::Add;
use std::slice;

use ggez::conf;
use ggez::event;
use ggez::timer;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event::*;

// pixel scaling
const GRID_SIZE: i32 = 20;
// delay of movement in miliseconds
const MOVEMENT_SPEED: u64 = 290;
// width and height of a level in number of tiles
const LEVEL_SIZE: i32 = 40;
const TERMINAL_LIMIT: usize = 20;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Terminal {
    text: Box<String>
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
enum InputState {
    Terminal,
    World
}

struct Scene {
    movement_timer: Duration,
    player: Player,
    player_front_tile: Position,
    walls: PositionLevelStorage<Wall>,
    doors: PositionLevelStorage<Door>,
    terminals: PositionLevelStorage<Terminal>,
    terminal_text: graphics::Text,
    input: InputState,
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {

        let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
        
        // initialize player and level object storages
        // state and object can be loaded seperatly

        let player = Player {
            position: Position { x: 10, y: 10 },
            movement: vec![],
            direction: Direction::Down
        };
        let player_front_tile = &player.direction.value() + &player.position;

        let walls = <PositionLevelStorage<Wall>>::new();
        let doors = <PositionLevelStorage<Door>>::new();
        let mut terminals = <PositionLevelStorage<Terminal>>::new();
        terminals.insert(20, 20, Terminal { text: Box::new(String::new()) });
        terminals.insert(22, 20, Terminal { text: Box::new(String::new()) });
        
        let scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            player_front_tile,
            walls,
            doors,
            terminals,
            terminal_text: graphics::Text::new(_ctx, "", &font)?,
            input: InputState::World,
        };

        Ok(scene)
    }

    fn check_player_collision(&self) -> bool {
        let mut found_collision = false;

        if let Some(&Some(_)) = self.walls.get(self.player_front_tile.x, self.player_front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(_)) = self.terminals.get(self.player_front_tile.x, self.player_front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(ref door)) = self.doors.get(self.player_front_tile.x, self.player_front_tile.y) {
            if let DoorStatus::Closed = door.status {
                found_collision = true;
            }
        }

        found_collision
    }

    fn interact_with_door(&mut self) {
        if let Some(&mut Some(ref mut door)) = self.doors.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
            match door.status {
                DoorStatus::Closed => {
                    door.status = DoorStatus::Open;
                    println!("door opened");
                },
                DoorStatus::Open => {
                    door.status = DoorStatus::Closed;
                    println!("door closed");
                },
            }
        }
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.movement_timer += timer::get_delta(_ctx);

        if self.movement_timer > Duration::from_millis(MOVEMENT_SPEED) {
            self.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.player.movement.last() {
                if !self.check_player_collision() {
                    self.player.position = &self.player.position + &current_movement.value();
                }
            };
        }

        self.player_front_tile = &self.player.direction.value() + &self.player.position;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if self.input == InputState::World {
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
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match self.input {
            InputState::World => {
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
                    Keycode::Return => {
                        self.interact_with_door();

                        // interact_with_terminal
                        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
                            self.input = InputState::Terminal;
                            
                            let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                            self.terminal_text = graphics::Text::new(_ctx, &current_terminal.text, &font).unwrap();
                        }
                    },
                    _ => ()
                }
            },
            InputState::Terminal => {
                match keycode {
                    Keycode::Backspace => {
                        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
                            if current_terminal.text.len() > 0 {
                                let text_len = current_terminal.text.len();
                                current_terminal.text.split_off(text_len - 1);

                                let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                                self.terminal_text = graphics::Text::new(_ctx, &current_terminal.text, &font).unwrap();
                            }
                        }
                    },
                    Keycode::Escape => {
                        let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                        self.terminal_text = graphics::Text::new(_ctx, "", &font).unwrap();
                        self.input = InputState::World;
                    },
                    _ => ()
                }
            },
        }
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _text: String) {
        if self.input == InputState::Terminal {

            if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player_front_tile.x, self.player_front_tile.y) {
                if current_terminal.text.len() <= TERMINAL_LIMIT {
                    let new_terminal_text = format!("{}{}", current_terminal.text, _text);
                    current_terminal.text = Box::new(new_terminal_text);

                    let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
                    self.terminal_text = graphics::Text::new(_ctx, &current_terminal.text, &font).unwrap();
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, wall) in self.walls.iter().enumerate() {
            // Match for entity presence
            if let &Some(_) = wall {
                let x = pos as i32 % LEVEL_SIZE;
                let y = pos as i32 / LEVEL_SIZE;
                graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
            }
        }

        for (pos, terminal) in self.terminals.iter().enumerate() {
            // Match for entity presence
            if let &Some(_) = terminal {
                let x = pos as i32 % LEVEL_SIZE;
                let y = pos as i32 / LEVEL_SIZE;
                graphics::set_color(ctx, graphics::BLACK)?;
                graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
                graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
                graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
            }
        }

        for (pos, door_pos) in self.doors.iter().enumerate() {
            // Match for entity presence
            if let &Some(ref door) = door_pos {
                let x = pos as i32 % LEVEL_SIZE;
                let y = pos as i32 / LEVEL_SIZE;
                match door.status {
                    DoorStatus::Open => {
                        graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
                        graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
                    },
                    DoorStatus::Closed => {
                        graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
                        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
                    },
                }
            }
        }

        graphics::set_color(ctx, graphics::BLACK)?;

        let player = graphics::Rect::new(self.player.position.viewport_x(), self.player.position.viewport_y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.viewport_x() + 5.0 + (self.player.direction.value().viewport_x() * 0.2), self.player.position.viewport_y() + 5.0 + (self.player.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        graphics::draw(ctx, &self.terminal_text, graphics::Point2::new(320.0, 500.0), 0.0)?;

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Space", "Jonathan Kieling", c).unwrap();
    let scene = &mut Scene::new(ctx).unwrap();

    load_scene(scene);

    if let Err(e) = event::run(ctx, scene) {
        writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
        std::process::exit(1);
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

    let mut level_doors: Vec<(i32, i32, Door)> = vec![];
    for (pos, item) in scene.doors.iter().enumerate() {
        if let Some(ref door) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_doors.push((x, y, *door.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_doors).unwrap();
    File::create("level0/doors.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_terminals: Vec<(i32, i32, Terminal)> = vec![];
    for (pos, item) in scene.terminals.iter().enumerate() {
        if let Some(ref terminal) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_terminals.push((x, y, *terminal.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_terminals).unwrap();
    File::create("level0/terminals.bin").unwrap().write_all(&bytes).unwrap();

    let file = File::create("level0.tar").unwrap();
    let mut a = Builder::new(file);
    a.append_dir_all("level0", "level0").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("level0").unwrap();
    println!("saved game: level0");
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
            "terminals" => {
                let mut level_terminals: Vec<(i32, i32, Terminal)> = bincode::deserialize_from(file).unwrap();
                for terminal in level_terminals {
                    scene.terminals.insert(terminal.0, terminal.1, terminal.2);
                }
            },
            "player" => {
                let mut level_player: Player = bincode::deserialize_from(file).unwrap();
                scene.player = level_player;
            },
            _ => (),
        }
    }
    println!("game loaded: level0");
}
