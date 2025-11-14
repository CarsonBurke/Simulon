use bevy::{pbr::StandardMaterial, prelude::*};
use hexx::{hex, shapes};
use lazy_static::lazy_static;
use libnoise::prelude::*;

use crate::{
    components::TerrainKind,
    constants::{map::{terrain_colors, terrain_type_noise_thresholds, HEX_LAYOUT, MAP_RADIUS}, z_layers},
    systems::init::tiles::hexagonal_plane,
};

lazy_static! {
    pub static ref SIMPLEX_GENERATOR: Blend<2, Fbm<2, Simplex<2>>, Scale<2, Worley<2>>, Scale<2, Worley<2>>> = Source::simplex(43)                 // start with simplex noise
    .fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
    .blend(                                         // apply blending...
        Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
        Source::worley(44).scale([0.01, 0.01]));     // ...controlled by other worley noise
}

pub fn generate_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let material_mountain = materials.add(StandardMaterial::from(terrain_colors::MOUNTAIN));
    let material_grass = materials.add(StandardMaterial::from(terrain_colors::GRASS));
    let material_water = materials.add(StandardMaterial::from(terrain_colors::WATER));

    for hex in shapes::hexagon(hex(0, 0), MAP_RADIUS) {
        let noise = SIMPLEX_GENERATOR.sample([
            hex.x as f64,
            hex.y as f64, /* hex.x as f64, hex.y as f64 */
        ]);
        
        let hex_world_pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let transform = Transform::from_xyz(hex_world_pos.x as f32, hex_world_pos.y as f32, z_layers::TERRAIN);

        if noise > terrain_type_noise_thresholds::WATER.0
            && noise < terrain_type_noise_thresholds::WATER.1
        {
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_water.clone()),
                transform,
                TerrainKind::Water,
            ));
            continue;
        }

        if noise > terrain_type_noise_thresholds::MOUNTAIN.0
            && noise < terrain_type_noise_thresholds::MOUNTAIN.1
        {
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_mountain.clone()),
                transform,
                TerrainKind::Mountain,
            ));
            continue;
        }

        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_grass.clone()),
            transform,
            TerrainKind::Grass,
        ));
    }
}
