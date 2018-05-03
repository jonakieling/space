use state::world::*;
use player::Player;
use objects::*;
use storage::SelectionStorage;
use misc::*;

pub fn _static_level0(scene: &mut Scene) {
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
    let mut inventory = <SelectionStorage<Item>>::new();
    inventory.insert(Item::Log);
    inventory.insert(Item::Communicator);
    let player = Player {
        position: player_position,
        movement: vec![],
        direction: player_direction,
        front_tile: player_front_tile,
        inventory,
        terminal: Box::new(Terminal {
            text: Box::new(String::new()),
            front: Direction::Down
        })
    };
    scene.player = player;

    println!("game loaded: static level0");
}


pub fn _static_ship_tech_2_1(scene: &mut Scene) {
    scene.walls.insert(6, 8, Wall {});
    scene.walls.insert(7, 8, Wall {});
    scene.walls.insert(8, 8, Wall {});
    scene.walls.insert(9, 8, Wall {});
    scene.walls.insert(10, 8, Wall {});

    scene.walls.insert(6, 9, Wall {});
    scene.walls.insert(6, 10, Wall {});

    scene.walls.insert(7, 11, Wall {});
    scene.walls.insert(9, 11, Wall {});

    scene.walls.insert(10, 9, Wall {});
    scene.walls.insert(10, 10, Wall {});

    scene.doors.insert(8, 11, Door {status: DoorStatus::Closed});

    let mut parts = SelectionStorage::new();
    parts.insert(Item::Chip);
    parts.insert(Item::Chip);
    parts.insert(Item::Cable);
    parts.insert(Item::Isolation);
    parts.insert(Item::Isolation);
    parts.insert(Item::Isolation);
    parts.insert(Item::Adapter);
    scene.circuitry.insert(8, 13, Circuitry {parts, powered: false});


    scene.terminals.insert(8, 9, Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });


    let player_position = Position { x: 8, y: 10 };
    let player_direction = Direction::Up;
    let player_front_tile = &player_direction.value() + &player_position;
    let mut inventory = <SelectionStorage<Item>>::new();
    inventory.insert(Item::Log);
    inventory.insert(Item::PilotLicense);
    inventory.insert(Item::Communicator);
    inventory.insert(Item::Terminal);
    let player = Player {
        position: player_position,
        movement: vec![],
        direction: player_direction,
        front_tile: player_front_tile,
        inventory,
        terminal: Box::new(Terminal {
            text: Box::new(String::new()),
            front: Direction::Down
        })
    };
    scene.player = player;

    println!("game loaded: static ship tech 2.1");
}
