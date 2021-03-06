use specs::{VecStorage, DenseVecStorage};

use storage::{SelectionStorage, Node};
use dialog::DialogItem;
use misc::Direction;
use objects::Item;
use app::SpriteId;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Face {
    pub direction: Direction
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Name {
    pub name: String
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Inventory {
    pub parts: SelectionStorage<Item>
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Dialog {
    pub tree: Node<DialogItem>
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Tile {
    pub sprite: SpriteId
}
