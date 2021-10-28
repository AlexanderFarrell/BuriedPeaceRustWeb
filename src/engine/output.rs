use crate::engine::EngineComponent;

pub struct Output;

impl EngineComponent for Output {}

impl Output {
    pub fn new() -> Self {
        Output {}
    }
}
