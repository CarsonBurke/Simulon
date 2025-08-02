use bevy::{prelude::*};
use libnoise::prelude::*;
use lazy_static::lazy_static;

use crate::{components::{TerrainKind, Tile}, constants::map::resource_noise_tresholds};

lazy_static! {
    pub static ref SIMPLEX_GENERATOR: Blend<2, Fbm<2, Simplex<2>>, Scale<2, Worley<2>>, Scale<2, Worley<2>>> = Source::simplex(43)                 // start with simplex noise
    .fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
    .blend(                                         // apply blending...
        Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
        Source::worley(44).scale([0.01, 0.01]));     // ...controlled by other worley noise
}

pub fn generate_terrain(tiles: Query<(&Tile, Entity)>, mut commands: Commands) {
    for (tile, entity) in tiles.iter() {
        
        let noise = SIMPLEX_GENERATOR.sample([
            tile.hex.x as f64,
            tile.hex.y as f64, /* hex.x as f64, hex.y as f64 */
        ]);
        
        if noise > resource_noise_tresholds::WATER.0 && noise < resource_noise_tresholds::WATER.1 {
            commands.entity(entity).insert(TerrainKind::Water);
            continue;
        }
        
        if noise > resource_noise_tresholds::MOUNTAIN.0 && noise < resource_noise_tresholds::MOUNTAIN.1 {
            commands.entity(entity).insert(TerrainKind::Mountain);
            continue;
        }
        
        commands.entity(entity).insert(TerrainKind::Grass);
    }
}