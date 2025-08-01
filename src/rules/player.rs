use crate::rules::piece::PieceColor;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerRules {}

/// Uses [`IndexMap`] to ensure a stable iteration order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerRuleSet(IndexMap<PieceColor, PlayerRules>);

impl PlayerRuleSet {
    /// Returns all players.
    pub fn players(&self) -> impl Iterator<Item = (&PieceColor, &PlayerRules)> {
        self.0.iter()
    }
}
