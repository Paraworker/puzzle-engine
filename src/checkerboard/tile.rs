/// Id of a tile on the checkerboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(u64);

/// A tile on the checkerboard.
pub struct Tile {
    id: TileId,
    edges: Vec<Edge>,
}

impl Tile {
    /// Creates a new tile with the given id and edges.
    pub fn new(id: TileId, edges: Vec<Edge>) -> Self {
        Tile { id, edges }
    }

    /// Returns the id of the tile.
    pub fn id(&self) -> TileId {
        self.id
    }

    /// Returns the edge at the given index.
    pub fn edge(&self, index: usize) -> Option<&Edge> {
        self.edges.get(index)
    }

    /// Returns a mutable reference to the edge at the given index.
    pub fn edge_mut(&mut self, index: usize) -> Option<&mut Edge> {
        self.edges.get_mut(index)
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
pub struct Edge {
    neighbor: Option<TileId>,
}

impl Edge {
    /// Creates a new edge with the given neighbor.
    pub fn new(neighbor: Option<TileId>) -> Self {
        Edge { neighbor }
    }

    /// Returns the id of the neighbor tile, if any.
    pub fn neighbor(&self) -> Option<TileId> {
        self.neighbor
    }
}
