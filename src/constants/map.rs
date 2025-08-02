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

pub mod resource_noise_tresholds {
    pub const MOUNTAIN: (f64, f64) = (0.15, 1.);
    pub const GRASS: (f64, f64) = (0.0, 0.15);
    pub const WATER: (f64, f64) = (-1., -0.60);
}