pub mod map;
pub mod camera;
pub mod creature;
pub mod laws;
pub mod building;
pub mod human;

pub mod z_layers {
    pub const BACKGROUND: f32 = -1.;
    pub const TERRAIN: f32 = 0.;
    pub const CREATURE: f32 = 100.;
    pub const BUILDING: f32 = 2.;
    pub const RESOURCE: f32 = 3.;
    pub const PLANT: f32 = 3.;
}