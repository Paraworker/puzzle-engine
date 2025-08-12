use bevy::{
    asset::{Assets, Handle},
    math::primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere, Tetrahedron, Torus},
    platform::collections::HashMap,
    render::mesh::Mesh,
};
use rule_engine::piece::PieceModel;

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
                (PieceModel::Cone, meshes.add(Cone::default())),
                (PieceModel::Capsule, meshes.add(Capsule3d::default())),
                (PieceModel::Torus, meshes.add(Torus::new(0.25, 0.5))),
                (PieceModel::Tetrahedron, meshes.add(Tetrahedron::default())),
                (PieceModel::Cuboid, meshes.add(Cuboid::default())),
            ]),
        }
    }

    pub fn get(&self, model: PieceModel) -> &Handle<Mesh> {
        self.map.get(&model).expect("No mesh found")
    }
}
