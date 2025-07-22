use crate::piece::PieceModel;
use bevy::{
    asset::{Assets, Handle},
    math::primitives::{Cuboid, Cylinder, Sphere},
    platform::collections::HashMap,
    render::mesh::Mesh,
};

#[derive(Debug)]
pub struct PieceMeshes {
    map: HashMap<PieceModel, Handle<Mesh>>,
}

impl PieceMeshes {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            map: HashMap::from([
                (PieceModel::Cube, meshes.add(Cuboid::default())),
                (PieceModel::Sphere, meshes.add(Sphere::default())),
                (PieceModel::Cylinder, meshes.add(Cylinder::default())),
            ]),
        }
    }

    pub fn get(&self, model: PieceModel) -> &Handle<Mesh> {
        self.map.get(&model).expect("No mesh found")
    }
}
