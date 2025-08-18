use crate::{
    RulesError,
    count::Count,
    expr::{Context, boolean::BoolExpr},
    utils::{from_ron_str, to_ron_str},
};
use indexmap::{IndexMap, map::Entry};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceModel {
    Cube,
    Sphere,
    Cylinder,
    Capsule,
    Cone,
    Torus,
    Tetrahedron,
}

impl fmt::Display for PieceModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PieceModel::Cube => "Cube",
            PieceModel::Sphere => "Sphere",
            PieceModel::Cylinder => "Cylinder",
            PieceModel::Capsule => "Capsule",
            PieceModel::Cone => "Cone",
            PieceModel::Torus => "Torus",
            PieceModel::Tetrahedron => "Tetrahedron",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
    Red,
    Yellow,
    Green,
    Cyan,
    Purple,
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
            PieceColor::Red => "Red",
            PieceColor::Yellow => "Yellow",
            PieceColor::Green => "Green",
            PieceColor::Cyan => "Cyan",
            PieceColor::Purple => "Purple",
        };
        write!(f, "{text}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceRules {
    /// The maximum number of pieces allowed for this kind.
    count: Count,

    /// A boolean expression that defines whether a move is allowed.
    movement: BoolExpr,

    /// A boolean expression that defines whether placement is allowed.
    placement: BoolExpr,
}

impl PieceRules {
    pub fn new(count: Count, movement: BoolExpr, placement: BoolExpr) -> Self {
        Self {
            count,
            movement,
            placement,
        }
    }

    /// Returns the count of pieces allowed.
    pub fn count(&self) -> Count {
        self.count
    }

    /// Evaluates the movement condition.
    pub fn can_move<C>(&self, ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        self.movement.evaluate(ctx)
    }

    /// Evaluates the placement condition.
    pub fn can_place<C>(&self, ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        self.placement.evaluate(ctx)
    }
}

/// Uses [`IndexMap`] to ensure a stable iteration order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct PieceRuleSet(IndexMap<PieceModel, PieceRules>);

impl PieceRuleSet {
    pub(crate) fn new() -> Self {
        Self(IndexMap::new())
    }

    /// Adds a new piece model with its rules.
    pub(crate) fn add(&mut self, model: PieceModel, rules: PieceRules) -> Result<(), RulesError> {
        match self.0.entry(model) {
            Entry::Vacant(v) => {
                v.insert(rules);
                Ok(())
            }
            Entry::Occupied(_) => Err(RulesError::DuplicateModel),
        }
    }

    /// Returns the piece rules for the specified model.
    pub(crate) fn get_by_model(&self, model: PieceModel) -> Result<&PieceRules, RulesError> {
        self.0.get(&model).ok_or(RulesError::NoSuchModel(model))
    }

    /// Returns all rules.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (PieceModel, &PieceRules)> {
        self.0.iter().map(|(model, rules)| (*model, rules))
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
