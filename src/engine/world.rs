use crate::engine::EngineComponent;
use std::collections::BTreeSet;
use std::rc::Rc;

pub trait Component {

}

pub struct ComponentController<T: Component> {
    
}

pub struct Entity {
    pub states: TypeMap<>;
}

// impl Entity {
//     pub fn create() -> Rc<Entity> {
//         Rc::new(Entity {0})
//     }
// }



// pub struct States {
//
// }
//
// pub struct Entity {
//
// }
//
// pub struct Realm {
//     entities: BTreeSet<Rc<Entity>>,
// }
//
// impl Realm {
//     pub fn new() -> Realm {
//         Realm {
//             entities: BTreeSet::new(),
//         }
//     }
//
//     pub fn create_entity(&mut self) -> Rc<Entity> {
//         let entity = Rc::new(Entity {});
//         self.entities.insert(entity.clone());
//         entity
//     }
//
//     pub fn remove_entity(&mut self) -> Rc<Entity> {
//
//     }
// }

pub struct World {

}

impl EngineComponent for World {}

impl World {
    pub fn new() -> Self {
        World {}
    }
}
