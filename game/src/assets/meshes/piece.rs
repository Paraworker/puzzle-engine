use bevy::{
    asset::{Assets, Handle},
    math::primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere, Tetrahedron, Torus},
    mesh::Mesh,
    prelude::*,
};
use rulery::{CheckedGameRules, piece::PieceModel};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PieceMeshes {
    map: HashMap<PieceModel, (Handle<Mesh>, Transform)>,
}

impl PieceMeshes {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            map: HashMap::from([
                (
                    PieceModel::Cube,
                    (
                        meshes.add(Cuboid::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.2, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.4),
                            ..default()
                        },
                    ),
                ),
                (
                    PieceModel::Sphere,
                    (
                        meshes.add(Sphere::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.25, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.5),
                            ..default()
                        },
                    ),
                ),
                (
                    PieceModel::Cylinder,
                    (
                        meshes.add(Cylinder::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.2, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.4),
                            ..default()
                        },
                    ),
                ),
                (
                    PieceModel::Cone,
                    (
                        meshes.add(Cone::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.25, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.5),
                            ..default()
                        },
                    ),
                ),
                (
                    PieceModel::Capsule,
                    (
                        meshes.add(Capsule3d::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.3, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.3),
                            ..default()
                        },
                    ),
                ),
                (
                    PieceModel::Torus,
                    (
                        meshes.add(Torus::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.25, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.25),
                            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                            ..default()
                        },
                    ),
                ),
                (
                    PieceModel::Tetrahedron,
                    (
                        meshes.add(Tetrahedron::default()),
                        Transform {
                            translation: Vec3::new(0.0, CheckedGameRules::tile_size() * 0.2, 0.0),
                            scale: Vec3::splat(CheckedGameRules::tile_size() * 0.4),
                            ..default()
                        },
                    ),
                ),
            ]),
        }
    }

    pub fn get(&self, model: PieceModel) -> &(Handle<Mesh>, Transform) {
        self.map.get(&model).expect("No mesh found")
    }
}
