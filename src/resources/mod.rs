use bevy::prelude::*;

pub mod chunk;

#[derive(Resource)]
pub struct GameSettings {
    pub lights: bool
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            lights: true,
        }
    }
}