use crate::utils::load_ron;
use bevy::ecs::component::Component;
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::HashMap, path::Path};

/// Id of a tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Component)]
pub struct TileId(u64);

impl FromStr for TileId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TileId(s.parse()?))
    }
}

impl From<u64> for TileId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<TileId> for u64 {
    fn from(id: TileId) -> Self {
        id.0
    }
}

/// A tile topology.
#[derive(Debug, Deserialize)]
pub struct TileTopology {
    /// The tiles in the topology.
    tiles: HashMap<TileId, Tile>,

    /// (start tile, max_distance) → reachable set
    #[serde(skip)]
    distance_cache: HashMap<(TileId, usize), HashSet<TileId>>,
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

    /// Returns all tiles that are reachable from `start` within `max_distance` steps (inclusive).
    pub fn tiles_within_distance(
        &mut self,
        start: TileId,
        max_distance: usize,
    ) -> &HashSet<TileId> {
        // Query the distance cache first
        self.distance_cache
            .entry((start, max_distance))
            .or_insert_with(|| Self::compute_reachable(&self.tiles, start, max_distance))
    }

    fn compute_reachable(
        tiles: &HashMap<TileId, Tile>,
        start: TileId,
        max_distance: usize,
    ) -> HashSet<TileId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back((start, 0));

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_distance {
                continue;
            }

            if let Some(tile) = tiles.get(&current) {
                for neighbor in tile.neighbors() {
                    if visited.insert(neighbor) {
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }

        visited
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
