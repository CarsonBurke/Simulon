use bevy::prelude::*;
use hexx::Hex;
use rand::Rng;

use crate::constants::creature::heritable_initial_ranges;

#[derive(Component)]
pub struct Creature {
    pub hex: Hex,
    pub health: f32,
    pub heritable: CreatureHeritable,
    pub age: u32,
    pub maturity: f32,
    pub knowledge: f32,
    pub intelligence: f32,
    pub charisma: f32,
    pub kind: CreatureKind,
}

impl Creature {
    pub fn new(hex: Hex, kind: CreatureKind) -> Self {
        let heritable = CreatureHeritable::new_random();
        Self {
            hex,
            health: heritable.max_health,
            heritable,
            age: 0,
            maturity: 0.0,
            knowledge: 0.0,
            intelligence: 0.0,
            charisma: 0.0,
            kind,
        }
    }

    pub fn attractiveness(&self) -> f32 {
        self.intelligence + self.charisma + self.knowledge + self.heritable.children_desire
    }
}

#[derive(Clone)]
pub struct CreatureHeritable {
    pub kind: CreatureKind,
    /// For every increase in age, increase the chance of death by this amount.
    pub age_death_chance: f32,
    pub max_health: f32,
    pub maturity_rate: f32,
    pub aggression: f32,
    pub intelligence: f32,
    pub charisma: f32,
    pub material_desire: f32,
    pub social_desire: f32,
    pub food_desire: f32,
    pub children_desire: f32,
}

impl CreatureHeritable {
    pub fn new_random() -> Self {
        let mut rng = rand::rng();

        Self {
            kind: CreatureKind::Human,
            age_death_chance: rng.random_range(
                heritable_initial_ranges::AGE_DEATH_CHANCE.0
                    ..heritable_initial_ranges::AGE_DEATH_CHANCE.1,
            ),
            max_health: rng.random_range(
                heritable_initial_ranges::MAX_HEALTH.0..heritable_initial_ranges::MAX_HEALTH.1,
            ),
            maturity_rate: rng.random_range(
                heritable_initial_ranges::MATURITY_RATE.0
                    ..heritable_initial_ranges::MATURITY_RATE.1,
            ),
            aggression: rng.random_range(
                heritable_initial_ranges::AGGRESSION.0..heritable_initial_ranges::AGGRESSION.1,
            ),
            intelligence: rng.random_range(
                heritable_initial_ranges::INTELLIGENCE.0..heritable_initial_ranges::INTELLIGENCE.1,
            ),
            charisma: rng.random_range(
                heritable_initial_ranges::CHARISMA.0..heritable_initial_ranges::CHARISMA.1,
            ),
            material_desire: rng.random_range(
                heritable_initial_ranges::MATERIAL_DESIRE.0
                    ..heritable_initial_ranges::MATERIAL_DESIRE.1,
            ),
            social_desire: rng.random_range(
                heritable_initial_ranges::SOCIAL_DESIRE.0
                    ..heritable_initial_ranges::SOCIAL_DESIRE.1,
            ),
            food_desire: rng.random_range(
                heritable_initial_ranges::FOOD_DESIRE.0..heritable_initial_ranges::FOOD_DESIRE.1,
            ),
            children_desire: rng.random_range(
                heritable_initial_ranges::CHILDREN_DESIRE.0
                    ..heritable_initial_ranges::CHILDREN_DESIRE.1,
            ),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CreatureKind {
    Human,
    Sheep,
    Pig,
    Cow,
    Fish,
}
