use crate::piece::PieceModel;
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
}

impl PieceRules {
    /// Creates a new piece rules.
    pub fn new(count: Count) -> Self {
        Self { count }
    }

    /// Returns the count of pieces.
    pub fn count(&self) -> &Count {
        &self.count
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
