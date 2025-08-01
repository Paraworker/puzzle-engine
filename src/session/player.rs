use crate::rules::{
    RulesError,
    count::Count,
    piece::{PieceColor, PieceModel, PieceRuleSet},
    player::{PlayerRuleSet, PlayerRules},
};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Player {
    /// A mapping from each piece model to the remaining count for this player.
    ///
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    piece_stock: IndexMap<PieceModel, Count>,
}

impl Player {
    pub fn from_rules(player_rules: &PlayerRules, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            piece_stock: piece_rule_set
                .pieces()
                .map(|(model, rules)| (model.clone(), rules.count()))
                .collect(),
        }
    }

    pub fn stock_of(&self, model: PieceModel) -> Count {
        self.piece_stock
            .get(&model)
            .copied()
            .expect("No such piece model found")
    }

    pub fn decrease_stock_of(&mut self, model: PieceModel) -> Result<(), RulesError> {
        self.piece_stock
            .get_mut(&model)
            .expect("No such piece model found")
            .decrease()
    }

    pub fn piece_stocks(&self) -> impl Iterator<Item = (PieceModel, Count)> {
        self.piece_stock
            .iter()
            .map(|(model, count)| (*model, *count))
    }
}

#[derive(Debug)]
pub struct Players {
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    players: IndexMap<PieceColor, Player>,

    /// The index of the current player.
    current: usize,
}

impl Players {
    pub fn from_rules(player_rule_set: &PlayerRuleSet, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            players: player_rule_set
                .players()
                .map(|(color, rules)| (*color, Player::from_rules(rules, piece_rule_set)))
                .collect(),
            current: 0,
        }
    }

    /// Returns the current player info.
    pub fn current(&self) -> (PieceColor, &Player) {
        self.players
            .get_index(self.current)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Returns the current mutable player info.
    pub fn current_mut(&mut self) -> (PieceColor, &mut Player) {
        self.players
            .get_index_mut(self.current)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Switches to the next player.
    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.players.len();
    }

    /// Returns the player with the specified color.
    pub fn get(&self, color: PieceColor) -> &Player {
        self.players.get(&color).expect("No such player found")
    }

    /// Returns the mutable player with the specified color.
    pub fn get_mut(&mut self, color: PieceColor) -> &mut Player {
        self.players.get_mut(&color).expect("No such player found")
    }
}
