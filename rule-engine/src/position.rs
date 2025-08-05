use std::fmt;

use serde::{Deserialize, Serialize};

/// Tile position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Pos(i64, i64);

impl Pos {
    /// Creates a new `Pos`.
    pub const fn new(row: i64, col: i64) -> Self {
        Self(row, col)
    }

    /// Returns the row index of the tile position.
    pub const fn row(&self) -> i64 {
        self.0
    }

    /// Returns the column index of the tile position.
    pub const fn col(&self) -> i64 {
        self.1
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
