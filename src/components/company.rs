use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;
use uuid::Uuid;

use crate::components::{company_building::CompanyBuilding, human::Employable};

pub struct Company {
    pub name: String,
    pub id: Uuid,
    pub executive: Entity,
    pub employees: Vec<Entity>,
    pub global_positions: Vec<EmploymentPosition>,
    pub shareholders: HashMap<Uuid, Shareholder>,
    pub buildings: HashMap<Hex, CompanyBuilding>,
}

impl Company {
    pub fn new(name: String, executive: Entity) -> Self {
        
        let mut shareholders = HashMap::new();
        let share_id = Uuid::new_v4();
        shareholders.insert(share_id, Shareholder {
            ownership_percent: 1.0,
            human: executive,
        });
        
        let mut employees = Vec::new();
        employees.push(executive);
        
        Company {
            name,
            id: Uuid::new_v4(),
            executive,
            employees,
            global_positions: Vec::new(),
            shareholders,
            buildings: HashMap::new(),
        }
    }
}

pub struct Shareholder {
    pub ownership_percent: f32,
    pub human: Entity,
}

#[derive(Clone)]
pub struct EmploymentPosition {
    pub starting_wage: u32,
    pub job: Employable,
    pub building: Hex,
}