use crate::states::{
    AppState,
    playing::phases::{
        game_over::GameOverPlugin, moving::MovingPlugin, placing::PlacingPlugin,
        selecting::SelectingPlugin, turn_end::TurnEndPlugin,
    },
};
use bevy::prelude::*;

pub mod game_over;
pub mod moving;
pub mod placing;
pub mod selecting;
pub mod turn_end;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::Playing)]
pub enum GamePhase {
    /// The player is selecting a piece to move on the board.
    #[default]
    Selecting,

    /// The player is moving a piece on the board.
    Moving,

    /// The player is placing a new piece on the board.
    Placing,

    /// The turn has ended, evaluating win/loss conditions.
    TurnEnd,

    /// The game has ended, player is reviewing in a read-only state.
    GameOver,
}

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GamePhase>()
            .add_plugins(SelectingPlugin)
            .add_plugins(MovingPlugin)
            .add_plugins(PlacingPlugin)
            .add_plugins(TurnEndPlugin)
            .add_plugins(GameOverPlugin);
    }
}
