use crate::{
    GameError,
    expr_contexts::{game_over::GameOverContext, win_or_lose::WinOrLoseContext},
    states::{
        AppState,
        error::CurrentError,
        game_setup::LoadedRules,
        playing::{phases::GamePhase, piece::PlacedPiece, session::GameSession},
    },
};
use bevy::prelude::*;
use rule_engine::player::PlayerState;

pub struct TurnEndPlugin;

impl Plugin for TurnEndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, evaluate_turn.run_if(in_state(GamePhase::TurnEnd)));
    }
}

/// A system that evaluates win/loss conditions, and prepares for the next turn or ends the game.
fn evaluate_turn(
    mut commands: Commands,
    placed_piece_query: Query<&PlacedPiece>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut next_state: ResMut<NextState<AppState>>,
    rules: Res<LoadedRules>,
) {
    if let NextState::Pending(_) = *next_state {
        return;
    }

    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let session = session.as_mut();

    // Evaluates state for each active player.
    for (piece_color, player) in session
        .players
        .iter_mut()
        .filter(|(_, player)| player.state() == PlayerState::Active)
    {
        let ctx = WinOrLoseContext {
            turn: &session.turn,
            last_action: &session.last_action,
            placed_piece_index: &session.placed_pieces,
            placed_piece_query,
        };

        player.set_state(
            rules
                .get_player(piece_color)
                .unwrap()
                .evaluate_state(&ctx)
                .unwrap(),
        );
    }

    let ctx = GameOverContext {
        session,
        placed_piece_query,
    };

    // Check game over condition
    if rules.evaluate_game_over_condition(&ctx).unwrap() {
        // Game over.
        next_phase.set(GamePhase::GameOver);
    } else {
        // Advance the turn
        match session.turn.advance_turn(&session.players) {
            Ok(()) => {
                // Start next turn.
                next_phase.set(GamePhase::Selecting);
            }
            Err(GameError::NoActivePlayer) => {
                // Game over.
                next_phase.set(GamePhase::GameOver);
            }
            Err(err) => {
                commands.insert_resource(CurrentError(err));
                next_state.set(AppState::Error);
            }
        }
    }
}
