use crate::game_parts::plant::PlantKind;

pub enum ResourceKind {
    Apple,
    Wheat,
    Wood,
    Fibre,
}

impl ResourceKind {
    pub fn from_plant_kind(plant_kind: PlantKind) -> Self {
        match plant_kind {
            PlantKind::Apple => ResourceKind::Apple,
            PlantKind::Wheat => ResourceKind::Wheat,
            PlantKind::Tree => ResourceKind::Wood,
            PlantKind::Grass => ResourceKind::Fibre,
        }
    }
}