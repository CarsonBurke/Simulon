use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
pub struct Company {
    pub name: String,
    pub executive: Entity,
    pub employees: Vec<Entity>,
    pub shareholders: HashMap<Entity, Shareholder>,
}

pub struct Shareholder {
    pub ownership_percent: f32,
    pub human: Entity,
}

