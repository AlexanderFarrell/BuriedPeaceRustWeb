use crate::engine::EngineComponent;

pub struct Input;

impl EngineComponent for Input {}

impl Input {
    pub fn new() -> Self {
        Input {}
    }
}
