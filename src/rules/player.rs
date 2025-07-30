use crate::rules::piece::PieceColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerRules(PieceColor);

impl PlayerRules {
    /// Returns the piece color of the player.
    pub fn piece_color(&self) -> PieceColor {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerRuleSet(Vec<PlayerRules>);

impl PlayerRuleSet {
    /// Returns all players.
    pub fn players(&self) -> impl Iterator<Item = &PlayerRules> {
        self.0.iter()
    }
}
