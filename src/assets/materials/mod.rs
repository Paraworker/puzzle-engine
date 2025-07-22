use crate::assets::materials::{common::CommonMaterials, piece::PieceMaterials};
use bevy::{asset::Assets, pbr::StandardMaterial};

pub mod common;
pub mod piece;

#[derive(Debug)]
pub struct GameMaterials {
    pub common: CommonMaterials,
    pub piece: PieceMaterials,
}

impl GameMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            common: CommonMaterials::new(materials),
            piece: PieceMaterials::new(materials),
        }
    }
}
