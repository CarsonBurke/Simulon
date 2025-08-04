use hexx::Hex;
use bevy::prelude::*;

#[derive(Component)]
pub struct Building {
    pub name: String,
    pub hex: Hex,
    pub stories: u32,
    pub slots_total: u32,
    pub used_slots: u32,
}

impl Building {
    pub fn new(name: String, hex: Hex) -> Self {
        Self { name, hex, stories: 1, slots_total: 10, used_slots: 0 }
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
