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

impl Plant {
    pub fn new(hex: Hex) -> Self {
        Plant {
            health: 100.0,
            resource_amount: 0,
            age: 0,
            heritable: PlantHeritable::default(),
        }
    }
}

pub struct PlantHeritable {
    pub kind: PlantKind,
    pub growth_rate: f32,
    pub crop_yield: f32,
    pub temperature_resistance: f32,
    pub disease_resistance: f32,
}

impl Default for PlantHeritable {
    fn default() -> Self {
        PlantHeritable {
            kind: PlantKind::Grass,
            growth_rate: 0.1,
            crop_yield: 0.5,
            temperature_resistance: 0.5,
            disease_resistance: 0.5,
        }
    }
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
