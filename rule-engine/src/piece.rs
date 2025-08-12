use crate::{RulesError, count::Count, expr::boolean::BoolExpr};
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
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
            PieceColor::Red => "Red",
            PieceColor::Yellow => "Yellow",
            PieceColor::Green => "Green",
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

impl Default for PieceRules {
    fn default() -> Self {
        Self {
            count: Count::Infinite,
            movement: BoolExpr::False,
            placement: BoolExpr::False,
        }
    }
}

impl PieceRules {
    /// Returns the count of pieces allowed.
    pub fn count(&self) -> Count {
        self.count
    }

    /// Returns the movement condition.
    pub fn movement(&self) -> &BoolExpr {
        &self.movement
    }

    /// Returns the placement condition.
    pub fn placement(&self) -> &BoolExpr {
        &self.placement
    }
}

/// Uses [`IndexMap`] to ensure a stable iteration order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PieceRuleSet(IndexMap<PieceModel, PieceRules>);

impl Default for PieceRuleSet {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl PieceRuleSet {
    /// Adds a new piece model with its rules.
    pub fn add(&mut self, model: PieceModel, rules: PieceRules) -> Result<(), RulesError> {
        match self.0.entry(model) {
            Entry::Vacant(v) => {
                v.insert(rules);
                Ok(())
            }
            Entry::Occupied(_) => Err(RulesError::DuplicateModel),
        }
    }

    /// Returns the piece rules at given index.
    ///
    /// Panic if out of index.
    pub fn get_by_index(&self, index: usize) -> (PieceModel, &PieceRules) {
        self.0
            .get_index(index)
            .map(|(model, rules)| (*model, rules))
            .expect("Out of index!")
    }

    /// Returns the piece rules for the specified model.
    pub fn get_by_model(&self, model: PieceModel) -> &PieceRules {
        self.0.get(&model).expect("No such piece model found")
    }

    /// Remove the piece rules at given index.
    ///
    /// Panic if out of index.
    pub fn remove_by_index(&mut self, index: usize) {
        self.0.shift_remove_index(index).expect("Out of index!");
    }

    /// Remove the piece rules with the specified model.
    ///
    /// Panic if no such piece model found.
    pub fn remove_by_model(&mut self, model: PieceModel) {
        self.0
            .shift_remove(&model)
            .expect("No such piece model found");
    }

    /// Returns all rules.
    pub fn iter(&self) -> impl Iterator<Item = (PieceModel, &PieceRules)> {
        self.0.iter().map(|(model, rules)| (*model, rules))
    }

    /// Returns number of added piece models.
    pub fn model_num(&self) -> usize {
        self.0.len()
    }
}
