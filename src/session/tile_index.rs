use crate::rules::position::Pos;
use bevy::prelude::*;
use std::collections::HashMap;

/// Entities associated with a tile.
#[derive(Debug, Clone)]
pub struct TileEntities {
    base: Entity,
    drag_initial: Entity,
    placeable: Entity,
}

impl TileEntities {
    /// Creates a new `TileEntities`.
    pub fn new(base: Entity, drag_initial: Entity, placeable: Entity) -> Self {
        Self {
            base,
            drag_initial,
            placeable,
        }
    }

    /// Returns the base entity.
    pub fn base(&self) -> Entity {
        self.base
    }

    /// Returns the drag initial entity.
    pub fn drag_initial(&self) -> Entity {
        self.drag_initial
    }

    /// Returns the placeable entity.
    pub fn placeable(&self) -> Entity {
        self.placeable
    }
}

/// A index of tile entities by their position.
#[derive(Debug)]
pub struct TileIndex(HashMap<Pos, TileEntities>);

impl TileIndex {
    /// Creates a new `TileIndex`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Adds a new tile entity at the given position.
    pub fn add(&mut self, pos: Pos, entities: TileEntities) {
        self.0.insert(pos, entities);
    }

    /// Returns the tile entities at the given position.
    pub fn get(&self, pos: Pos) -> Option<&TileEntities> {
        self.0.get(&pos)
    }
}
