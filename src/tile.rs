use serde::Deserialize;
use std::{collections::HashMap, path::Path};

use crate::utils::load_ron;

/// Id of a tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct TileId(u64);

impl TileId {
    /// Creates a new tile id from a raw value.
    pub fn from_raw(id: u64) -> Self {
        TileId(id)
    }

    /// Converts the tile id into a raw value.
    pub fn into_raw(self) -> u64 {
        self.0
    }
}

/// A tile topology.
#[derive(Debug, Deserialize)]
pub struct TileTopology {
    tiles: HashMap<TileId, Tile>,
}

impl TileTopology {
    /// Load a new tile topology.
    pub fn load<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        load_ron(path)
    }

    /// Returns the tile with the given id.
    pub fn tile(&self, id: TileId) -> Option<&Tile> {
        self.tiles.get(&id)
    }
}

/// A tile.
#[derive(Debug, Deserialize)]
pub struct Tile {
    edges: Vec<Edge>,
}

impl Tile {
    /// Returns the edge at the given index.
    pub fn edge(&self, index: usize) -> Option<&Edge> {
        self.edges.get(index)
    }

    /// Returns the number of edges of the tile.
    pub fn edges_num(&self) -> usize {
        self.edges.len()
    }

    /// Returns all neighbors of the tile.
    pub fn neighbors(&self) -> impl Iterator<Item = TileId> {
        self.edges.iter().filter_map(|edge| edge.neighbor())
    }

    /// Checks if the tile is a neighbor of the given tile.
    pub fn is_neighbor(&self, other: TileId) -> bool {
        self.neighbors().any(|id| id == other)
    }
}

/// A tile edge.
#[derive(Debug, Deserialize)]
pub struct Edge {
    neighbor: Option<TileId>,
}

impl Edge {
    /// Returns the id of the neighbor tile, if any.
    pub fn neighbor(&self) -> Option<TileId> {
        self.neighbor
    }
}
