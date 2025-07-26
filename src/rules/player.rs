use crate::rules::piece::PieceColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRules {
    name: String,
    color: PieceColor,
}

impl PlayerRules {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> PieceColor {
        self.color
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerRuleSet(Vec<PlayerRules>);

impl PlayerRuleSet {
    /// Returns all players.
    pub fn players(&self) -> &[PlayerRules] {
        &self.0
    }
}
