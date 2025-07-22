use serde::{Deserialize, Serialize};

/// Tile position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Pos(usize, usize);

impl Pos {
    /// Creates a new `Pos`.
    pub const fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    /// Returns the row index of the tile position.
    pub const fn row(&self) -> usize {
        self.0
    }

    /// Returns the column index of the tile position.
    pub const fn col(&self) -> usize {
        self.1
    }
}
