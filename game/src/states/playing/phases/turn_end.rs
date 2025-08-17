use crate::{
    GameError,
    expr_contexts::{game_over::GameOverContext, win_or_lose::WinOrLoseContext},
    states::{
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
    placed_piece_query: Query<&PlacedPiece>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    rules: Res<LoadedRules>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let session = session.as_mut();

    // Check lose and win conditions for each active player.
    for (piece_color, player) in session
        .players
        .iter_mut()
        .filter(|(_, player)| player.state() == PlayerState::Active)
    {
        let player_rules = rules.players.get_by_color(piece_color);

        let ctx = WinOrLoseContext {
            turn: &session.turn,
            last_action: &session.last_action,
            placed_piece_index: &session.placed_pieces,
            placed_piece_query,
        };

        // Check lose condition first
        if player_rules.lose_condition().evaluate(&ctx).unwrap() {
            player.set_state(PlayerState::Lost);

            // If the player has lost, we don't need to check win condition.
            continue;
        }

        // Check win condition
        if player_rules.win_condition().evaluate(&ctx).unwrap() {
            player.set_state(PlayerState::Won);
        }
    }

    let ctx = GameOverContext {
        session,
        placed_piece_query,
    };

    // Check game over condition
    if rules.game_over_condition.evaluate(&ctx).unwrap() {
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
            Err(_) => {
                panic!("Unexpected error occurred");
            }
        }
    }
}
