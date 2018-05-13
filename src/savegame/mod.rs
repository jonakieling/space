use std::fs;
use std::fs::File;
use std::io::Write;

use tar::{Builder, Archive};
use bincode;
use serde_yaml;

use constants::LEVEL_SIZE;
use app_state::ingame::*;
use player::Player;
use objects::*;
use misc::Position;
use storage::Tree;
use dialog::*;

pub mod static_levels;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Save {
    name: String,
    backdrop: String,
    offset: Position
}

pub fn save_scene(scene_data: &SceneData, filename: &str) {
    fs::create_dir("temp-save").unwrap();

    let save_info = Save { name: String::from(filename), backdrop: scene_data.backdrop.clone(), offset: Position { x: 0, y: 0} };
    let bytes: Vec<u8> = bincode::serialize(&save_info).unwrap();
    File::create("temp-save/save-meta.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&scene_data.player).unwrap();
    File::create("temp-save/player.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_walls: Vec<(i32, i32, Wall)> = vec![];
    for (pos, item) in scene_data.walls.iter().enumerate() {
        if let Some(ref wall) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_walls.push((x, y, wall.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_walls).unwrap();
    File::create("temp-save/walls.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_doors: Vec<(i32, i32, Door)> = vec![];
    for (pos, item) in scene_data.doors.iter().enumerate() {
        if let Some(ref door) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_doors.push((x, y, door.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_doors).unwrap();
    File::create("temp-save/doors.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_terminals: Vec<(i32, i32, Terminal)> = vec![];
    for (pos, item) in scene_data.terminals.iter().enumerate() {
        if let Some(ref terminal) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_terminals.push((x, y, terminal.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_terminals).unwrap();
    File::create("temp-save/terminals.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_circuitry: Vec<(i32, i32, Circuitry)> = vec![];
    for (pos, item) in scene_data.circuitry.iter().enumerate() {
        if let Some(ref circuitry) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_circuitry.push((x, y, circuitry.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_circuitry).unwrap();
    File::create("temp-save/circuitry.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_storages: Vec<(i32, i32, Storage)> = vec![];
    for (pos, item) in scene_data.storages.iter().enumerate() {
        if let Some(ref storage) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_storages.push((x, y, storage.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_storages).unwrap();
    File::create("temp-save/storages.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_generators: Vec<(i32, i32, Generator)> = vec![];
    for (pos, item) in scene_data.generators.iter().enumerate() {
        if let Some(ref generator) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_generators.push((x, y, generator.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_generators).unwrap();
    File::create("temp-save/generators.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_npc: Vec<(i32, i32, Npc)> = vec![];
    for (pos, item) in scene_data.npc.iter().enumerate() {
        if let Some(ref npc) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_npc.push((x, y, npc.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_npc).unwrap();
    File::create("temp-save/npc.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_npc_dialog: Vec<Tree<DialogItem>> = vec![];
    for item in scene_data.npc.iter() {
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

pub fn load_scene(scene_data: &mut SceneData, filename: &str) {
    if let Ok(file) = File::open(filename) {
        let mut a = Archive::new(file);

        for file in a.entries().unwrap() {
            // Make sure there wasn't an I/O error
            let file = file.unwrap();

            match file.path().unwrap().file_stem().unwrap().to_str().unwrap() as &str {
                "walls" => {
                    let level_walls: Vec<(i32, i32, Wall)> = bincode::deserialize_from(file).unwrap();
                    for wall in level_walls {
                        scene_data.walls.insert(Position {x: wall.0, y: wall.1}, wall.2);
                    }
                },
                "doors" => {
                    let level_doors: Vec<(i32, i32, Door)> = bincode::deserialize_from(file).unwrap();
                    for door in level_doors {
                        scene_data.doors.insert(Position {x: door.0, y: door.1}, door.2);
                    }
                },
                "terminals" => {
                    let level_terminals: Vec<(i32, i32, Terminal)> = bincode::deserialize_from(file).unwrap();
                    for terminal in level_terminals {
                        scene_data.terminals.insert(Position {x: terminal.0, y: terminal.1}, terminal.2);
                    }
                },
                "circuitry" => {
                    let level_circuitry: Vec<(i32, i32, Circuitry)> = bincode::deserialize_from(file).unwrap();
                    for circuitry in level_circuitry {
                        scene_data.circuitry.insert(Position {x: circuitry.0, y: circuitry.1}, circuitry.2);
                    }
                },
                "storages" => {
                    let level_storages: Vec<(i32, i32, Storage)> = bincode::deserialize_from(file).unwrap();
                    for storage in level_storages {
                        scene_data.storages.insert(Position {x: storage.0, y: storage.1}, storage.2);
                    }
                },
                "generators" => {
                    let level_generators: Vec<(i32, i32, Generator)> = bincode::deserialize_from(file).unwrap();
                    for generator in level_generators {
                        scene_data.generators.insert(Position {x: generator.0, y: generator.1}, generator.2);
                    }
                },
                "npc" => {
                    let level_npc: Vec<(i32, i32, Npc)> = bincode::deserialize_from(file).unwrap();
                    for npc in level_npc {
                        scene_data.npc.insert(Position {x: npc.0, y: npc.1}, npc.2);
                    }
                },
                "player" => {
                    let level_player: Player = bincode::deserialize_from(file).unwrap();
                    scene_data.player = level_player;
                },
                "save-meta" => {
                    let level_info: Save = bincode::deserialize_from(file).unwrap();
                    scene_data.backdrop = level_info.backdrop;
                },
                _ => (),
            }

            scene_data.update_power();
        }
        println!("game loaded: from file {}", filename);
    } else {
        static_levels::static_level0(scene_data);
    }
    
}