use crate::states::{GameState, TileHoverMaterial, TileNormalMaterial, TilePressedMaterial};
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::{
        Color,
        palettes::tailwind::{CYAN_300, YELLOW_300},
    },
    ecs::system::{Commands, ResMut},
    pbr::StandardMaterial,
    state::state::NextState,
};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, on_startup);
    }
}

fn on_startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // global materials
    commands.insert_resource(TileNormalMaterial(materials.add(Color::WHITE)));
    commands.insert_resource(TileHoverMaterial(materials.add(Color::from(CYAN_300))));
    commands.insert_resource(TilePressedMaterial(materials.add(Color::from(YELLOW_300))));

    // Switch to the `Menu` state once the startup is complete
    next_state.set(GameState::Menu);
}
