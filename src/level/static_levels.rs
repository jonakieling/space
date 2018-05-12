use state::world::*;
use player::Player;
use objects::*;
use storage::{SelectionStorage, Tree, Node};
use misc::*;
use dialog::*;

pub fn static_level0(scene: &mut Scene) {
    scene.data.backdrop = String::from("/realm_of_sol__0000s_0000_1.1.png");
    let mut inventory = SelectionStorage::new();
    inventory.insert(Item::DataChip);
    inventory.insert(Item::Communicator);
    inventory.insert(Item::Terminal);

    let mut dialog = SelectionStorage::new();
    let mut dialog2 = SelectionStorage::new();
    dialog2.insert(Node {
        value: DialogItem {
            text: "Trade".to_string(),
            response: "Here are my goods".to_string(),
            action: Some(DialogAction::Trade)
        },
        children: SelectionStorage::new()
    });
    dialog.insert(Node {
        value: DialogItem {
            text: "Hi".to_string(),
            response: "Hello".to_string(),
            action: None
        },
        children: dialog2
    });
    dialog.insert(Node {
        value: DialogItem {
            text: "Bye".to_string(),
            response: "Goodbye".to_string(),
            action: None
        },
        children: SelectionStorage::new()
    });
    let npc_gnoerf = Npc {
        name: "Gnoerf".to_string(),
        direction: Direction::Left,
        look_at: Direction::Left,
        dialog: Tree {
            root: Node {
                value: DialogItem {
                    text: "".to_string(),
                    response: "...".to_string(),
                    action: None
                },
                children: dialog
            }
        },
        inventory
    };
    scene.data.npc.insert(Position::new(12, 12), npc_gnoerf);
    
    let mut inventory = <SelectionStorage<Item>>::new();
    inventory.insert(Item::MicroController);
    inventory.insert(Item::DataChip);
    scene.data.player.inventory = inventory;
    
    println!("game loaded: static level0");
}


pub fn _static_ship_tech_2_1(scene: &mut Scene) {
    scene.data.backdrop = String::from("/realm_of_sol__0000s_0001_2.1.png");

    scene.data.walls.insert(Position::new(6, 8), Wall {});
    scene.data.walls.insert(Position::new(7, 8), Wall {});
    scene.data.walls.insert(Position::new(8, 8), Wall {});
    scene.data.walls.insert(Position::new(9, 8), Wall {});
    scene.data.walls.insert(Position::new(10, 8), Wall {});

    scene.data.walls.insert(Position::new(6, 9), Wall {});
    scene.data.walls.insert(Position::new(6, 10), Wall {});

    scene.data.walls.insert(Position::new(7, 11), Wall {});
    scene.data.walls.insert(Position::new(9, 11), Wall {});

    scene.data.walls.insert(Position::new(10, 9), Wall {});
    scene.data.walls.insert(Position::new(10, 10), Wall {});

    scene.data.doors.insert(Position::new(8, 11), Door {status: DoorStatus::Closed});

    let mut parts = SelectionStorage::new();
    parts.insert(Item::PowerConductor);
    scene.data.circuitry.insert(Position::new(8, 13), Circuitry {parts, powered: false});

    scene.data.terminals.insert(Position::new(8, 9), Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });

    let player_position = Position::new(8, 10);
    let player_direction = Direction::Up;
    let player_front_tile = &player_direction.value() + &player_position;
    let mut inventory = <SelectionStorage<Item>>::new();
    inventory.insert(Item::Log);
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
    scene.data.player = player;

    println!("game loaded: static ship tech 2.1");
}
