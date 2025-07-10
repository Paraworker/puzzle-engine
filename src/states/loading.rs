use crate::{
    config::{BoardName, Config},
    states::{ActiveBoard, GameState},
    tile::TileTopology,
};
use bevy::prelude::*;

pub fn load_board(
    commands: &mut Commands,
    asset_server: &AssetServer,
    next_state: &mut NextState<GameState>,
    name: BoardName,
) {
    let scene_path = Config::board_scene_path(&name);
    let topology_path = Config::board_topology_path(&name);

    commands.insert_resource(ActiveBoard(
        name,
        asset_server.load(scene_path),
        TileTopology::load(topology_path).expect("Unable to load tile topology"),
    ));

    next_state.set(GameState::Loading);
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), on_enter)
            .add_systems(Update, update.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), on_exit);
    }
}

fn on_enter() {
    // no-op
}

fn update(
    asset_server: Res<AssetServer>,
    scene: Res<ActiveBoard>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.is_loaded(scene.1.id()) {
        // Switch to the `Playing` state.
        next_state.set(GameState::Playing);
    }
}

fn on_exit() {
    // no-op
}
