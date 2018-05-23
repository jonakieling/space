use std::fs;
use std::fs::File;
use std::io::Write;

use tar::{Builder, Archive};
use bincode;

use world::WorldData;
use player::Player;
use objects::*;
use misc::{Position, Direction};
use storage::{Node, SelectionStorage};

pub mod static_levels;
pub mod static_npc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Save {
    location: Location,
    backdrop: String,
    offset: Position
}

pub fn save_scene(world: &WorldData, filename: &str) {
    fs::create_dir("temp-save").unwrap();

    let save_info = Save { location: world.level.location.clone(), backdrop: world.level.backdrop.clone(), offset: Position { x: 0, y: 0} };
    let bytes: Vec<u8> = bincode::serialize(&save_info).unwrap();
    File::create("temp-save/save-meta.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&world.level).unwrap();
    File::create("temp-save/level.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&world.universe).unwrap();
    File::create("temp-save/universe.bin").unwrap().write_all(&bytes).unwrap();

    let file = File::create(filename).unwrap();
    let mut a = Builder::new(file);
    a.append_dir_all("save", "temp-save").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("temp-save").unwrap();
    println!("saved game: {}", filename);
}

pub fn load_scene(world: &mut WorldData, filename: &str) {
    if let Ok(file) = File::open(filename) {

        world.level.clear();

        let mut a = Archive::new(file);

        for file in a.entries().unwrap() {
            // Make sure there wasn't an I/O error
            let file = file.unwrap();

            match file.path().unwrap().file_stem().unwrap().to_str().unwrap() as &str {
                "level" => {
                    world.level = bincode::deserialize_from(file).unwrap();
                },
                "universe" => {
                    world.universe = bincode::deserialize_from(file).unwrap();
                },
                "save-meta" => {
                    let level_info: Save = bincode::deserialize_from(file).unwrap();
                    world.level.backdrop = level_info.backdrop;
                    world.level.location = level_info.location;
                },
                _ => (),
            }

            world.level.update_power();
        }
        println!("game loaded: from file {}", filename);
    } else {
        static_levels::empty(world);
    }
    
}

pub fn insert_walls(world: &mut WorldData, walls: Vec<(i32, i32, WallType, Direction)>) {
    for wall in walls {
        world.level.walls.insert(
            Position { x: wall.0, y: wall.1 },
            Wall { variant: wall.2, face: wall.3 }
        );
    }
}

pub fn insert_floor(world: &mut WorldData, floor: Vec<(i32, i32, FloorType)>) {
    for tile in floor {
        world.level.floor.insert(
            Position { x: tile.0, y: tile.1 },
            Floor { variant: tile.2 }
        );
    }
}

pub fn insert_generator(world: &mut WorldData, generators: Vec<(i32, i32, Direction)>) {
    for generator in generators {
        world.level.generators.insert(
            Position { x: generator.0, y: generator.1 },
            Generator { face: generator.2 }
        );
    }
}

pub fn insert_pilot_seat(world: &mut WorldData, pilot_seats: Vec<(i32, i32, Direction)>) {
    for pilot_seat in pilot_seats {
        world.level.pilot_seats.insert(
            Position { x: pilot_seat.0, y: pilot_seat.1 },
            PilotSeat { front: pilot_seat.2 }
        );
    }
}

pub fn insert_circuitry(world: &mut WorldData, circuitry: Vec<(i32, i32)>) {
    let mut parts = SelectionStorage::new();
    parts.insert(Item::PowerConductor);
    for circuit in circuitry {
        world.level.circuitry.insert(
            Position { x: circuit.0, y: circuit.1 },
            Circuitry { parts: parts.clone(), powered: false }
        );
    }
}

pub fn insert_storage(world: &mut WorldData, storages: Vec<(i32, i32, Direction)>) {
    for storage in storages {
        world.level.storages.insert(
            Position { x: storage.0, y: storage.1 },
            Storage { content: SelectionStorage::new(), face: storage.2 }
        );
    }
}

pub fn insert_doors(world: &mut WorldData, doors: Vec<(i32, i32, DoorStatus, DoorType, Direction)>) {
    for door in doors {
        world.level.doors.insert(
            Position { x: door.0, y: door.1 },
            Door { status: door.2, variant: door.3, face: door.4 }
        );
    }
}

pub fn insert_player(world: &mut WorldData, pos: (i32, i32), dir: Direction, inv: Vec<Item>) {
    let player_position = Position::new(pos.0, pos.1);
    let player_front_tile = &dir.value() + &player_position;
    let mut inventory = <SelectionStorage<Item>>::new();
    for item in inv {
        inventory.insert(item);
    }
    let player = Player {
        position: player_position,
        movement: vec![],
        direction: dir,
        front_tile: player_front_tile,
        inventory,
        terminal: Box::new(Terminal {
            variant: TerminalType::Hud,
            dialog: Node::new(),
            front: Direction::Down
        }),
        log: SelectionStorage::new()
    };
    world.level.player = player;
}

pub fn insert_npc(world: &mut WorldData, x: i32, y: i32, npc: Npc) {
    world.level.npc.insert(
        Position { x, y },
        npc
    );
}