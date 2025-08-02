use bevy::prelude::*;

use crate::{resources::{chunk::Chunks, GameSettings}, systems::{controls::camera::CameraControlsPlugin, init::InitPlugin}};

mod constants;
mod components;
mod resources;
mod systems;

fn main() {
    println!("Hello, world!");
    
    App::new()
    .add_plugins((DefaultPlugins, InitPlugin, CameraControlsPlugin))
    // .add_systems(Startup, )
    .insert_resource::<GameSettings>(GameSettings::default())
    .insert_resource::<Chunks>(Chunks::default())
    .run();
}
