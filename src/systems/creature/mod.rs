use bevy::prelude::*;

use crate::{components::creature::Creature, constants::{self, creature::MAX_MATURITY}};

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mature);
    }
}

pub fn mature(mut creatures: Query<&mut Creature>) {
    for mut creature in creatures.iter_mut() {
        if creature.maturity >= constants::creature::MAX_MATURITY {
            continue
        }
        
        creature.maturity = (creature.maturity + 1.0).max(MAX_MATURITY);
    }
}

pub fn age(mut creatures: Query<(&mut Creature, Entity)>, mut commands: Commands) {
    
    let mut dead_entities = Vec::new();
    
    for (mut creature, entity) in creatures.iter_mut() {
        
        let death_chance = creature.heritable.age_death_chance * creature.age as f32;
        let should_die = rand::random::<f32>() <= death_chance;
        
        if should_die {
            creature.health = 0.0;
            dead_entities.push(entity);
            continue;
        }
        
        creature.age += 1;
    }
    
    for entity in dead_entities {
        commands.entity(entity).despawn();
    }
}
