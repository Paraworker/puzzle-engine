use std::fmt;

/// Tile position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl From<(i64, i64)> for Pos {
    fn from(tuple: (i64, i64)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl From<Pos> for (i64, i64) {
    fn from(pos: Pos) -> Self {
        (pos.row(), pos.col())
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
