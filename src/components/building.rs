use hexx::Hex;
use bevy::prelude::*;
use uuid::Uuid;

use crate::components::company::EmploymentPosition;

#[derive(Component)]
pub struct Building {
    pub name: String,
    pub company_id: Uuid,
    pub hex: Hex,
    pub stories: u32,
    pub slots_total: u32,
    pub used_slots: u32,
    pub local_positions: Vec<EmploymentPosition>,
}

impl Building {
    pub fn new(name: String, company_id: Uuid, hex: Hex) -> Self {
        Self { name, company_id, hex, stories: 1, slots_total: 10, used_slots: 0, local_positions: Vec::new() }
    }
    
    pub fn remaining_slots(&self) -> u32 {
        self.slots_total - self.used_slots
    }
    
    pub fn add_story(&mut self) {
        self.stories += 1;
        self.slots_total += 10;
    }
    
    pub fn remove_story(&mut self) {
        if self.stories > 1 {
            self.stories -= 1;
            self.slots_total -= 10;
        }
    }
}
