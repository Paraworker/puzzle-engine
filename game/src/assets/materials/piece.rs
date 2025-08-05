use bevy::{
    asset::{Assets, Handle},
    color::Color,
    pbr::StandardMaterial,
};
use rule_engine::piece::PieceColor;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PieceMaterials {
    map: HashMap<PieceColor, Handle<StandardMaterial>>,
}

impl PieceMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        PieceMaterials {
            map: HashMap::from([
                (PieceColor::White, materials.add(Color::srgb(0.9, 0.9, 0.9))),
                (PieceColor::Black, materials.add(Color::srgb(0.1, 0.1, 0.1))),
            ]),
        }
    }

    /// Returns the material for the given color.
    pub fn get(&self, color: PieceColor) -> &Handle<StandardMaterial> {
        self.map.get(&color).expect("Piece material not found")
    }

    /// Returns an iterator over the supported materials.
    pub fn materials(&self) -> impl Iterator<Item = (PieceColor, &Handle<StandardMaterial>)> {
        self.map.iter().map(|(color, material)| (*color, material))
    }
}
