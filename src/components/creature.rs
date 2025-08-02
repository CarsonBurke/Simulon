use bevy::prelude::*;

#[derive(Component)]
pub struct Creature {
    pub health: f32,
    pub heritable: CreatureHeritable,
    pub age: u32,
    pub maturity: f32,
    pub knowledge: f32,
    pub intelligence: f32,
    pub charisma: f32,
}

impl Creature {
    pub fn attractiveness(&self) -> f32 {
        self.intelligence
            + self.charisma
            + self.knowledge
            + self.heritable.children_desire
    }
}

pub struct CreatureHeritable {
    pub kind: CreatureKind,
    /**
     * For every increase in age, increase the chance of death by this amount.
     */
    pub age_death_chance: f32,
    pub maturity_rate: f32,
    pub age_rate: f32,
    pub aggression: f32,
    pub intelligence: f32,
    pub charisma: f32,
    pub material_desire: f32,
    pub social_desire: f32,
    pub food_desire: f32,
    pub children_desire: f32,
}

pub enum CreatureKind {
    Human,
    Sheep,
    Pig,
    Cow,
    Fish,
}