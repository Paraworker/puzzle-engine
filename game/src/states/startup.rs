use crate::{assets::GameAssets, settings::Settings, states::AppState};
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::StandardMaterial,
    render::mesh::Mesh,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Load setting
    commands.insert_resource(Settings::load("game/settings.ron").unwrap());

    // Load assets
    commands.insert_resource(GameAssets::new(&mut materials, &mut meshes));

    // Switch to the `Menu` state once the startup is complete
    next_state.set(AppState::Menu);
}
