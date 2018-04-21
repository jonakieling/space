use std::slice;
use scene::LEVEL_SIZE;

#[derive(Clone)]
pub struct PositionLevelStorage<T: Clone> {
    storage: Vec<Option<Box<T>>>
}

impl<T: Clone> PositionLevelStorage<T> {
    pub fn new() -> PositionLevelStorage<T> {
        PositionLevelStorage {
            storage: vec![None; (LEVEL_SIZE * LEVEL_SIZE) as usize]
        }
    }
    
    pub fn get(&self, x: i32, y: i32) -> Option<&Option<Box<T>>> {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
        let position = x + y * LEVEL_SIZE;
            match self.storage.get(position as usize) {
                Some(item) => Some(item),
                None => None
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Option<Box<T>>> {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
        let position = x + y * LEVEL_SIZE;
            match self.storage.get_mut(position as usize) {
                Some(item) => Some(item),
                None => None
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, x: i32, y: i32, item: T) {
        if x <= LEVEL_SIZE && y <= LEVEL_SIZE  {
            let position = x + y * LEVEL_SIZE;
            self.storage.remove(position as usize);
            self.storage.insert(position as usize, Some(Box::new(item)));
        }
    }

    pub fn iter(&self) -> slice::Iter<Option<Box<T>>> {
        self.storage.iter()
    }
}