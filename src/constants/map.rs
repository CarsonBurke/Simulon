use bevy::prelude::*;
use hexx::{HexLayout, HexOrientation};

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
pub const CHUNK_SIZE: u32 = 5;
pub const HEX_LAYOUT: HexLayout = HexLayout {
    scale: HEX_SIZE,
    orientation: HexOrientation::Flat,
    origin: Vec2::new(0., 0.),
};
pub const MAP_RADIUS: u32 = 100;

pub mod terrain_type_noise_thresholds {
    pub const MOUNTAIN: (f64, f64) = (0.15, 1.);
    pub const GRASS: (f64, f64) = (0.0, 0.15);
    pub const WATER: (f64, f64) = (-1., -0.60);
}

pub mod terrain_colors {
    use bevy::prelude::*;

    pub const MOUNTAIN: Color = Color::srgba(0.5, 0.5, 0.5, 0.5);
    pub const GRASS: Color = Color::srgba(0.0, 1.0, 0.0, 0.5);
    pub const WATER: Color = Color::srgba(0.0, 0.0, 1.0, 0.5);
}