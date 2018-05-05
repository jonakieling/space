use std::fs;
use std::fs::File;
use std::io::Write;

use tar::{Builder, Archive};
use bincode;
use serde_yaml;

use constants::LEVEL_SIZE;
use state::world::*;
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

pub fn save_scene(scene: &Scene, filename: &str) {
    fs::create_dir("temp-save").unwrap();

    let save_info = Save { name: String::from(filename), backdrop: scene.backdrop.clone(), offset: Position { x: 0, y: 0} };
    let bytes: Vec<u8> = bincode::serialize(&save_info).unwrap();
    File::create("temp-save/save-meta.bin").unwrap().write_all(&bytes).unwrap();

    let bytes: Vec<u8> = bincode::serialize(&scene.player).unwrap();
    File::create("temp-save/player.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_walls: Vec<(i32, i32, Wall)> = vec![];
    for (pos, item) in scene.walls.iter().enumerate() {
        if let Some(ref wall) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_walls.push((x, y, wall.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_walls).unwrap();
    File::create("temp-save/walls.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_doors: Vec<(i32, i32, Door)> = vec![];
    for (pos, item) in scene.doors.iter().enumerate() {
        if let Some(ref door) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_doors.push((x, y, door.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_doors).unwrap();
    File::create("temp-save/doors.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_terminals: Vec<(i32, i32, Terminal)> = vec![];
    for (pos, item) in scene.terminals.iter().enumerate() {
        if let Some(ref terminal) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_terminals.push((x, y, terminal.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_terminals).unwrap();
    File::create("temp-save/terminals.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_circuitry: Vec<(i32, i32, Circuitry)> = vec![];
    for (pos, item) in scene.circuitry.iter().enumerate() {
        if let Some(ref circuitry) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_circuitry.push((x, y, circuitry.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_circuitry).unwrap();
    File::create("temp-save/circuitry.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_generators: Vec<(i32, i32, Generator)> = vec![];
    for (pos, item) in scene.generators.iter().enumerate() {
        if let Some(ref generator) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_generators.push((x, y, generator.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_generators).unwrap();
    File::create("temp-save/generators.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_npc: Vec<(i32, i32, NPC)> = vec![];
    for (pos, item) in scene.npc.iter().enumerate() {
        if let Some(ref npc) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_npc.push((x, y, npc.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_npc).unwrap();
    File::create("temp-save/npc.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_npc_dialog: Vec<Tree<DialogItem>> = vec![];
    for item in scene.npc.iter() {
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

pub fn load_scene(scene: &mut Scene, filename: &str) {
    if let Ok(file) = File::open(filename) {
        let mut a = Archive::new(file);

        for file in a.entries().unwrap() {
            // Make sure there wasn't an I/O error
            let file = file.unwrap();

            match file.path().unwrap().file_stem().unwrap().to_str().unwrap() as &str {
                "walls" => {
                    let level_walls: Vec<(i32, i32, Wall)> = bincode::deserialize_from(file).unwrap();
                    for wall in level_walls {
                        scene.walls.insert(wall.0, wall.1, wall.2);
                    }
                },
                "doors" => {
                    let level_doors: Vec<(i32, i32, Door)> = bincode::deserialize_from(file).unwrap();
                    for door in level_doors {
                        scene.doors.insert(door.0, door.1, door.2);
                    }
                },
                "terminals" => {
                    let level_terminals: Vec<(i32, i32, Terminal)> = bincode::deserialize_from(file).unwrap();
                    for terminal in level_terminals {
                        scene.terminals.insert(terminal.0, terminal.1, terminal.2);
                    }
                },
                "circuitry" => {
                    let level_circuitry: Vec<(i32, i32, Circuitry)> = bincode::deserialize_from(file).unwrap();
                    for circuitry in level_circuitry {
                        scene.circuitry.insert(circuitry.0, circuitry.1, circuitry.2);
                    }
                },
                "generators" => {
                    let level_generators: Vec<(i32, i32, Generator)> = bincode::deserialize_from(file).unwrap();
                    for generator in level_generators {
                        scene.generators.insert(generator.0, generator.1, generator.2);
                    }
                },
                "npc" => {
                    let level_npc: Vec<(i32, i32, NPC)> = bincode::deserialize_from(file).unwrap();
                    for npc in level_npc {
                        scene.npc.insert(npc.0, npc.1, npc.2);
                    }
                },
                "player" => {
                    let level_player: Player = bincode::deserialize_from(file).unwrap();
                    scene.player = level_player;
                },
                "save-meta" => {
                    let level_info: Save = bincode::deserialize_from(file).unwrap();
                    scene.backdrop = level_info.backdrop;
                },
                _ => (),
            }
        }
        println!("game loaded: from file {}", filename);
    } else {
        static_levels::static_level0(scene);
    }
    
}