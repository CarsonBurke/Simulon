use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game_parts::Companies, resources::{GameSettings, chunk::Chunks}, systems::{
        controls::camera::CameraControlsPlugin, creature::CreaturePlugin, human::HumanPlugin,
        init::InitPlugin,
    }
};

mod components;
mod constants;
mod game_parts;
mod resources;
mod systems;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            InitPlugin,
            CameraControlsPlugin,
            CreaturePlugin,
            HumanPlugin,
        ))
        // .add_systems(Startup, )
        .insert_resource::<GameSettings>(GameSettings::default())
        .insert_resource::<Chunks>(Chunks::default())
        .insert_resource::<Companies>(Companies(HashMap::new()))
        .run();
}
