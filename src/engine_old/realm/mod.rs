use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

use crate::engine_old::data::id_map::IdMap;
use crate::engine_old::data::kind_map::KindMap;
use crate::wasm_bindgen::__rt::core::any::{Any, TypeId};
use crate::wasm_bindgen::__rt::core::fmt::Formatter;
use crate::wasm_bindgen::__rt::std::collections::hash_map::Iter;

pub mod aspect;
pub mod adj;
mod verb;

// static REALM: Realm = Realm::new();
//
// pub struct Realm {
//     nouns: IdMap<Arc<Mutex<Noun>>>,
//     names_by_noun: HashMap<&'static str, i32>,
// }
//
// impl Realm {
//     pub fn new() -> Self {
//         Self {
//             nouns: IdMap::new(),
//             names_by_noun: HashMap::new()
//         }
//     }
// }

pub struct Noun {
    being: KindMap,
    having: KindMap,
    performing: HashSet<i32>,
    name: Option<&'static str>,
    id: i32,
}

impl Noun {
    pub fn new(name: Option<&'static str>) -> Noun {
        Noun {
            being: KindMap::new(),
            having: KindMap::new(),
            performing: HashSet::new(),
            name,
            id: -1
        }
    }

    pub fn now_is(&mut self, adj: impl Is + 'static){
        self.being.add(adj);
    }
    pub fn now_is_not<T: Is + 'static>(&mut self) {
        self.being.remove::<T>();
    }
    pub fn is<T: Is + 'static>(&mut self) -> Option<&T> {
        self.being.get::<T>()
    }
    pub fn is_assume<T: Is + 'static>(&mut self) -> &T {
        self.being.get::<T>().unwrap()
    }
    pub fn is_bool<T: Is + 'static>(&mut self) -> bool {
        self.being.has::<T>()
    }

    pub fn now_has(&mut self, container: impl Has + 'static) {
        self.having.add(container);
    }
    pub fn now_has_not<T: Has +'static>(&mut self) { self.having.remove::<T>();}
    pub fn has<T: Has + 'static>(&mut self) -> Option<&T> { self.having.get::<T>()}
    pub fn has_assume<T: Has + 'static>(&mut self) -> &T { self.having.get::<T>().unwrap()}
    pub fn has_bool<T: Has + 'static>(&mut self) -> bool { self.having.has::<T>()}

    pub fn describe_name(&self) -> &'static str {
        match self.name {
            None => {
                "unnamed"
            }
            Some(name) => {
                name
            }
        }
    }
}

impl Display for Noun {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.describe_name())
    }
}

#[cfg(test)]
mod realm_noun_tests {
    use crate::engine_old::realm::adj::Aged;
    use crate::engine_old::realm::Noun;

    #[test]
    fn name_noun() {
        let alex = Noun::new(Some("Alex"));
        assert_eq!(alex.name.unwrap(), "Alex")
    }

    #[test]
    fn desc_noun_age() {
        let mut adult = Noun::new(None);
        adult.now_is(Aged::new(0, 9));
        assert_eq!(adult.is::<Aged>().unwrap().years, 40)
    }
}

pub trait Is: Describe + Debug {
    fn noun_word(&self) -> &'static str;
    fn adj_word(&self) -> &'static str;
}

pub trait Has: Describe + Debug {
    fn display(&self, f: &mut Formatter<'_>) -> core::fmt::Result;
}

pub trait Do: Describe + Debug {
    fn display(&self, f: &mut Formatter<'_>) -> core::fmt::Result;
}

pub trait Describe {
    fn describe(&self) -> String;
}

impl Display for dyn Describe {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
