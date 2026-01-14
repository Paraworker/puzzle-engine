use crate::assets::meshes::piece::PieceMeshes;
use bevy::{asset::Assets, mesh::Mesh};

pub mod piece;

#[derive(Debug)]
pub struct GameMeshes {
    pub piece: PieceMeshes,
}

impl GameMeshes {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            piece: PieceMeshes::new(meshes),
        }
    }
}
