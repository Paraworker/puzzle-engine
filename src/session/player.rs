use crate::rules::{
    count::Count,
    piece::{PieceColor, PieceModel, PieceRuleSet},
    player::{PlayerRuleSet, PlayerRules},
};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Player {
    /// The color of the player's pieces.
    piece_color: PieceColor,

    /// A mapping from each piece model to the remaining count for this player.
    ///
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    piece_stock: IndexMap<PieceModel, Count>,
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

    pub fn piece_stocks(&self) -> impl Iterator<Item = (&PieceModel, &Count)> {
        self.piece_stock.iter()
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
    pub fn current(&self) -> &Player {
        &self.players[self.current]
    }

    /// Returns the current mutable player.
    pub fn current_mut(&mut self) -> &mut Player {
        &mut self.players[self.current]
    }

    /// Switches to the next player.
    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.players.len();
    }

    /// Returns the player with the specified color.
    pub fn get(&self, color: PieceColor) -> &Player {
        self.players
            .iter()
            .find(|player| player.piece_color() == color)
            .expect("No such player found")
    }

    /// Returns the mutable player with the specified color.
    pub fn get_mut(&mut self, color: PieceColor) -> &mut Player {
        self.players
            .iter_mut()
            .find(|player| player.piece_color() == color)
            .expect("No such player found")
    }
}
