use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;

use crate::components::building::Building;

#[derive(Component)]
pub struct Company {
    pub name: String,
    pub executive: Entity,
    pub employees: Vec<Entity>,
    pub shareholders: HashMap<Entity, Shareholder>,
    pub buildings: HashMap<Hex, Building>,
}

pub struct Shareholder {
    pub ownership_percent: f32,
    pub human: Entity,
}

