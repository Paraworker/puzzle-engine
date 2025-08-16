use crate::states::{
    error::ErrorPlugin, game_setup::GameSetupPlugin, loading::LoadingPlugin, menu::MenuPlugin,
    playing::PlayingPlugin, startup::StartupPlugin,
};
use bevy::prelude::*;

pub mod error;
pub mod game_setup;
pub mod loading;
pub mod menu;
pub mod playing;
pub mod startup;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Startup,
    Menu,
    GameSetup,
    Loading,
    Playing,
    Error,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(StartupPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(GameSetupPlugin)
            .add_plugins(LoadingPlugin)
            .add_plugins(PlayingPlugin)
            .add_plugins(ErrorPlugin);
    }
}
