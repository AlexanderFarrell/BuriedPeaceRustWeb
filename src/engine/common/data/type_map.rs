use std::collections::HashMap;
use crate::wasm_bindgen::__rt::std::collections::hash_map::Iter;
use std::marker::PhantomData;
use std::any::{TypeId, Any};
use downcast_rs::Downcast;
use std::collections::hash_map::Iter;

pub struct TypeMap {
    adj: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: impl Any + Downcast) -> Option<Box<dyn Any>> {
        self.adj.insert(item.type_id(), Box::new(item))
    }

    pub fn remove<T: Any>(&mut self) -> Option<Box<dyn Any>> {
        self.adj.remove(&TypeId::of::<T>())
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        match self.adj.get(&TypeId::of::<T>()) {
            None => {None}
            Some(item) => {
                item.downcast_ref::<T>()
            }
        }
    }

    pub fn has<T: Any>(&self) -> bool {
        self.adj.contains_key(&TypeId::of::<T>())
    }

    pub fn iter(&self) -> Iter<'_, TypeId, Box<dyn Any>> {
        self.adj.iter()
    }

    pub fn len(&self) -> usize {
        self.adj.len()
    }

    pub fn is_empty(&self) -> bool {
        self.adj.is_empty()
    }
}

#[cfg(test)]
mod kind_map_tests {
    use crate::engine_old::data::kind_map::KindMap;

    fn create() -> KindMap{
        let mut i = KindMap::new();
        i.add(145 as i32);
        i
    }

    #[test]
    fn add_and_has() {
        assert!(create().has::<i32>())
    }

    #[test]
    fn add_and_get() {
        assert_eq!(create().get::<i32>().unwrap().clone(), 145)
    }

    #[test]
    fn size() {
        assert_eq!(create().len(), 1)
    }

    #[test]
    fn is_empty() {
        assert_eq!(create().is_empty(), false)
    }

    #[test]
    fn is_empty_new() {
        assert_eq!(KindMap::new().is_empty(), true)
    }

    #[test]
    fn is_empty_remove() {
        let mut i = create();
        i.remove::<i32>();
        assert_eq!(i.is_empty(), true)
    }

    #[test]
    fn doesnt_have_anymore() {
        let mut i = create();
        i.remove::<i32>();
        assert!(!i.has::<i32>())
    }

    enum SomeVariant {
        Something,
        SomethingElse,
        YetSomethingElse,
    }

    #[test]
    fn is_none() {
        let mut i = create();
        i.remove::<i32>();
        assert!(i.get::<i32>().is_none())
    }

    #[test]
    fn variant_type_is_enum() {
        let mut i = KindMap::new();
        i.add(SomeVariant::Something);
        assert!(i.has::<SomeVariant>())
    }
}
















