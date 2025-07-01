use bevy::prelude::*;
use in_game::InGamePlugin;
use main_menu::MainMenuPlugin;

mod checkerboard;
mod in_game;
mod main_menu;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
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
        .init_state::<AppState>()
        .add_plugins(MeshPickingPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(InGamePlugin)
        .run();
}
