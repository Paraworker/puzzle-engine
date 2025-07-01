use crate::states::{ActiveScene, GameState};
use bevy::prelude::*;

pub fn request_load_scene(
    commands: &mut Commands,
    asset_server: &AssetServer,
    next_state: &mut NextState<GameState>,
    path: &str,
) {
    commands.insert_resource(ActiveScene(asset_server.load(path)));
    next_state.set(GameState::Loading);
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup)
            .add_systems(Update, update.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup);
    }
}

fn setup() {
    // no-op
}

fn update(
    asset_server: Res<AssetServer>,
    scene: Res<ActiveScene>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.is_loaded(scene.0.id()) {
        // Switch to the `Playing` state.
        next_state.set(GameState::Playing);
    }
}

fn cleanup() {
    // no-op
}
