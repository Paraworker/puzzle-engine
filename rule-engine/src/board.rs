use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardRuleSet {
    rows: i64,
    cols: i64,
}

impl Default for BoardRuleSet {
    fn default() -> Self {
        Self { rows: 8, cols: 8 }
    }
}

impl BoardRuleSet {
    /// Returns the number of rows in the board.
    pub const fn rows(&self) -> i64 {
        self.rows
    }

    /// Returns the number of columns in the board.
    pub const fn cols(&self) -> i64 {
        self.cols
    }

    /// Returns the size of each tile.
    pub const fn tile_size() -> f32 {
        1.0
    }

    /// Returns the height of the tile.
    pub const fn tile_height() -> f32 {
        0.2
    }
}
