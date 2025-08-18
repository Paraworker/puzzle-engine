use crate::states::AppStatePlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_tweening::TweeningPlugin;
use ron::de::SpannedError;
use rule_engine::{RulesError, pos::Pos};
use thiserror::Error;

mod assets;
mod expr_contexts;
mod settings;
mod states;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("no active player")]
    NoActivePlayer,
    #[error("no last action")]
    NoLastAction,
    #[error("piece already exists at position: {0}")]
    DuplicatePiece(Pos),
    #[error("no piece at position: {0}")]
    NoPieceAtPos(Pos),
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
        .add_plugins(MeshPickingPlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(AppStatePlugin)
        .run();
}
