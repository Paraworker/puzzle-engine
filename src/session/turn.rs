use crate::rules::{
    RulesError,
    count::Count,
    piece::{PieceColor, PieceModel, PieceRuleSet},
    player::{PlayerRuleSet, PlayerRules},
};
use indexmap::IndexMap;

/// Represents a player in the game.
///
/// This struct is managed by [`TurnController`].
#[derive(Debug)]
pub struct Player {
    /// A mapping from each piece model to the remaining count for this player.
    ///
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    stock: IndexMap<PieceModel, Count>,
}

impl Player {
    pub fn from_rules(player_rules: &PlayerRules, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            stock: piece_rule_set
                .pieces()
                .map(|(model, rules)| (model.clone(), rules.count()))
                .collect(),
        }
    }

    /// Returns the piece stock for the specified model.
    pub fn stock(&self, model: PieceModel) -> Count {
        self.stock
            .get(&model)
            .copied()
            .expect("No such piece model found")
    }

    /// Decreases the piece stock for the specified piece model.
    pub fn decrease_stock(&mut self, model: PieceModel) -> Result<(), RulesError> {
        self.stock
            .get_mut(&model)
            .expect("No such piece model found")
            .decrease()
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
    pub fn from_rules(player_rule_set: &PlayerRuleSet, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            players: player_rule_set
                .players()
                .map(|(color, rules)| (*color, Player::from_rules(rules, piece_rule_set)))
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
    pub fn advance_turn(&mut self) {
        // Increment the current player index
        self.current_player = (self.current_player + 1) % self.players.len();

        // Increment the turn number
        self.turn_number += 1;

        // If we have cycled through all players, increment the round number
        if self.current_player == 0 {
            self.round_number += 1;
        }
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

    /// Returns formatted string for the current turn message.
    pub fn formatted_turn_message(&self) -> String {
        format!(
            "{}'s Turn — Turn {}, Round {}",
            self.current_player().0,
            self.turn_number,
            self.round_number
        )
    }
}
