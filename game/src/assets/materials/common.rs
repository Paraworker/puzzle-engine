use bevy::prelude::*;

#[derive(Debug)]
pub struct CommonMaterials {
    pub tile_black: Handle<StandardMaterial>,
    pub tile_white: Handle<StandardMaterial>,
    pub highlight_source_or_target: Handle<StandardMaterial>,
    pub highlight_placeable: Handle<StandardMaterial>,
}

impl CommonMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        CommonMaterials {
            tile_black: materials.add(Color::srgb(0.4, 0.4, 0.4)),
            tile_white: materials.add(Color::srgb(0.8, 0.8, 0.8)),
            highlight_source_or_target: materials.add(StandardMaterial {
                base_color: Color::srgba(0.4, 0.6, 1.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            highlight_placeable: materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.9, 0.2, 0.5),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
        }
    }
}
