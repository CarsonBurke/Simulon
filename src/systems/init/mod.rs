use bevy::{prelude::*, render::camera::Viewport};

use crate::{
    components::{MoveableCamera, ScrollableCamera},
    systems::init::{creatures::spawn_creatures, terrain::generate_terrain, tiles::generate_tiles},
};

pub mod tiles;
pub mod creatures;
pub mod terrain;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cameras, generate_tiles, generate_terrain, spawn_creatures));
    }
}

pub fn spawn_cameras(mut commands: Commands, window: Single<&Window>) {
    commands.spawn((
        Camera2d,
        Camera { ..default()  },
        ScrollableCamera,
        MoveableCamera,
    ));
}
