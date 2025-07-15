use crate::assets::meshes::piece::PieceMeshes;
use bevy::{asset::Assets, render::mesh::Mesh};

pub mod piece;

#[derive(Debug)]
pub struct GameMeshes {
    piece: PieceMeshes,
}

impl GameMeshes {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            piece: PieceMeshes::new(meshes),
        }
    }

    pub fn piece(&self) -> &PieceMeshes {
        &self.piece
    }
}
