use std::fs;
use std::fs::File;
use std::io;
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

pub fn save_game(world: &WorldData) {
    fs::create_dir("temp-save").unwrap();

    let save_info = Save { location: world.level.location.clone(), backdrop: world.level.backdrop.clone(), offset: Position { x: 0, y: 0} };
    let bytes: Vec<u8> = bincode::serialize(&save_info).unwrap();
    File::create("temp-save/save-meta.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&world.level).unwrap();
    File::create("temp-save/level.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&world.universe).unwrap();
    File::create("temp-save/universe.bin").unwrap().write_all(&bytes).unwrap();

    let file = File::create("saves/autosave.tar").expect("saves/autosave.tar");
    let mut a = Builder::new(file);
    a.append_dir_all("save", "temp-save").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("temp-save").unwrap();
    println!("saved game to saves/autosave.tar");
}

pub fn load_game(world: &mut WorldData) {
    if let Ok(file) = File::open("saves/autosave.tar") {

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
        println!("game loaded from saves/autosave.tar");
    } else {
        static_levels::empty(world);
    }
}

pub fn save_location(world: &mut WorldData) {
    world.levels.insert(world.level.location.clone(), world.level.clone());

    let dir = format!("levels/u{}", world.universe.id);
    fs::create_dir_all(dir).expect("universe folder could not be created");
    match &world.level.location {
        Location::Ship(id) => {
            let file = format!("levels/u{}/{}.ship.tar", world.universe.id, id);
            save_level(world, &file);
        },
        Location::Station(id) => {
            let file = format!("levels/u{}/{}.station.tar", world.universe.id, id);
            save_level(world, &file);
        },
        Location::Space => {
            let file = format!("levels/u{}/space.tar", world.universe.id);
            save_level(world, &file);
        }
    }
}

pub fn load_location(world: &mut WorldData, location: &Location) {
    let mut loaded = false;
    let inventory = world.level.player.inventory.clone();
    let direction = world.level.player.direction.clone();

    {
        if let Some(level) = world.levels.get(location) {
            world.level = level.clone();
            loaded = true;
            println!("loaded game from memory");
        }
    }

    if !loaded {
        match location {
            Location::Ship(id) => {
                let file = format!("levels/u{}/{}.ship.tar", world.universe.id, id);
                if load_level(world, &file).is_err() {
                    if id == "Tech 2.1" {
                        static_levels::static_ship_tech(world);
                    } else {
                        static_levels::empty(world);
                        world.level.location = location.clone();
                    }
                }
            },
            Location::Station(id) => {
                let file = format!("levels/u{}/{}.station.tar", world.universe.id, id);
                if load_level(world, &file).is_err() {
                    if id == "Mun" {
                        static_levels::static_station_outpost(world);
                    } else {
                        static_levels::empty(world);
                        world.level.location = location.clone();
                    }    
                }
            },
            Location::Space => static_levels::empty(world),
        }
    }

    world.universe.player_location = world.level.location.clone();
    world.level.player.inventory = inventory;
    world.level.player.direction = direction;
}

fn save_level(world: &WorldData, filename: &str) {
    fs::create_dir("temp-save").unwrap();

    let save_info = Save { location: world.level.location.clone(), backdrop: world.level.backdrop.clone(), offset: Position { x: 0, y: 0} };
    let bytes: Vec<u8> = bincode::serialize(&save_info).unwrap();
    File::create("temp-save/save-meta.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&world.level).unwrap();
    File::create("temp-save/level.bin").unwrap().write_all(&bytes).unwrap();

    let file = File::create(filename).expect(filename);
    let mut a = Builder::new(file);
    a.append_dir_all("save", "temp-save").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("temp-save").unwrap();
    println!("saved level: {}", filename);
}

fn load_level(world: &mut WorldData, filename: &str) -> Result<(), io::Error> {
    let file = File::open(filename)?;

    world.level.clear();

    let mut a = Archive::new(file);

    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let file = file.unwrap();

        match file.path().unwrap().file_stem().unwrap().to_str().unwrap() as &str {
            "level" => {
                world.level = bincode::deserialize_from(file).unwrap();
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
    println!("level loaded: from file {}", filename);

    Ok(())
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