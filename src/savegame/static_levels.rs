use app_state::ingame::*;
use objects::*;
use storage::{SelectionStorage, Tree, Node};
use misc::*;
use dialog::*;

pub fn empty(scene_data: &mut SceneData) {
    scene_data.backdrop = String::from("");

    super::insert_player(scene_data, (8, 10), Direction::Up, vec![]);
    
    println!("game loaded: static empty");
}

pub fn static_empty_npc(scene_data: &mut SceneData) {
    scene_data.backdrop = String::from("");

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
    scene_data.npc.insert(Position::new(12, 12), npc_gnoerf);
    
    let mut inventory = <SelectionStorage<Item>>::new();
    inventory.insert(Item::MicroController);
    inventory.insert(Item::DataChip);
    scene_data.player.inventory = inventory;
    
    println!("game loaded: static empty npc");
}

pub fn static_station_outpost(scene_data: &mut SceneData) {
    scene_data.backdrop = String::from("");

    super::insert_walls(scene_data, vec![
        (6, 8), (7, 8), (8, 8), (9, 8), (10, 8),
        (6, 9),                         (10, 9),
        (6, 10),                        (10, 10),
                (7, 11),        (9, 11)
    ]);

    super::insert_doors(scene_data, vec![
        (8, 11, DoorStatus::Closed)
    ]);

    super::insert_circuitry(scene_data, vec![
        (8, 13)
    ]);

    scene_data.terminals.insert(Position::new(8, 9), Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });

    super::insert_player(scene_data, (8, 10), Direction::Up, vec![]);

    println!("game loaded: static station outpost");
}

pub fn static_ship_tech(scene_data: &mut SceneData) {
    scene_data.backdrop = String::from("/realm_of_sol__0000s_0001_2.1.png");

    super::insert_walls(scene_data, vec![
        (6, 8), (7, 8), (8, 8), (9, 8), (10, 8),
        (6, 9),                         (10, 9),
        (6, 10),                        (10, 10),
                (7, 11),        (9, 11)
    ]);

    super::insert_doors(scene_data, vec![
        (8, 11, DoorStatus::Closed)
    ]);

    super::insert_circuitry(scene_data, vec![
        (8, 13)
    ]);

    scene_data.terminals.insert(Position::new(8, 9), Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });

    super::insert_player(scene_data, (8, 10), Direction::Up, vec![]);

    println!("game loaded: static ship tech 2.1");
}
