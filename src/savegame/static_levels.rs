use world::WorldData;
use objects::*;
use misc::*;
use savegame::static_npc::*;
use storage::{Node, SelectionStorage};
use dialog::*;
use feature::map::MapFeature;

pub fn empty(data: &mut WorldData) {
    data.level.backdrop = String::from("");

    super::insert_player(data, (1, 1), Direction::Down, vec![]);
    
    println!("game loaded: static empty");
}

pub fn static_station_outpost(data: &mut WorldData) {
    data.level.clear();
    data.level.backdrop = String::from("");
    data.level.location = Location::Station("Mun".to_string());

    super::insert_floor(data, vec![
        (7, 9, FloorType::Regular),
        (8, 9, FloorType::Regular),
        (9, 9, FloorType::Regular),
        (10, 9, FloorType::Regular),
        (7, 10, FloorType::Regular),
        (8, 10, FloorType::Regular),
        (9, 10, FloorType::Regular),
        (10, 10, FloorType::Regular),
        (7, 11, FloorType::Regular),
        (8, 11, FloorType::Regular),
        (9, 11, FloorType::Regular),
        (10, 11, FloorType::Regular),
        (7, 12, FloorType::Regular),
        (8, 12, FloorType::Regular),
        (9, 12, FloorType::Regular),
        (10, 12, FloorType::Regular),
        (7, 13, FloorType::Regular),
        (8, 13, FloorType::Regular),
        (9, 13, FloorType::Regular),
        (10, 13, FloorType::Regular),
        (7, 14, FloorType::Regular),
        (8, 14, FloorType::Regular),
        (9, 14, FloorType::Regular),
        (10, 14, FloorType::Regular),
        (7, 15, FloorType::Regular),
        (8, 15, FloorType::Regular),
        (9, 15, FloorType::Regular),
        (10, 15, FloorType::Regular),
        (12, 14, FloorType::Regular),
        (13, 13, FloorType::Regular),
        (13, 14, FloorType::Regular),
        (13, 15, FloorType::Regular),
    ]);

    super::insert_walls(data, vec![
        (6, 8, WallType::Corner, Direction::Down),
        (7, 8, WallType::Wall, Direction::Down),
        (8, 8, WallType::Wall, Direction::Down),
        (9, 8, WallType::Wall, Direction::Down),
        (10, 8, WallType::Wall, Direction::Down),
        (11, 8, WallType::Corner, Direction::Left),
        (6, 9, WallType::Wall, Direction::Right),
        (11, 9, WallType::Wall, Direction::Left),
        (6, 10, WallType::Wall, Direction::Right),
        (11, 10, WallType::Wall, Direction::Left),
        (6, 11, WallType::Wall, Direction::Right),
        (11, 11, WallType::Wall, Direction::Left),
        (6, 12, WallType::Wall, Direction::Right),
        (11, 12, WallType::Wall, Direction::Left),
        (12, 12, WallType::Corner, Direction::Down),
        (13, 12, WallType::Wall, Direction::Down),
        (14, 12, WallType::Corner, Direction::Left),
        (11, 13, WallType::Wall, Direction::Left),
        (12, 13, WallType::Wall, Direction::Right),
        (14, 13, WallType::Wall, Direction::Left),
        (6, 14, WallType::Wall, Direction::Right),                                                     
        (6, 15, WallType::Wall, Direction::Right), 
        (14, 14, WallType::Wall, Direction::Left),
        (11, 15, WallType::Wall, Direction::Left),
        (12, 15, WallType::Wall, Direction::Right),
        (14, 15, WallType::Wall, Direction::Left),
        (6, 16, WallType::Corner, Direction::Right),
        (7, 16, WallType::Wall, Direction::Up),
        (8, 16, WallType::Wall, Direction::Up),
        (9, 16, WallType::Wall, Direction::Up),
        (10, 16, WallType::Wall, Direction::Up),
        (11, 16, WallType::Corner, Direction::Up),
        (12, 16, WallType::Corner, Direction::Right),
        (13, 16, WallType::Wall, Direction::Up),
        (14, 16, WallType::Corner, Direction::Up)       
    ]);

    super::insert_doors(data, vec![
        (11, 14, DoorStatus::Closed, DoorType::Passage, Direction::Left),
        (6, 13, DoorStatus::Closed, DoorType::Exit(Location::Ship("Tech 2.1".to_string())), Direction::Right)
    ]);

    data.level.terminals.insert(Position::new(14, 14), Terminal {
        variant: TerminalType::Intercomm,
        dialog: Node::new(),
        front: Direction::Left
    });

    super::insert_storage(data, vec![
        (7, 9, Direction::Right),
        (7, 10, Direction::Right)
    ]);

    super::insert_npc(data, 10 ,13, guard(Direction::Left));

    super::insert_npc(data, 9 ,9, gnoerf(Direction::Down));

    super::insert_player(data, (7, 13), Direction::Right, vec![Item::Navcomp]);

    println!("game loaded: static station outpost");
}

