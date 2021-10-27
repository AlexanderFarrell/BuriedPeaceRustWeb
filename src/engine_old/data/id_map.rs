use std::collections::HashMap;
use std::collections::hash_map::Iter;

pub struct Counter {
    i: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            i: -1
        }
    }

    pub fn next(&mut self) -> i32 {
        self.i += 1;
        self.i.clone()
    }

    pub fn next_removed(){
        todo!()
    }
}

pub struct IdMap<T> {
    items: HashMap<i32, T>,
    counter:  Counter
}

impl<T> IdMap<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            counter:  Counter::new(),
        }
    }

    pub fn add(&mut self, t: T) -> i32{
        let i = self.counter.next();
        self.items.insert(i.clone(), t);
        i
    }

    pub fn has(&self, i: &i32) -> bool {
        self.items.contains_key(i)
    }

    pub fn iter(&self) -> Iter<'_, i32, T> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn remove(&mut self, i: &i32) {
        self.items.remove(&i);
    }

    pub fn borrow(&self, i: &i32) -> Option<&T> {
        self.items.get(i)
    }

    pub fn borrow_mut(&mut self, i: &i32) -> Option<&mut T> {
        self.items.get_mut(i)
    }
}

#[cfg(test)]
mod id_map_tests {
    use crate::engine_old::data::id_map::IdMap;

    #[test]
    pub fn add_id_map(){
        let mut map = IdMap::new();
        let id = map.add(32);
        assert_eq!(map.borrow(&id).unwrap().clone(), 32)
    }
}