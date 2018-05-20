use objects::*;
use storage::{SelectionStorage, Node};
use dialog::*;
use misc::Direction;

pub fn gnoerf(direction: Direction) -> Npc {
    let look_at = direction;
    let inventory = SelectionStorage::new();

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
    dialog2.insert(Node {
        value: DialogItem {
            text: "Bye".to_string(),
            response: "Goodbye".to_string(),
            action: None
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
    
    Npc {
        name: "Gnoerf".to_string(),
        variant: NpcType::Gnoerf,
        direction,
        look_at,
        dialog: Node {
            value: DialogItem {
                text: "".to_string(),
                response: "...".to_string(),
                action: None
            },
            children: dialog
        },
        inventory
    }
}

pub fn guard(direction: Direction) -> Npc {
    let look_at = direction;
    let inventory = SelectionStorage::new();
    
    let mut dialog = SelectionStorage::new();
    dialog.insert(Node {
        value: DialogItem {
            text: "Hi".to_string(),
            response: "Hello".to_string(),
            action: None
        },
        children: SelectionStorage::new()
    });
    dialog.insert(Node {
        value: DialogItem {
            text: "Bye".to_string(),
            response: "Goodbye".to_string(),
            action: None
        },
        children: SelectionStorage::new()
    });
    
    Npc {
        name: "Guard".to_string(),
        variant: NpcType::Guard,
        direction,
        look_at,
        dialog: Node {
            value: DialogItem {
                text: "".to_string(),
                response: "...".to_string(),
                action: None
            },
            children: dialog
        },
        inventory
    }
}