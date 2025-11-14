use bevy::{camera::ScalingMode, math::VectorSpace, prelude::*};

use crate::{
    components::{MoveableCamera, ScrollableCamera},
    systems::init::{creatures::spawn_creatures, plants::spawn_plants, terrain::generate_terrain, tiles::generate_tiles},
};

pub mod tiles;
pub mod creatures;
pub mod terrain;
pub mod plants;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cameras, generate_tiles, generate_terrain, spawn_creatures, spawn_plants));
    }
}

const HEIGHT: f32 = 200.;

pub fn spawn_cameras(mut commands: Commands, _window: Single<&Window>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., HEIGHT, 500.).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: HEIGHT,
                },
            ..OrthographicProjection::default_3d()
        }),
        ScrollableCamera,
        MoveableCamera,
    ));
}