pub fn static_ship_tech(data: &mut WorldData) {
    data.level.clear();

    data.level.backdrop = String::from("/realm_of_sol__0000s_0001_2.1.png");
    data.level.location = Location::Station("Tech 2.1".to_string());

    super::insert_floor(data, vec![
        (7, 9, FloorType::Light),
        (8, 9, FloorType::Light),
        (9, 9, FloorType::Light),
        (7, 10, FloorType::Regular),
        (8, 10, FloorType::Regular),
        (9, 10, FloorType::Regular),
        (7, 12, FloorType::Regular),
        (8, 12, FloorType::Regular),
        (9, 12, FloorType::Regular),
        (7, 13, FloorType::Regular),
        (8, 13, FloorType::Regular),
        (9, 13, FloorType::Regular),
        (7, 14, FloorType::Regular),
        (8, 14, FloorType::Regular),
        (9, 14, FloorType::Regular),
        (7, 15, FloorType::Light),
        (8, 15, FloorType::Light),
        (9, 15, FloorType::Light),
        (7, 16, FloorType::Regular),
        (8, 16, FloorType::Regular),
        (9, 16, FloorType::Regular),
    ]);

    super::insert_walls(data, vec![
        (6, 8, WallType::Corner, Direction::Down),
        (7, 8, WallType::Window, Direction::Down),
        (8, 8, WallType::Wall, Direction::Down),
        (9, 8, WallType::Window, Direction::Down),
        (10, 8, WallType::Corner, Direction::Left),
        (6, 9, WallType::Window, Direction::Right),
        (10, 9, WallType::Window, Direction::Left),
        (6, 10, WallType::Corner, Direction::Right),
        (7, 10, WallType::Wall, Direction::Up),
        (9, 10, WallType::Wall, Direction::Up),
        (10, 10, WallType::Corner, Direction::Up),
        (6, 11, WallType::Corner, Direction::Down),
        (7, 11, WallType::Wall, Direction::Down),
        (9, 11, WallType::Wall, Direction::Down),
        (10, 11, WallType::Corner, Direction::Left),
        (6, 12, WallType::Wall, Direction::Right),
        (10, 12, WallType::Wall, Direction::Left),
        (6, 13, WallType::Wall, Direction::Right),
        (6, 14, WallType::Wall, Direction::Right),
        (10, 14, WallType::Wall, Direction::Left),
        (6, 15, WallType::Window, Direction::Right),
        (10, 15, WallType::Wall, Direction::Left),
        (6, 16, WallType::Wall, Direction::Right),
        (10, 16, WallType::Wall, Direction::Left),
        (6, 17, WallType::Corner, Direction::Right),
        (7, 17, WallType::Wall, Direction::Up),
        (8, 17, WallType::Wall, Direction::Up),
        (9, 17, WallType::Wall, Direction::Up),
        (10, 17, WallType::Corner, Direction::Up)
    ]);

    super::insert_doors(data, vec![
        (8, 11, DoorStatus::Open, DoorType::Passage, Direction::Down),
        (10, 13, DoorStatus::Closed, DoorType::Exit(Location::Station("Mun".to_string())), Direction::Left)
    ]);

    super::insert_generator(data, vec![
        (8, 15, Direction::Down)
    ]);

    super::insert_pilot_seat(data, vec![
        (8, 9, Direction::Down)
    ]);

    super::insert_circuitry(data, vec![
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

    data.level.update_power();

    super::insert_storage(data, vec![
        (7, 12, Direction::Right),
        (7, 13, Direction::Right)
    ]);

    let mut ship_console_dialog_children =  SelectionStorage::new();
    ship_console_dialog_children.insert(
        Node {
            value: DialogItem {
                text: "Navigate".to_string(),
                response: "".to_string(),
                action: Some(DialogAction::Map(MapFeature::Navigate))
            },
            children: SelectionStorage::new()
        }
    );

    data.level.terminals.insert(Position::new(8, 8), Terminal {
        variant: TerminalType::ShipConsole,
        dialog: Node {
            value: DialogItem {
                text: "".to_string(),
                response: "ship console".to_string(),
                action: None
            },
            children: ship_console_dialog_children
        },
        front: Direction::Down
    });

    data.level.terminals.insert(Position::new(10, 15), Terminal {
        variant: TerminalType::Intercomm,
        dialog: Node::new(),
        front: Direction::Left
    });

    super::insert_player(data, (9, 13), Direction::Left, vec![Item::Navcomp]);

    println!("game loaded: static ship tech");
}
