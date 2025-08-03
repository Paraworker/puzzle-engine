use crate::states::{
    GameState, game_setup::GameSetupPlugin, loading::LoadingPlugin, menu::MenuPlugin,
    playing::PlayingPlugin, startup::StartupPlugin,
};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crazy_puzzle_rules::RulesError;
use ron::de::SpannedError;
use thiserror::Error;

mod assets;
mod config;
mod piece;
mod session;
mod states;
mod tile;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("no active player")]
    NoActivePlayer,
    #[error("rules error: {0}")]
    Rules(#[from] RulesError),
    #[error("config format error: {0}")]
    ConfigFormat(#[from] SpannedError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

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
        .add_plugins(EguiPlugin::default())
        .add_plugins(StartupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GameSetupPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(PlayingPlugin)
        .run();
}
