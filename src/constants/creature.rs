pub const MAX_MATURITY: f32 = 100.0;

pub mod heritable_initial_ranges {
    pub const MAX_HEALTH: (f32, f32) = (10.0, 20.0);
    pub const AGE_DEATH_CHANCE: (f32, f32) = (0.01, 1.0);
    pub const MATURITY_RATE: (f32, f32) = (0.1, 1.0);
    pub const MATURITY_SPEED: (f32, f32) = (0.1, 1.0);
    pub const CHARISMA: (f32, f32) = (0.1, 1.0);
    pub const INTELLIGENCE: (f32, f32) = (0.1, 1.0);
    pub const AGGRESSION: (f32, f32) = (0.1, 1.0);
    pub const MATERIAL_DESIRE: (f32, f32) = (0.1, 1.0);
    pub const SOCIAL_DESIRE: (f32, f32) = (0.1, 1.0);
    pub const FOOD_DESIRE: (f32, f32) = (0.1, 1.0);
    pub const CHILDREN_DESIRE: (f32, f32) = (0.1, 1.0);
}

pub mod heritable_bounds {
    pub const MAX_HEALTH: (f32, f32) = (2.0, 200.0);
    pub const AGE_DEATH_CHANCE: (f32, f32) = (0.001, 5.0);
    pub const MATURITY_RATE: (f32, f32) = (0.01, 10.0);
    pub const MATURITY_SPEED: (f32, f32) = (0.01, 10.0);
    pub const CHARISMA: (f32, f32) = (0.01, 10.0);
    pub const INTELLIGENCE: (f32, f32) = (0.01, 10.0);
    pub const AGGRESSION: (f32, f32) = (0.01, 10.0);
    pub const MATERIAL_DESIRE: (f32, f32) = (0.01, 10.0);
    pub const SOCIAL_DESIRE: (f32, f32) = (0.01, 10.0);
    pub const FOOD_DESIRE: (f32, f32) = (0.01, 10.0);
    pub const CHILDREN_DESIRE: (f32, f32) = (0.01, 10.0);
}
