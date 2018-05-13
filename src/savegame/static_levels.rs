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
        (6, 8), (7, 8), (8, 8), (9, 8), (10, 8), (11, 8),
        (6, 9),                                  (11, 9),
        (6, 10),                                 (11, 10),
        (6, 11),                                 (11, 11),
        (6, 12),                                 (11, 12),(12, 12),(13, 12), (14, 12),
        (6, 13),                                 (11, 13),                   (14, 13),
        (6, 14),                                                    
        (6, 15),                                 (11, 15),                   (14, 15),
        (6, 16),(7, 16),(8, 16),(9, 16),(10, 16),(11, 16),(12, 16),(13, 16), (14, 16)       
    ]);

    super::insert_doors(scene_data, vec![
        (11, 14, DoorStatus::Closed)
    ]);

    scene_data.terminals.insert(Position::new(14, 14), Terminal {
        text: Box::new(String::new()),
        front: Direction::Left
    });

    super::insert_storage(scene_data, vec![
        (7, 9),
        (7, 10)
    ]);

    super::insert_player(scene_data, (9, 11), Direction::Down, vec![]);

    println!("game loaded: static station outpost");
}

pub fn static_ship_tech(scene_data: &mut SceneData) {
    scene_data.backdrop = String::from("/realm_of_sol__0000s_0001_2.1.png");

    super::insert_walls(scene_data, vec![
        (6, 8), (7, 8), (8, 8), (9, 8), (10, 8),
        (6, 9),                         (10, 9),
        (6, 10),                        (10, 10),
                (7, 11),        (9, 11),
        (6, 12),                        (10, 12),
        (6, 13),
        (6, 14),                        (10, 14),
        (6, 15),                        (10, 15),
        (6, 16),                        (10, 16),
        (6, 17),(7, 17),(8, 17),(9, 17),(10, 17)
    ]);

    super::insert_doors(scene_data, vec![
        (8, 11, DoorStatus::Open),
        (10, 13, DoorStatus::Closed)
    ]);

    super::insert_generator(scene_data, vec![
        (8, 15)
    ]);

    super::insert_circuitry(scene_data, vec![
                        (10, 2),
                        (10, 3),          (12, 3),
                        (10, 4),          (12, 4),
                        (10, 5),          (12, 5),
                        (10, 6),          (12, 6),
                        (10, 7),          (12, 7),
        (8, 8), (9, 8), (10, 8), (11, 8), (12, 8),
        (8, 9),         (10, 9),
                        (10, 10),
                        (10, 11),
                        (10, 12),
                        (10, 13),
                        (10, 14),
                (9, 15),(10, 15)
    ]);

    scene_data.update_power();

    super::insert_storage(scene_data, vec![
        (7, 12),
        (7, 13)
    ]);

    scene_data.terminals.insert(Position::new(8, 9), Terminal {
        text: Box::new(String::new()),
        front: Direction::Down
    });

    super::insert_player(scene_data, (8, 10), Direction::Up, vec![]);

    println!("game loaded: static ship tech");
}
