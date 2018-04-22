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
}