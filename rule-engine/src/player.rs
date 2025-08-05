use crate::{expr::boolean::BoolExpr, piece::PieceColor};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRules {
    /// A boolean expression that defines whether a player can win.
    win_condition: BoolExpr,
    /// A boolean expression that defines whether a player can lose.
    lose_condition: BoolExpr,
}

impl Default for PlayerRules {
    fn default() -> Self {
        Self { win_condition: BoolExpr::False, lose_condition: BoolExpr::False }
    }
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

impl Default for PlayerRuleSet {
    fn default() -> Self {
        // At least one player, `White` as default.
        Self(IndexMap::from([(PieceColor::White, PlayerRules::default())]))
    }
}

impl PlayerRuleSet {
    /// Returns the player rules with the specified color.
    pub fn get(&self, color: PieceColor) -> &PlayerRules {
        self.0.get(&color).expect("No such player found")
    }

    /// Returns all players.
    pub fn iter(&self) -> impl Iterator<Item = (PieceColor, &PlayerRules)> {
        self.0.iter().map(|(color, rules)| (*color, rules))
    }

    /// Returns player number.
    pub fn player_num(&self) -> usize {
        self.0.len()
    }
}
