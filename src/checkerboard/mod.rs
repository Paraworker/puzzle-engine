use crate::checkerboard::tile::{Tile, TileId};
use std::collections::HashMap;

pub mod tile;

/// A checkerboard.
pub struct Checkerboard {
    tiles: HashMap<TileId, Tile>,
}

impl Checkerboard {
    /// Creates a new checkerboard.
    pub fn new() -> Self {
        Checkerboard {
            tiles: HashMap::new(),
        }
    }

    /// Adds a tile to the checkerboard.
    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.insert(tile.id(), tile);
    }

    /// Returns a reference to the tile with the given id.
    pub fn get_tile(&self, id: TileId) -> Option<&Tile> {
        self.tiles.get(&id)
    }

    /// Returns a mutable reference to the tile with the given id.
    pub fn get_tile_mut(&mut self, id: TileId) -> Option<&mut Tile> {
        self.tiles.get_mut(&id)
    }
}
