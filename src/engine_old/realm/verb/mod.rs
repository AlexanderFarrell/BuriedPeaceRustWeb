use crate::engine::realm::Noun;

pub struct Verb {
    pub start: Box<dyn Fn(Vec<Noun>)>,
    pub end: Box<dyn Fn(Vec<Noun>)>,
}