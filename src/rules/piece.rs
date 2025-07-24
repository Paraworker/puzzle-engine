use crate::{piece::PieceModel, rules::expr::boolean::BoolExpr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Count {
    Infinite,
    Finite(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceRules {
    count: Count,
    placement: BoolExpr,
}

impl PieceRules {
    /// Returns the count of pieces.
    pub fn count(&self) -> &Count {
        &self.count
    }

    /// Returns the boolean expression used to determine valid placement tiles
    pub fn placement(&self) -> &BoolExpr {
        &self.placement
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PieceRuleSet(HashMap<PieceModel, PieceRules>);

impl PieceRuleSet {
    /// Returns the piece rules for the specified model.
    pub fn get(&self, model: PieceModel) -> Option<&PieceRules> {
        self.0.get(&model)
    }
}
