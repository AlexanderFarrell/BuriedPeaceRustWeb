use crate::engine::EngineComponent;

pub struct Player<Info: PlayerInfo, Platform: PlayerPlatform> {
    info: Option<Info>,
    platform: Platform,
}

impl<I: PlayerInfo,
    Pl: PlayerPlatform> EngineComponent for Player<I, Pl> {}

impl<Info: PlayerInfo,
    Platform: PlayerPlatform> Player<Info, Platform> {
    pub fn new() -> Self {
        Player {
            info: None,
            platform: PlayerPlatform::create_platform(),
        }
    }

    pub fn get_info(&self) -> &Option<Info> {
        &self.info
    }

    pub fn get_info_mut(&mut self) -> &mut Option<Info> {
        &mut self.info
    }

    pub fn get_platform(&self) -> &Platform {
        &self.platform
    }

    pub fn login(&mut self, info: Info) {
        self.info = Some(info)
    }

    pub fn logout(&mut self) {
        self.info = None
    }

    pub fn is_logged_in(&self) -> bool {
        self.info.is_some()
    }
}

pub trait PlayerInfo {}

pub trait PlayerPlatform {
    fn create_platform<T: PlayerPlatform>() -> T;
}
