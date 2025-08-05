use crate::{
    RulesError,
    expr::boolean::BoolExpr,
    piece::PieceColor,
    utils::{from_ron_str, to_ron_str},
};
use indexmap::{IndexMap, map::Entry};
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
        Self {
            win_condition: BoolExpr::False,
            lose_condition: BoolExpr::False,
        }
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
        Self(Default::default())
    }
}

impl PlayerRuleSet {
    /// Add a player.
    pub fn add(&mut self, color: PieceColor, rules: PlayerRules) -> Result<(), RulesError> {
        match self.0.entry(color) {
            Entry::Vacant(v) => {
                v.insert(rules);
                Ok(())
            }
            Entry::Occupied(_) => Err(RulesError::DuplicateColor),
        }
    }

    /// Returns the player rules at given index.
    ///
    /// Panic if out of index.
    pub fn get_by_index(&self, index: usize) -> (PieceColor, &PlayerRules) {
        self.0
            .get_index(index)
            .map(|(color, rules)| (*color, rules))
            .expect("Out of index!")
    }

    /// Returns the player rules with the specified color.
    pub fn get_by_color(&self, color: PieceColor) -> &PlayerRules {
        self.0.get(&color).expect("No such player found")
    }

    /// Remove the player rules at given index.
    ///
    /// Panic if out of index.
    pub fn remove_by_index(&mut self, index: usize) {
        self.0.shift_remove_index(index).expect("Out of index!");
    }

    /// Remove the player rules with the specified color.
    ///
    /// Panic if no such player found.
    pub fn remove_by_color(&mut self, color: PieceColor) {
        self.0.shift_remove(&color).expect("No such player found");
    }

    /// Returns an iterator over all player rules.
    pub fn iter(&self) -> impl Iterator<Item = (PieceColor, &PlayerRules)> {
        self.0.iter().map(|(color, rules)| (*color, rules))
    }

    /// Returns player number.
    pub fn player_num(&self) -> usize {
        self.0.len()
    }

    /// Parses from a ron string.
    pub fn from_ron_str(str: &str) -> Result<Self, RulesError> {
        from_ron_str(str)
    }

    /// Converts into a ron string.
    pub fn to_ron_str(&self) -> Result<String, RulesError> {
        to_ron_str(self)
    }
}
