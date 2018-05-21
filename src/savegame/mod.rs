use std::fs;
use std::fs::File;
use std::io::Write;

use tar::{Builder, Archive};
use bincode;
use serde_yaml;

use constants::LEVEL_SIZE;
use world::WorldData;
use player::Player;
use objects::*;
use misc::{Position, Direction};
use storage::{Node, SelectionStorage};
use dialog::*;

pub mod static_levels;
pub mod static_npc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Save {
    name: String,
    backdrop: String,
    offset: Position
}

pub fn save_scene(world: &WorldData, filename: &str) {
    fs::create_dir("temp-save").unwrap();

    let save_info = Save { name: String::from(filename), backdrop: world.backdrop.clone(), offset: Position { x: 0, y: 0} };
    let bytes: Vec<u8> = bincode::serialize(&save_info).unwrap();
    File::create("temp-save/save-meta.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&world.player).unwrap();
    File::create("temp-save/player.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_walls: Vec<(i32, i32, Wall)> = vec![];
    for (pos, item) in world.walls.iter().enumerate() {
        if let Some(ref wall) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_walls.push((x, y, wall.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_walls).unwrap();
    File::create("temp-save/walls.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_floor: Vec<(i32, i32, Floor)> = vec![];
    for (pos, item) in world.floor.iter().enumerate() {
        if let Some(ref floor) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_floor.push((x, y, floor.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_floor).unwrap();
    File::create("temp-save/floor.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_doors: Vec<(i32, i32, Door)> = vec![];
    for (pos, item) in world.doors.iter().enumerate() {
        if let Some(ref door) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_doors.push((x, y, door.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_doors).unwrap();
    File::create("temp-save/doors.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_terminals: Vec<(i32, i32, Terminal)> = vec![];
    for (pos, item) in world.terminals.iter().enumerate() {
        if let Some(ref terminal) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_terminals.push((x, y, terminal.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_terminals).unwrap();
    File::create("temp-save/terminals.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_circuitry: Vec<(i32, i32, Circuitry)> = vec![];
    for (pos, item) in world.circuitry.iter().enumerate() {
        if let Some(ref circuitry) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_circuitry.push((x, y, circuitry.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_circuitry).unwrap();
    File::create("temp-save/circuitry.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_storages: Vec<(i32, i32, Storage)> = vec![];
    for (pos, item) in world.storages.iter().enumerate() {
        if let Some(ref storage) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_storages.push((x, y, storage.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_storages).unwrap();
    File::create("temp-save/storages.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_generators: Vec<(i32, i32, Generator)> = vec![];
    for (pos, item) in world.generators.iter().enumerate() {
        if let Some(ref generator) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_generators.push((x, y, generator.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_generators).unwrap();
    File::create("temp-save/generators.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_npc: Vec<(i32, i32, Npc)> = vec![];
    for (pos, item) in world.npc.iter().enumerate() {
        if let Some(ref npc) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_npc.push((x, y, npc.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_npc).unwrap();
    File::create("temp-save/npc.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_npc_dialog: Vec<Node<DialogItem>> = vec![];
    for item in world.npc.iter() {
        if let Some(ref npc) = *item {
            level_npc_dialog.push(npc.dialog.clone());
        }
    }
    let yaml = serde_yaml::to_string(&level_npc_dialog).unwrap();
    File::create("temp-save/npc-dialog.yaml").unwrap().write_all(&yaml.as_bytes()).unwrap();

    let file = File::create(filename).unwrap();
    let mut a = Builder::new(file);
    a.append_dir_all("save", "temp-save").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("temp-save").unwrap();
    println!("saved game: {}", filename);
}

pub fn load_scene(world: &mut WorldData, filename: &str) {
    if let Ok(file) = File::open(filename) {

        world.clear();

        let mut a = Archive::new(file);

        for file in a.entries().unwrap() {
            // Make sure there wasn't an I/O error
            let file = file.unwrap();

            match file.path().unwrap().file_stem().unwrap().to_str().unwrap() as &str {
                "walls" => {
                    let level_walls: Vec<(i32, i32, Wall)> = bincode::deserialize_from(file).unwrap();
                    for wall in level_walls {
                        world.walls.insert(Position {x: wall.0, y: wall.1}, wall.2);
                    }
                },
                "floor" => {
                    let level_floor: Vec<(i32, i32, Floor)> = bincode::deserialize_from(file).unwrap();
                    for floor in level_floor {
                        world.floor.insert(Position {x: floor.0, y: floor.1}, floor.2);
                    }
                },
                "doors" => {
                    let level_doors: Vec<(i32, i32, Door)> = bincode::deserialize_from(file).unwrap();
                    for door in level_doors {
                        world.doors.insert(Position {x: door.0, y: door.1}, door.2);
                    }
                },
                "terminals" => {
                    let level_terminals: Vec<(i32, i32, Terminal)> = bincode::deserialize_from(file).unwrap();
                    for terminal in level_terminals {
                        world.terminals.insert(Position {x: terminal.0, y: terminal.1}, terminal.2);
                    }
                },
                "circuitry" => {
                    let level_circuitry: Vec<(i32, i32, Circuitry)> = bincode::deserialize_from(file).unwrap();
                    for circuitry in level_circuitry {
                        world.circuitry.insert(Position {x: circuitry.0, y: circuitry.1}, circuitry.2);
                    }
                },
                "storages" => {
                    let level_storages: Vec<(i32, i32, Storage)> = bincode::deserialize_from(file).unwrap();
                    for storage in level_storages {
                        world.storages.insert(Position {x: storage.0, y: storage.1}, storage.2);
                    }
                },
                "generators" => {
                    let level_generators: Vec<(i32, i32, Generator)> = bincode::deserialize_from(file).unwrap();
                    for generator in level_generators {
                        world.generators.insert(Position {x: generator.0, y: generator.1}, generator.2);
                    }
                },
                "npc" => {
                    let level_npc: Vec<(i32, i32, Npc)> = bincode::deserialize_from(file).unwrap();
                    for npc in level_npc {
                        world.npc.insert(Position {x: npc.0, y: npc.1}, npc.2);
                    }
                },
                "player" => {
                    let level_player: Player = bincode::deserialize_from(file).unwrap();
                    world.player = level_player;
                },
                "save-meta" => {
                    let level_info: Save = bincode::deserialize_from(file).unwrap();
                    world.backdrop = level_info.backdrop;
                },
                _ => (),
            }

            world.update_power();
        }
        println!("game loaded: from file {}", filename);
    } else {
        static_levels::empty(world);
    }
    
}

pub fn insert_walls(world: &mut WorldData, walls: Vec<(i32, i32, WallType, Direction)>) {
    for wall in walls {
        world.walls.insert(
            Position { x: wall.0, y: wall.1 },
            Wall { variant: wall.2, face: wall.3 }
        );
    }
}

pub fn insert_floor(world: &mut WorldData, floor: Vec<(i32, i32, FloorType)>) {
    for tile in floor {
        world.floor.insert(
            Position { x: tile.0, y: tile.1 },
            Floor { variant: tile.2 }
        );
    }
}

pub fn insert_generator(world: &mut WorldData, generators: Vec<(i32, i32, Direction)>) {
    for generator in generators {
        world.generators.insert(
            Position { x: generator.0, y: generator.1 },
            Generator { face: generator.2 }
        );
    }
}

pub fn insert_pilot_seat(world: &mut WorldData, pilot_seats: Vec<(i32, i32, Direction)>) {
    for pilot_seat in pilot_seats {
        world.pilot_seats.insert(
            Position { x: pilot_seat.0, y: pilot_seat.1 },
            PilotSeat { front: pilot_seat.2 }
        );
    }
}

pub fn insert_circuitry(world: &mut WorldData, circuitry: Vec<(i32, i32)>) {
    let mut parts = SelectionStorage::new();
    parts.insert(Item::PowerConductor);
    for circuit in circuitry {
        world.circuitry.insert(
            Position { x: circuit.0, y: circuit.1 },
            Circuitry { parts: parts.clone(), powered: false }
        );
    }
}

pub fn insert_storage(world: &mut WorldData, storages: Vec<(i32, i32, Direction)>) {
    for storage in storages {
        world.storages.insert(
            Position { x: storage.0, y: storage.1 },
            Storage { content: SelectionStorage::new(), face: storage.2 }
        );
    }
}

pub fn insert_doors(world: &mut WorldData, doors: Vec<(i32, i32, DoorStatus, Direction)>) {
    for door in doors {
        world.doors.insert(
            Position { x: door.0, y: door.1 },
            Door { status: door.2, face: door.3 }
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
    world.player = player;
}

pub fn insert_npc(world: &mut WorldData, x: i32, y: i32, npc: Npc) {
    world.npc.insert(
        Position { x, y },
        npc
    );
}