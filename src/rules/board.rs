use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardGeometry {
    rows: usize,
    cols: usize,
    height: f32,
    tile_size: f32,
}

impl BoardGeometry {
    pub const fn with_rows_and_cols(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            height: 0.2,
            tile_size: 1.0,
        }
    }

    pub const fn new(rows: usize, cols: usize, height: f32, tile_size: f32) -> Self {
        Self {
            rows,
            cols,
            height,
            tile_size,
        }
    }

    /// Returns the number of rows in the board.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the board.
    pub const fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the height of the board.
    pub const fn height(&self) -> f32 {
        self.height
    }

    /// Returns the size of a tile in the board.
    pub const fn tile_size(&self) -> f32 {
        self.tile_size
    }
}
