use indexmap::IndexMap;
use rule_engine::{
    count::Count,
    piece::{PieceColor, PieceModel, PieceRuleSet},
    player::{PlayerRuleSet, PlayerState},
};

use crate::GameError;

/// Represents a player in the game.
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

#[derive(Debug)]
pub struct Players {
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    map: IndexMap<PieceColor, Player>,
}

impl Players {
    pub fn new(player_rule_set: &PlayerRuleSet, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            map: player_rule_set
                .players()
                .map(|(color, _)| (color, Player::new(piece_rule_set)))
                .collect(),
        }
    }

    /// Returns the player with the specified color.
    pub fn get_by_color(&self, color: PieceColor) -> &Player {
        self.map.get(&color).expect("No such player found")
    }

    /// Returns the mutable player with the specified color.
    pub fn get_by_color_mut(&mut self, color: PieceColor) -> &mut Player {
        self.map.get_mut(&color).expect("No such player found")
    }

    /// Returns the player with the index.
    pub fn get_by_index(&self, index: usize) -> (PieceColor, &Player) {
        self.map
            .get_index(index)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Returns the mutable player with the index.
    pub fn get_by_index_mut(&mut self, index: usize) -> (PieceColor, &mut Player) {
        self.map
            .get_index_mut(index)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Returns the number of players.
    pub fn num(&self) -> usize {
        self.map.len()
    }

    /// Returns all players.
    pub fn iter(&self) -> impl Iterator<Item = (PieceColor, &Player)> {
        self.map.iter().map(|(color, player)| (*color, player))
    }

    /// Returns all mutable players.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PieceColor, &mut Player)> {
        self.map.iter_mut().map(|(color, player)| (*color, player))
    }

    /// Returns formatted string for the player states.
    pub fn player_states_message(&self) -> String {
        self.map
            .iter()
            .map(|(color, player)| format!("{}[{}]", color, player.state()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
