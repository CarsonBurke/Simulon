use bevy::prelude::*;
use hexx::{hex, shapes};
use rand::{Rng, seq::IndexedRandom};

use crate::{components::Plant, constants::{map::{HEX_LAYOUT, MAP_RADIUS}, z_layers}};

pub fn spawn_plants(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut plants = Vec::new();
    let mut rng = rand::rng();

    let mut plant_assets: Vec<Handle<Image>> = Vec::new();

    for i in 1..=74 {
        plant_assets.push(asset_server.load(format!("plants/{}.png", i)));
    }

    for hex in shapes::hexagon(hex(0, 0), MAP_RADIUS) {
        let plant_chance = rng.random_bool(0.05);
        if !plant_chance {
            continue;
        }
        
        let hex_world_pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let mut transform = Transform::from_xyz(hex_world_pos.x, hex_world_pos.y, z_layers::PLANT);
        transform.scale = Vec3::new(4.0, 4.0, 1.0);

        plants.push((
            Plant::new(hex),
            Sprite::from_image(plant_assets.choose(&mut rng).unwrap().clone()),
            transform,
        ));
    }
    commands.spawn_batch(plants);
}
