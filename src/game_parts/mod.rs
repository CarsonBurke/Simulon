use std::collections::HashMap;

use bevy::prelude::*;
use uuid::Uuid;

use crate::components::company::Company;

pub mod plant;
pub mod resource;

#[derive(Resource)]
pub struct Companies(pub HashMap<Uuid, Company>);

