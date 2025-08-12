use bevy::{
    asset::{Assets, Handle},
    color::{
        Color,
        palettes::tailwind::{GREEN_300, RED_300, YELLOW_300},
    },
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
                (PieceColor::White, materials.add(Color::srgb(0.8, 0.8, 0.8))),
                (PieceColor::Black, materials.add(Color::srgb(0.3, 0.3, 0.3))),
                (PieceColor::Red, materials.add(Color::from(RED_300))),
                (PieceColor::Yellow, materials.add(Color::from(YELLOW_300))),
                (PieceColor::Green, materials.add(Color::from(GREEN_300))),
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
