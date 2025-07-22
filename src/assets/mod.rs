use crate::assets::{materials::GameMaterials, meshes::GameMeshes};
use bevy::{asset::Assets, ecs::resource::Resource, pbr::StandardMaterial, render::mesh::Mesh};

pub mod materials;
pub mod meshes;

#[derive(Debug, Resource)]
pub struct GameAssets {
    pub materials: GameMaterials,
    pub meshes: GameMeshes,
}

impl GameAssets {
    pub fn new(materials: &mut Assets<StandardMaterial>, meshes: &mut Assets<Mesh>) -> Self {
        Self {
            materials: GameMaterials::new(materials),
            meshes: GameMeshes::new(meshes),
        }
    }
}
