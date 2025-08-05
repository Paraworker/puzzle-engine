use crate::{GameError, session::player::Players};
use rule_engine::player::PlayerState;

/// Controls the turn-based flow of the game.
#[derive(Debug)]
pub struct TurnController {
    /// The index of the current player.
    current_player: usize,

    /// The current turn number, starting from 1.
    turn_number: i64,

    /// The current round number, starting from 1.
    round_number: i64,
}

impl TurnController {
    pub fn new() -> Self {
        Self {
            current_player: 0,
            turn_number: 1,
            round_number: 1,
        }
    }

    /// Returns the current player index.
    pub fn current_player(&self) -> usize {
        self.current_player
    }

    /// Advances the turn to the next player.
    pub fn advance_turn(&mut self, players: &Players) -> Result<(), GameError> {
        // Search for the next active player, checking each player at most once.
        for offset in 1..=players.num() {
            let next_index = (self.current_player + offset) % players.num();

            // Only pick active player.
            if players.get_by_index(next_index).1.state() == PlayerState::Active {
                // A new round begins if the turn has wrapped around to a player at or before
                // the previous player's index.
                if next_index <= self.current_player {
                    self.round_number += 1;
                }

                // Update the current player and increment the turn number.
                self.current_player = next_index;
                self.turn_number += 1;

                return Ok(());
            }
        }

        // If the loop completes, no active players were found.
        Err(GameError::NoActivePlayer)
    }

    /// Returns current turn number.
    pub fn turn_number(&self) -> i64 {
        self.turn_number
    }

    /// Returns current round number.
    pub fn round_number(&self) -> i64 {
        self.round_number
    }

    /// Returns formatted string for the current turn message.
    pub fn turn_message(&self, players: &Players) -> String {
        format!(
            "{}'s Turn — Turn {}, Round {}",
            players.get_by_index(self.current_player).0,
            self.turn_number,
            self.round_number
        )
    }
}
