use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;

#[derive(Component)]
pub struct Human {
    pub entity: Entity,
    pub last_name: String,
    pub first_name: String,
    pub employment: Option<Employment>,
    pub school: Option<School>,
    /**
     * How much money the human has. May still be positive even if they are in debt.
     */
    pub money: f32,
    pub debt: f32,
    pub residence: Option<Residence>,
    pub employable_skills: HashMap<EmployableSkill, f32>,
}

impl Human {
    pub fn new() -> Self {
        Self {
            entity: Entity::from_raw(0),
            last_name: String::new(),
            first_name: String::new(),
            employment: None,
            school: None,
            money: 0.0,
            debt: 0.0,
            residence: None,
            employable_skills: HashMap::new(),
        }
    }
}

pub struct Residence {
    pub hex: Hex,
    pub owner: Entity,
}

impl Residence {
    pub fn is_owner(&self, entity: Entity) -> bool {
        self.owner == entity
    }
}

pub struct Employment {
    pub company_id: u32,
    pub wage: u32,
}

pub struct School {
    pub company: Entity,
    pub program: SchoolProgram,
    /**
     * How many years they are into their program.
     */
    pub program_year: u32,
    /**
     * How many hours they are at school each day, on average.
     * For example if a student is working part-time, they might only do 5 hours per day.
     */
    pub daily_hours: u32,
}

pub enum SchoolProgram {
    Kindergarten,
    Elementary,
    Highschool,
    /**
     * Above a high school diploma.
     */
    Undergraduate,
    /**
     * Above a bachelor's degree.
     */
    Postgraduate,
}

pub enum SchoolFocus {
    General,
    
}

pub enum EmployableSkill {
    Administration,
    Logistics,
    Finance,
    Economics,
    Chemistry,
    Biology,
    ComputerScience,
    Carpentry,
    Metalworking,
    
}