use hexx::Hex;
use bevy::prelude::*;
use uuid::Uuid;

use crate::components::building::Building;

#[derive(Component)]
pub struct CompanyBuilding {
    pub company_id: Uuid,
    pub building: Building,
}

impl CompanyBuilding {
    pub fn new(name: String, company_id: Uuid, hex: Hex) -> Self {
        Self { company_id, building: Building::new(name, company_id, hex), }
    }
}
