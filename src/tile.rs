use crate::position::Pos;
use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Tile {
    pos: Pos,
    color: Handle<StandardMaterial>,
}

impl Tile {
    /// Creates a new tile.
    pub fn new(pos: Pos, color: Handle<StandardMaterial>) -> Self {
        Self { pos, color }
    }

    /// Returns the position of the tile.
    pub const fn pos(&self) -> Pos {
        self.pos
    }

    /// Returns the color of the tile.
    pub fn color(&self) -> Handle<StandardMaterial> {
        self.color.clone()
    }
}
