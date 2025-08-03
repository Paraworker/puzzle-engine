use crate::GameError;
use crazy_puzzle_rules::{
    count::Count,
    piece::{PieceColor, PieceModel, PieceRuleSet},
    player::PlayerRuleSet,
};
use indexmap::IndexMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Active,
    Won,
    Lost,
}

impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerState::Active => write!(f, "Active"),
            PlayerState::Won => write!(f, "Won"),
            PlayerState::Lost => write!(f, "Lost"),
        }
    }
}

/// Represents a player in the game.
///
/// This struct is managed by [`TurnController`].
#[derive(Debug)]
pub struct Player {
    /// The current state of the player.
    state: PlayerState,

    /// A mapping from each piece model to the remaining count for this player.
    ///
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    stock: IndexMap<PieceModel, Count>,
}

impl Player {
    pub fn new(piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            state: PlayerState::Active,
            stock: piece_rule_set
                .pieces()
                .map(|(model, rules)| (model.clone(), rules.count()))
                .collect(),
        }
    }

    /// Returns the current state of the player.
    pub fn state(&self) -> PlayerState {
        self.state
    }

    /// Sets the state of the player.
    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    /// Returns the piece stock for the specified model.
    pub fn stock(&self, model: PieceModel) -> Count {
        self.stock
            .get(&model)
            .copied()
            .expect("No such piece model found")
    }

    /// Decreases the piece stock for the specified piece model.
    pub fn decrease_stock(&mut self, model: PieceModel) -> Result<(), GameError> {
        self.stock
            .get_mut(&model)
            .expect("No such piece model found")
            .decrease()?;

        Ok(())
    }

    /// Returns an iterator over the piece stocks.
    pub fn stocks(&self) -> impl Iterator<Item = (PieceModel, Count)> {
        self.stock.iter().map(|(model, count)| (*model, *count))
    }
}

/// Controls the turn-based flow of the game, managing players and turn progression.
#[derive(Debug)]
pub struct TurnController {
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    players: IndexMap<PieceColor, Player>,

    /// The index of the current player.
    current_player: usize,

    /// The current turn number, starting from 1.
    turn_number: i64,

    /// The current round number, starting from 1.
    round_number: i64,
}

impl TurnController {
    pub fn new(player_rule_set: &PlayerRuleSet, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            players: player_rule_set
                .players()
                .map(|(color, _)| (color, Player::new(piece_rule_set)))
                .collect(),
            current_player: 0,
            turn_number: 1,
            round_number: 1,
        }
    }

    /// Returns the current player info.
    pub fn current_player(&self) -> (PieceColor, &Player) {
        self.players
            .get_index(self.current_player)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Returns the current mutable player info.
    pub fn current_player_mut(&mut self) -> (PieceColor, &mut Player) {
        self.players
            .get_index_mut(self.current_player)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Advances the turn to the next player.
    pub fn advance_turn(&mut self) -> Result<(), GameError> {
        let num_players = self.players.len();

        // Search for the next active player, checking each player at most once.
        for offset in 1..=num_players {
            let next_index = (self.current_player + offset) % num_players;

            // Only pick active player.
            if self.players[next_index].state() == PlayerState::Active {
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

    /// Returns the player with the specified color.
    pub fn player(&self, color: PieceColor) -> &Player {
        self.players.get(&color).expect("No such player found")
    }

    /// Returns the mutable player with the specified color.
    pub fn player_mut(&mut self, color: PieceColor) -> &mut Player {
        self.players.get_mut(&color).expect("No such player found")
    }

    /// Returns all players.
    pub fn players(&self) -> impl Iterator<Item = (PieceColor, &Player)> {
        self.players.iter().map(|(color, player)| (*color, player))
    }

    /// Returns all mutable players.
    pub fn players_mut(&mut self) -> impl Iterator<Item = (PieceColor, &mut Player)> {
        self.players
            .iter_mut()
            .map(|(color, player)| (*color, player))
    }

    /// Returns formatted string for the current turn message.
    pub fn turn_message(&self) -> String {
        format!(
            "{}'s Turn — Turn {}, Round {}",
            self.current_player().0,
            self.turn_number,
            self.round_number
        )
    }

    /// Returns formatted string for the player states.
    pub fn player_states_message(&self) -> String {
        self.players()
            .map(|(color, player)| format!("{}[{}]", color, player.state()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
