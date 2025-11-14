use std::collections::HashMap;

use bevy::prelude::*;
use uuid::Uuid;

use crate::{components::{company::{Company, EmploymentPosition}, creature::Creature, human::Human}, constants::human::MIN_EMPLOYMENT_SEARCH_DISTANCE, game_parts::Companies, systems::creature::state::CreatureState};

pub struct HumanPlugin;

impl Plugin for HumanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, find_employment);
    }
}

struct ClosestEmployment {
    company_id: Uuid,
    position: EmploymentPosition,
}

fn find_employment(mut humans: Query<(&mut Human, &mut Creature)>, companies: Res<Companies>) {
    
    
    // May want to do a map of employment positions - don't replace existing building map yet
    // let mut positions = HashMap::new();
    
    let mut building_map = HashMap::new();
    for (id, company) in companies.0.iter() {
        for (hex, building) in company.buildings.iter() {
            building_map.insert(hex, building);
        }
    }
    
    for (mut human, mut creature) in humans.iter_mut() {
        let search_distance = MIN_EMPLOYMENT_SEARCH_DISTANCE;
        
        let mut closest_employment = None;
        let mut closest_distance = i32::MAX;
        
        for (id, building) in building_map.iter() {
            let distance = building.building.hex.distance_to(creature.hex);
            if distance >= closest_distance {
               continue; 
            }
            
            let Some(position) = building.building.local_positions.first() else {
                continue;
            };
            
            closest_employment = Some(ClosestEmployment {
                company_id: building.company_id,
                position: position.clone(),
            });
            closest_distance = distance;
        }
        
        if let Some(closest_employment) = closest_employment {
            human.join_company(closest_employment.company_id, closest_employment.position);
            creature.state = CreatureState::Working;
        } else {
            creature.state = CreatureState::Searching;
        }
    }
}

