pub trait EngineComponent {}

pub struct Input;
pub struct Output;
pub struct World;
pub struct Player;

impl EngineComponent for Input {}
impl EngineComponent for Output {}
impl EngineComponent for World {}
impl EngineComponent for Player {}

pub struct Engine {
    pub input: Input,
    pub output: Output,
    pub world: World,
    pub player: Player,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            input: Input::new(),
            output: Output::new(),
            world: World::new(),
            player: Player::new(),
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Input {}
    }
}

impl Output {
    pub fn new() -> Self {
        Output {}
    }
}

impl World {
    pub fn new() -> Self {
        World {}
    }
}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}