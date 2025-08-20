use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BoardRuleSet {
    rows: i64,
    cols: i64,
}

impl BoardRuleSet {
    pub(crate) fn new() -> Self {
        Self { rows: 8, cols: 8 }
    }

    /// Sets the number of rows.
    pub(crate) const fn set_rows(&mut self, num: i64) {
        self.rows = num;
    }

    /// Sets the number of columns.
    pub(crate) const fn set_cols(&mut self, num: i64) {
        self.cols = num;
    }

    /// Returns the number of rows in the board.
    pub(crate) const fn rows(&self) -> i64 {
        self.rows
    }

    /// Returns the number of columns in the board.
    pub(crate) const fn cols(&self) -> i64 {
        self.cols
    }

    /// Returns the size of each tile.
    pub(crate) const fn tile_size() -> f32 {
        1.0
    }

    /// Returns the height of the tile.
    pub(crate) const fn tile_height() -> f32 {
        0.2
    }
}
