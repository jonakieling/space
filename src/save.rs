use tar::{Builder, Archive};
use std::fs;
use std::fs::File;
use std::io::Write;
use bincode;
use scene::*;
use player::Player;

pub fn save_scene(scene: &Scene) {
    fs::create_dir("dev-level").unwrap();

    let bytes: Vec<u8> = bincode::serialize(&scene.player).unwrap();
    File::create("dev-level/player.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_walls: Vec<(i32, i32, Wall)> = vec![];
    for (pos, item) in scene.walls.iter().enumerate() {
        if let Some(ref wall) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_walls.push((x, y, *wall.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_walls).unwrap();
    File::create("dev-level/walls.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_doors: Vec<(i32, i32, Door)> = vec![];
    for (pos, item) in scene.doors.iter().enumerate() {
        if let Some(ref door) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_doors.push((x, y, *door.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_doors).unwrap();
    File::create("dev-level/doors.bin").unwrap().write_all(&bytes).unwrap();

    let mut level_terminals: Vec<(i32, i32, Terminal)> = vec![];
    for (pos, item) in scene.terminals.iter().enumerate() {
        if let Some(ref terminal) = *item {
            let x = pos as i32 % LEVEL_SIZE;
            let y = pos as i32 / LEVEL_SIZE;
            level_terminals.push((x, y, *terminal.clone()));
        }
    }
    let bytes: Vec<u8> = bincode::serialize(&level_terminals).unwrap();
    File::create("dev-level/terminals.bin").unwrap().write_all(&bytes).unwrap();

    let file = File::create("dev-level.tar").unwrap();
    let mut a = Builder::new(file);
    a.append_dir_all("dev-level", "dev-level").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("dev-level").unwrap();
    println!("saved game: dev-level");
}

pub fn load_scene(scene: &mut Scene) {
    if let Ok(file) = File::open("dev-level.tar") {
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
        println!("game loaded: from file dev-level.tar");
    } else {
        static_ship_tech_2_1(scene);
    }
    
}

pub fn static_level0(scene: &mut Scene) {
    scene.walls.insert(1, 2, Wall {});
    scene.walls.insert(2, 2, Wall {});
    scene.walls.insert(3, 2, Wall {});
    scene.walls.insert(4, 2, Wall {});
    scene.walls.insert(5, 2, Wall {});
    scene.walls.insert(6, 2, Wall {});
    scene.walls.insert(7, 2, Wall {});
    scene.walls.insert(8, 2, Wall {});

    scene.walls.insert(1, 3, Wall {});
    scene.walls.insert(1, 4, Wall {});
    scene.walls.insert(1, 5, Wall {});
    scene.walls.insert(1, 6, Wall {});

    scene.walls.insert(1, 6, Wall {});
    scene.walls.insert(2, 6, Wall {});
    scene.walls.insert(3, 6, Wall {});
    scene.walls.insert(4, 6, Wall {});
    scene.walls.insert(6, 6, Wall {});
    scene.walls.insert(7, 6, Wall {});
    scene.walls.insert(8, 6, Wall {});

    scene.walls.insert(8, 3, Wall {});
    scene.walls.insert(8, 4, Wall {});
    scene.walls.insert(8, 5, Wall {});
    scene.walls.insert(8, 6, Wall {});


    scene.walls.insert(1, 7, Wall {});


    scene.walls.insert(1, 8, Wall {});
    scene.walls.insert(2, 8, Wall {});
    scene.walls.insert(3, 8, Wall {});
    scene.walls.insert(4, 8, Wall {});
    scene.walls.insert(6, 8, Wall {});
    scene.walls.insert(7, 8, Wall {});
    scene.walls.insert(8, 8, Wall {});

    scene.walls.insert(1, 8, Wall {});
    scene.walls.insert(1, 9, Wall {});
    scene.walls.insert(1, 10, Wall {});
    scene.walls.insert(1, 11, Wall {});

    scene.walls.insert(1, 12, Wall {});
    scene.walls.insert(2, 12, Wall {});
    scene.walls.insert(3, 12, Wall {});
    scene.walls.insert(4, 12, Wall {});
    scene.walls.insert(5, 12, Wall {});
    scene.walls.insert(6, 12, Wall {});
    scene.walls.insert(7, 12, Wall {});
    scene.walls.insert(8, 12, Wall {});

    scene.walls.insert(8, 8, Wall {});
    scene.walls.insert(8, 9, Wall {});
    scene.walls.insert(8, 10, Wall {});
    scene.walls.insert(8, 11, Wall {});


    scene.doors.insert(5, 6, Door {status: DoorStatus::Closed});
    scene.doors.insert(5, 8, Door {status: DoorStatus::Closed});
    scene.doors.insert(8, 7, Door {status: DoorStatus::Closed});


    scene.terminals.insert(12, 10, Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });
    scene.terminals.insert(12, 12, Terminal {
        text: Box::new(String::new()),
        front: Direction::Up
    });


    let player_position = Position { x: 10, y: 10 };
    let player_direction = Direction::Down;
    let player_front_tile = &player_direction.value() + &player_position;
    let player = Player {
        position: player_position,
        movement: vec![],
        direction: player_direction,
        front_tile: player_front_tile
    };
    scene.player = player;

    println!("game loaded: static level0");
}


pub fn static_ship_tech_2_1(scene: &mut Scene) {
    scene.walls.insert(2, 2, Wall {});
    scene.walls.insert(3, 2, Wall {});
    scene.walls.insert(4, 2, Wall {});
    scene.walls.insert(5, 2, Wall {});
    scene.walls.insert(6, 2, Wall {});

    scene.walls.insert(2, 3, Wall {});
    scene.walls.insert(2, 4, Wall {});

    scene.walls.insert(3, 5, Wall {});
    scene.walls.insert(5, 5, Wall {});

    scene.walls.insert(6, 3, Wall {});
    scene.walls.insert(6, 4, Wall {});

    scene.doors.insert(4, 5, Door {status: DoorStatus::Closed});


    scene.terminals.insert(4, 3, Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });


    let player_position = Position { x: 4, y: 4 };
    let player_direction = Direction::Up;
    let player_front_tile = &player_direction.value() + &player_position;
    let player = Player {
        position: player_position,
        movement: vec![],
        direction: player_direction,
        front_tile: player_front_tile
    };
    scene.player = player;

    println!("game loaded: static ship tech 2.1");
}
