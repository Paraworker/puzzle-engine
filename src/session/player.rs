use crate::rules::{
    count::Count,
    piece::{PieceColor, PieceModel, PieceRuleSet},
    player::{PlayerRuleSet, PlayerRules},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Player {
    piece_color: PieceColor,
    piece_stock: HashMap<PieceModel, Count>,
}

impl Player {
    pub fn from_rules(player_rules: &PlayerRules, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            piece_color: player_rules.piece_color(),
            piece_stock: piece_rule_set
                .pieces()
                .map(|(model, rules)| (model.clone(), rules.count()))
                .collect(),
        }
    }

    pub fn piece_color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn piece_stock(&mut self, model: PieceModel) -> &mut Count {
        self.piece_stock
            .get_mut(&model)
            .expect("No such piece model found")
    }
}

#[derive(Debug)]
pub struct Players {
    players: Vec<Player>,
    current: usize,
}

impl Players {
    pub fn from_rules(player_rule_set: &PlayerRuleSet, piece_rule_set: &PieceRuleSet) -> Self {
        Self {
            players: player_rule_set
                .players()
                .map(|rules| Player::from_rules(rules, piece_rule_set))
                .collect(),
            current: 0,
        }
    }

    /// Returns the current player.
    pub fn current(&mut self) -> &mut Player {
        &mut self.players[self.current]
    }

    /// Switches to the next player.
    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.players.len();
    }

    /// Returns the player with the specified color.
    pub fn get(&mut self, color: PieceColor) -> &mut Player {
        self.players
            .iter_mut()
            .find(|player| player.piece_color() == color)
            .expect("No such player found")
    }
}
