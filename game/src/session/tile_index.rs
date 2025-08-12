use bevy::prelude::*;
use rule_engine::position::Pos;
use std::collections::HashMap;

/// Entities associated with a tile.
#[derive(Debug, Clone)]
pub struct TileEntities {
    root: Entity,
    source_or_target: Entity,
    placeable: Entity,
}

impl TileEntities {
    /// Creates a new `TileEntities`.
    pub fn new(root: Entity, source_or_target: Entity, placeable: Entity) -> Self {
        Self {
            root,
            source_or_target,
            placeable,
        }
    }

    /// Returns the root entity.
    pub fn root(&self) -> Entity {
        self.root
    }

    /// Returns the source or target entity.
    pub fn source_or_target(&self) -> Entity {
        self.source_or_target
    }

    /// Returns the placeable entity.
    pub fn placeable(&self) -> Entity {
        self.placeable
    }
}

/// An index of tile entities by their position.
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
