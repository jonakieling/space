use std::cmp::max;
use std::slice;
use constants::LEVEL_SIZE;

#[derive(Clone)]
pub struct PositionLevelStorage<T: Clone> {
    storage: Box<Vec<Option<T>>>
}

impl<T: Clone> PositionLevelStorage<T> {
    pub fn new() -> PositionLevelStorage<T> {
        PositionLevelStorage {
            storage: Box::new(vec![None; (LEVEL_SIZE * LEVEL_SIZE) as usize])
        }
    }
    
    pub fn get(&self, x: i32, y: i32) -> Option<&Option<T>> {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
            let position = x + y * LEVEL_SIZE;
            self.storage.get(position as usize)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Option<T>> {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
            let position = x + y * LEVEL_SIZE;
            self.storage.get_mut(position as usize)
        } else {
            None
        }
    }

    pub fn insert(&mut self, x: i32, y: i32, item: T) {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
            let position = x + y * LEVEL_SIZE;
            self.storage.remove(position as usize);
            self.storage.insert(position as usize, Some(item));
        }
    }

    pub fn remove(&mut self, x: i32, y: i32) {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
            let position = x + y * LEVEL_SIZE;
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
        } else {
            self.current_selection = max(0, self.storage.len() - 1);
        }
        self.current()
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.current_selection < self.storage.len() - 1 {
            self.current_selection += 1;
        } else {
            self.current_selection = 0;
        }
        self.current()
    }

    pub fn current(&mut self) -> Option<&T> {
        self.storage.get(self.current_selection)
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
}