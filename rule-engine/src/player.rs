use crate::{
    RulesError,
    expr::{Context, boolean::BoolExpr},
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
    /// A boolean expression that defines whether the player loses.
    ///
    /// In evaluation order, this is usually checked **before** `win_condition`.
    /// If the `lose_condition` is satisfied, the player is considered to have lost
    /// regardless of whether the `win_condition` also holds.
    lose_condition: BoolExpr,

    /// A boolean expression that defines whether the player wins.
    ///
    /// Typically, evaluated only if `lose_condition` is not satisfied.
    /// This ensures that an invalid or failing state cannot be counted as a win.
    win_condition: BoolExpr,
}

impl PlayerRules {
    /// Creates a new player rules.
    pub fn new(win_condition: BoolExpr, lose_condition: BoolExpr) -> Self {
        Self {
            win_condition,
            lose_condition,
        }
    }

    /// Evaluates player state.
    pub fn evaluate_state<C>(&self, ctx: &C) -> Result<PlayerState, C::Error>
    where
        C: Context,
    {
        // Evaluate the lose condition first.
        if self.lose_condition.evaluate(ctx)? {
            return Ok(PlayerState::Lost);
        }

        // If the lose condition is not met, check the win condition.
        if self.win_condition.evaluate(ctx)? {
            return Ok(PlayerState::Won);
        }

        // If neither condition is met, the player is still active.
        Ok(PlayerState::Active)
    }
}

/// Uses [`IndexMap`] to ensure a stable iteration order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct PlayerRuleSet(IndexMap<PieceColor, PlayerRules>);

impl PlayerRuleSet {
    pub(crate) fn new() -> Self {
        Self(IndexMap::new())
    }

    /// Add a player.
    pub(crate) fn add(&mut self, color: PieceColor, rules: PlayerRules) -> Result<(), RulesError> {
        match self.0.entry(color) {
            Entry::Vacant(v) => {
                v.insert(rules);
                Ok(())
            }
            Entry::Occupied(_) => Err(RulesError::DuplicateColor),
        }
    }

    /// Returns the player rules with the specified color.
    pub(crate) fn get_by_color(&self, color: PieceColor) -> Result<&PlayerRules, RulesError> {
        self.0.get(&color).ok_or(RulesError::NoSuchColor(color))
    }

    /// Returns an iterator over all player rules.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (PieceColor, &PlayerRules)> {
        self.0.iter().map(|(color, rules)| (*color, rules))
    }

    /// Returns if the set is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Parses from a ron string.
    pub(crate) fn from_ron_str(str: &str) -> Result<Self, RulesError> {
        from_ron_str(str)
    }

    /// Converts into a ron string.
    pub(crate) fn to_ron_str(&self) -> Result<String, RulesError> {
        to_ron_str(self)
    }
}
