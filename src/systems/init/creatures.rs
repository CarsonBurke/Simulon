use bevy::prelude::*;
use hexx::{hex, shapes};
use rand::Rng;

use crate::{
    components::{
        creature::{Creature, CreatureKind}, human::Human, Tile
    },
    constants::{map::{HEX_LAYOUT, MAP_RADIUS}, z_layers},
    systems::creature,
};

pub fn spawn_creatures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::rng();
    let mut creatures = Vec::new();

    let sprite_human = asset_server.load("sprites/person011.png");

    for hex in shapes::hexagon(hex(0, 0), MAP_RADIUS) {
        let creature_chance = rng.random_bool(0.05);
        if !creature_chance {
            continue;
        }

        let is_human = rng.random_bool(0.5);
        let (species, kind) = if is_human {
            (Human::new(), CreatureKind::Human)
        } else {
            (Human::new(), CreatureKind::Human)
        };

        let hex_world_pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let mut transform = Transform::from_xyz(hex_world_pos.x, hex_world_pos.y, z_layers::CREATURE);
        transform.scale = Vec3::new(4.0, 4.0, 1.0);
        
        creatures.push((
            Creature::new(kind),
            species,
            Sprite::from_image(sprite_human.clone()),
            transform,
        ));
    }
    
    commands.spawn_batch(creatures);
}
