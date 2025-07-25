use bevy::{
    asset::{Assets, Handle},
    color::Color,
    pbr::StandardMaterial,
};

#[derive(Debug)]
pub struct CommonMaterials {
    pub tile_black: Handle<StandardMaterial>,
    pub tile_white: Handle<StandardMaterial>,
    pub highlight_source: Handle<StandardMaterial>,
    pub highlight_placeable: Handle<StandardMaterial>,
}

impl CommonMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        CommonMaterials {
            tile_black: materials.add(Color::srgb(0.2, 0.2, 0.2)),
            tile_white: materials.add(Color::srgb(0.8, 0.8, 0.8)),
            highlight_source: materials.add(Color::srgba(0.4, 0.6, 1.0, 0.6)),
            highlight_placeable: materials.add(Color::srgba(1.0, 0.9, 0.2, 0.6)),
        }
    }
}
