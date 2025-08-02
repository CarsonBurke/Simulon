use bevy::prelude::*;

use crate::{resources::{chunk::Chunks, GameSettings}, systems::{controls::camera::CameraControlsPlugin, creature::CreaturePlugin, init::InitPlugin}};

mod constants;
mod components;
mod resources;
mod systems;
mod game_parts;

fn main() {
    println!("Hello, world!");
    
    App::new()
    .add_plugins((DefaultPlugins, InitPlugin, CameraControlsPlugin, CreaturePlugin))
    // .add_systems(Startup, )
    .insert_resource::<GameSettings>(GameSettings::default())
    .insert_resource::<Chunks>(Chunks::default())
    .run();
}
