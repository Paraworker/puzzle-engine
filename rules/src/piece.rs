use crate::{count::Count, expr::boolean::BoolExpr};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceModel {
    Cube,
    Sphere,
    Cylinder,
}

impl fmt::Display for PieceModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PieceModel::Cube => "Cube",
            PieceModel::Sphere => "Sphere",
            PieceModel::Cylinder => "Cylinder",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        };
        write!(f, "{text}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceRules {
    /// The maximum number of pieces allowed for this kind.
    count: Count,

    /// A boolean expression that defines whether a move is allowed.
    /// Evaluated in the context of [`ExprScenario::PieceMovement`].
    movement: BoolExpr,

    /// A boolean expression that defines whether placement is allowed.
    /// Evaluated in the context of [`ExprScenario::PiecePlacement`].
    placement: BoolExpr,
}

impl PieceRules {
    /// Returns the count of pieces allowed.
    pub fn count(&self) -> Count {
        self.count
    }

    /// Returns the movement boolean expression.
    pub fn movement(&self) -> &BoolExpr {
        &self.movement
    }

    /// Returns the placement boolean expression.
    pub fn placement(&self) -> &BoolExpr {
        &self.placement
    }
}

/// Uses [`IndexMap`] to ensure a stable iteration order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PieceRuleSet(IndexMap<PieceModel, PieceRules>);

impl PieceRuleSet {
    /// Returns the piece rules for the specified model.
    pub fn get(&self, model: PieceModel) -> &PieceRules {
        self.0.get(&model).expect("No such piece model found")
    }

    /// Returns all piece rules.
    pub fn pieces(&self) -> impl Iterator<Item = (&PieceModel, &PieceRules)> {
        self.0.iter()
    }
}
