use std::cmp::max;
use std::fmt::Debug;
use std::slice;
use dialog::DialogItem;
use misc::Position;

#[derive(Clone)]
pub struct PositionLevelStorage<T: Clone> {
    storage: Box<Vec<Option<T>>>
}

impl<T: Clone + Debug> PositionLevelStorage<T> {
    pub fn new() -> PositionLevelStorage<T> {
        PositionLevelStorage {
            storage: Box::new(Vec::new())
        }
    }
    
    pub fn get(&self, pos: Position) -> Option<&T> {
        let position = pos.to_int();
        if position < self.storage.len() as i32 {
            if let Some(&Some(ref item)) = self.storage.get(position as usize) {
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        let position = pos.to_int();
        if position < self.storage.len() as i32 {
            if let Some(&mut Some(ref mut item)) = self.storage.get_mut(position as usize) {
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, pos: Position, item: T) {
        let position = pos.to_int();
        if position < self.storage.len() as i32 {
            self.storage.remove(position as usize);
            self.storage.insert(position as usize, Some(item));
        } else {
            self.storage.resize(position as usize, None);
            self.storage.insert(position as usize, Some(item));
        }
    }

    pub fn remove(&mut self, pos: Position) {
        let position = pos.to_int();
        if position < self.storage.len() as i32 {
            self.storage.remove(position as usize);
            self.storage.insert(position as usize, None);
        }
    }

    pub fn iter(&self) -> slice::Iter<Option<T>> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Option<T>> {
        self.storage.iter_mut()
    }

    pub fn get_neighbors_at(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if let Some(_) = self.get(pos.x_up()) {
            neighbors.push(pos.x_up());
        }
        if let Some(_) = self.get(pos.x_down()) {
            neighbors.push(pos.x_down());
        }
        if let Some(_) = self.get(pos.y_up()) {
            neighbors.push(pos.y_up());
        }
        if let Some(_) = self.get(pos.y_down()) {
            neighbors.push(pos.y_down());
        }

        neighbors
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelectionStorage<T: Clone> {
    storage: Vec<T>,
    current_selection: usize
}

impl<T: Clone> SelectionStorage<T> {
    pub fn new() -> SelectionStorage<T> {
        SelectionStorage {
            storage: Vec::new(),
            current_selection: 0
        }
    }

    pub fn prev(&mut self) -> Option<&T> {
        if self.current_selection > 0 {
            self.current_selection -= 1;
        } else if self.storage.len() != 0 {
            self.current_selection = max(0, self.storage.len() - 1);
        }
        self.current()
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.current_selection + 1 < self.storage.len() {
            self.current_selection += 1;
        } else {
            self.current_selection = 0;
        }
        self.current()
    }

    pub fn current(&mut self) -> Option<&T> {
        if self.storage.len() != 0 {
            self.storage.get(self.current_selection)
        } else {
            None
        }
    }

    pub fn extract_current(&mut self) -> Option<T> {
        if self.storage.len() != 0 {
            let item = self.storage.get(self.current_selection).unwrap().clone();
            self.storage.remove(self.current_selection);

            if self.storage.len() <= self.current_selection && self.current_selection > 0 {
                self.current_selection -= 1;
            }

            Some(item)
        } else {
            None
        }
    }

    pub fn current_index(&self) -> usize {
        self.current_selection
    }

    pub fn insert(&mut self, item: T) {
        self.storage.push(item);
    }

    pub fn iter(&self) -> slice::Iter<T> {
        self.storage.iter()
    }

    pub fn clear(&mut self) {
        self.storage.clear()
    }

    pub fn storage(&self) -> Vec<T> {
        self.storage.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tree<T: Clone> {
    pub root: Node<T>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Node<T: Clone> {
    pub value: T,
    pub children: SelectionStorage<Node<T>>
}

impl ToString for Node<DialogItem> {
    fn to_string(&self) -> String {
        self.value.text.clone()
    }
}