use bevy::prelude::*;
use rulery::pos::Pos;

/// Entities associated with a tile.
#[derive(Debug, Clone)]
pub struct TileEntities {
    root: Entity,
    base_mesh: Entity,
    source_or_target: Entity,
    placeable: Entity,
}

impl TileEntities {
    /// Creates a new `TileEntities`.
    pub fn new(
        root: Entity,
        base_mesh: Entity,
        source_or_target: Entity,
        placeable: Entity,
    ) -> Self {
        Self {
            root,
            base_mesh,
            source_or_target,
            placeable,
        }
    }

    /// Returns the root entity.
    pub fn root(&self) -> Entity {
        self.root
    }

    /// Returns the base mesh entity.
    pub fn base_mesh(&self) -> Entity {
        self.base_mesh
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

/// Component for a base tile entity.
#[derive(Debug, Clone, Component)]
pub struct Tile {
    pos: Pos,
}

impl Tile {
    /// Creates a new tile.
    pub fn new(pos: Pos) -> Self {
        Self { pos }
    }

    /// Returns the position of the tile.
    pub const fn pos(&self) -> Pos {
        self.pos
    }
}
