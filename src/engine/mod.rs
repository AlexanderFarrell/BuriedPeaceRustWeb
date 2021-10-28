use input::Input;
use output::Output;
use player::Player;
use world::World;
use crate::engine::player::{PlayerInfo, PlayerPlatform};

mod player;
mod world;
mod input;
mod output;

pub trait EngineComponent {}

pub struct Engine<PI: PlayerInfo, PL: PlayerPlatform> {
    pub player: Player<PI, PL>,
    pub world: World,
    pub input: Input,
    pub output: Output,
}

impl<PI: PlayerInfo, PL: PlayerPlatform> Engine<PI, PL> {
    pub fn new() -> Engine<PI, PL> {
        Engine {
            player: Player::new(),
            world: World::new(),
            input: Input::new(),
            output: Output::new(),
        }
    }
}
