use bevy::prelude::*;
use hexx::Hex;

use crate::game_parts::plant::PlantKind;

pub mod creature;
pub mod human;
pub mod company;
pub mod building;

#[derive(Component)]
pub struct ScrollableCamera;

#[derive(Component)]
pub struct MoveableCamera;

#[derive(Component)]
pub enum TerrainKind {
    Water,
    Grass,
    Mountain,
    Desert,
}

#[derive(Component)]
pub struct Tile {
    pub hex: Hex,
}

impl Tile {
    pub fn new(hex: Hex) -> Self {
        Tile { hex }
    }
}

#[derive(Component)]
pub struct OccupiesTile {}

#[derive(Component)]
pub struct Plant {
    pub health: f32,
    pub resource_amount: u32,
    pub age: u32,
    pub heritable: PlantHeritable,
}

pub struct PlantHeritable {
    pub kind: PlantKind,
    pub growth_rate: f32,
    pub crop_yield: f32,
    pub temperature_resistance: f32,
    pub disease_resistance: f32,
}

impl PlantHeritable {
    pub fn name(&self) -> String {
        format!(
            "Growth Rate: {}, Crop Yield: {}, Temperature Resistance: {}, Disease Resistance: {}",
            self.growth_rate, self.crop_yield, self.temperature_resistance, self.disease_resistance
        )
    }
}

#[derive(Component)]
pub struct Mine {}
