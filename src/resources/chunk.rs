use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;

use crate::constants::map::CHUNK_SIZE;

#[derive(Resource, Default)]
pub struct Chunks(pub HashMap<Hex, Chunk>);

impl Chunks {
    pub fn get_with_hex(&self, hex: &Hex) -> Option<&Chunk> {
        let chunk = hex.to_lower_res(CHUNK_SIZE);
        self.0.get(&chunk)
    }

    pub fn get_with_hex_mut(&mut self, hex: &Hex) -> Option<&mut Chunk> {
        let chunk = hex.to_lower_res(CHUNK_SIZE);
        self.0.get_mut(&chunk)
    }
}

pub struct Chunk {
    pub hex: Hex,
    pub is_active: bool,
}

impl Chunk {
    pub fn new(hex: Hex) -> Self {
        Self {
            hex,
            is_active: false,
        }
    }
}
