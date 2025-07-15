use bevy::state::state::States;

pub mod game_setup;
pub mod loading;
pub mod menu;
pub mod playing;
pub mod startup;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Startup,
    Menu,
    GameSetup,
    Loading,
    Playing,
}
