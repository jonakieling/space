use state::world::*;
use player::Player;
use objects::*;
use storage::{SelectionStorage, Tree, Node};
use misc::*;
use dialog::*;

pub fn static_level0(scene: &mut Scene) {
    scene.backdrop = String::from("/realm_of_sol__0000s_0000_1.1.png");
    let mut inventory = SelectionStorage::new();
    inventory.insert(Item::Paper);
    let mut dialog = SelectionStorage::new();
    dialog.insert(Node {
        value: DialogItem {
            text: "Hi".to_string(),
            response: "Hello".to_string()
        },
        children: SelectionStorage::new()
    });
    dialog.insert(Node {
        value: DialogItem {
            text: "Bye".to_string(),
            response: "Goodbye".to_string()
        },
        children: SelectionStorage::new()
    });
    let npc_gnoerf = NPC {
        name: "Gnoerf".to_string(),
        direction: Direction::Left,
        look_at: Direction::Left,
        dialog: Tree {
            root: Node {
                value: DialogItem {
                    text: "".to_string(),
                    response: "...".to_string()
                },
                children: dialog
            }
        },
        inventory
    };
    scene.npc.insert(12, 12, npc_gnoerf);
    println!("game loaded: static level0");
}


pub fn _static_ship_tech_2_1(scene: &mut Scene) {
    scene.backdrop = String::from("/realm_of_sol__0000s_0001_2.1.png");

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
