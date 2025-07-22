use bevy::{
    asset::{Assets, Handle},
    color::{
        Color,
        palettes::tailwind::{CYAN_300, ROSE_400, YELLOW_300},
    },
    pbr::StandardMaterial,
};

#[derive(Debug)]
pub struct CommonMaterials {
    pub tile_black: Handle<StandardMaterial>,
    pub tile_white: Handle<StandardMaterial>,
    pub tile_hover: Handle<StandardMaterial>,
    pub piece_hover: Handle<StandardMaterial>,
    pub piece_dragged: Handle<StandardMaterial>,
}

impl CommonMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        CommonMaterials {
            tile_black: materials.add(Color::srgb(0.2, 0.2, 0.2)),
            tile_white: materials.add(Color::srgb(0.8, 0.8, 0.8)),
            tile_hover: materials.add(Color::from(CYAN_300)),
            piece_hover: materials.add(Color::from(YELLOW_300)),
            piece_dragged: materials.add(Color::from(ROSE_400)),
        }
    }
}
