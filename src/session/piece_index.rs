use bevy::prelude::*;
use std::collections::HashMap;

use crate::rules::position::Pos;

/// Entities associated with a piece.
#[derive(Debug, Clone)]
pub struct PieceEntities {
    base: Entity,
    highlighted: Entity,
}

impl PieceEntities {
    /// Creates a new `PieceEntities`.
    pub fn new(base: Entity, highlighted: Entity) -> Self {
        Self { base, highlighted }
    }

    /// Returns the base entity.
    pub fn base(&self) -> Entity {
        self.base
    }

    /// Returns the highlighted entity.
    pub fn highlighted(&self) -> Entity {
        self.highlighted
    }
}

/// A index of placed piece entities by their position.
#[derive(Debug)]
pub struct PlacedPieceIndex(HashMap<Pos, PieceEntities>);

impl PlacedPieceIndex {
    /// Creates a new `PieceIndex`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Adds a piece entities at the given position.
    pub fn add(&mut self, pos: Pos, entities: PieceEntities) {
        self.0.insert(pos, entities);
    }

    /// Removes the piece entities at the given position.
    pub fn remove(&mut self, pos: Pos) -> Option<PieceEntities> {
        self.0.remove(&pos)
    }

    /// Returns the piece entities at the given position.
    pub fn get(&self, pos: Pos) -> Option<&PieceEntities> {
        self.0.get(&pos)
    }
}
