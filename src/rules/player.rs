use crate::rules::{expr::boolean::BoolExpr, piece::PieceColor};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRules {
    /// A boolean expression that defines whether a player can win.
    win_condition: BoolExpr,
    /// A boolean expression that defines whether a player can lose.
    lose_condition: BoolExpr,
}

impl PlayerRules {
    /// Returns the win condition expression.
    pub fn win_condition(&self) -> &BoolExpr {
        &self.win_condition
    }

    /// Returns the lose condition expression.
    pub fn lose_condition(&self) -> &BoolExpr {
        &self.lose_condition
    }
}

/// Uses [`IndexMap`] to ensure a stable iteration order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerRuleSet(IndexMap<PieceColor, PlayerRules>);

impl PlayerRuleSet {
    /// Returns the player rules with the specified color.
    pub fn get(&self, color: PieceColor) -> &PlayerRules {
        self.0.get(&color).expect("No such player found")
    }

    /// Returns all players.
    pub fn players(&self) -> impl Iterator<Item = (PieceColor, &PlayerRules)> {
        self.0.iter().map(|(color, rules)| (*color, rules))
    }
}
