use bevy::prelude::*;
use rule_engine::position::Pos;

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

/// Component for a source or target entity.
#[derive(Debug, Component)]
pub struct SourceOrTargetTile;

/// Component for a Placeable entity.
#[derive(Debug, Component)]
pub struct PlaceableTile;
