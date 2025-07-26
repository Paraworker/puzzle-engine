use crate::states::{
    GameState, game_setup::GameSetupPlugin, loading::LoadingPlugin, menu::MenuPlugin,
    playing::PlayingPlugin, startup::StartupPlugin,
};
use bevy::prelude::*;

mod assets;
mod config;
mod piece;
mod rules;
mod session;
mod states;
mod tile;
mod utils;

fn new_window_plugin() -> WindowPlugin {
    const WINDOW_TITLE: &str = "Crazy Puzzle";

    WindowPlugin {
        primary_window: Some(Window {
            title: WINDOW_TITLE.to_string(),
            ..default()
        }),
        ..default()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(new_window_plugin()))
        .init_state::<GameState>()
        .add_plugins(MeshPickingPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GameSetupPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(PlayingPlugin)
        .run();
}
