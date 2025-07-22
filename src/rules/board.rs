use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardRuleSet {
    rows: usize,
    cols: usize,
}

impl BoardRuleSet {
    /// Returns the number of rows in the board.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the board.
    pub const fn cols(&self) -> usize {
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

    /// Returns half width of the column in the board.
    pub const fn half_width_col(&self) -> f32 {
        (self.cols as f32 - 1.0) * Self::tile_size() / 2.0
    }

    /// Returns half width of the row in the board.
    pub const fn half_width_row(&self) -> f32 {
        (self.rows as f32 - 1.0) * Self::tile_size() / 2.0
    }
}
