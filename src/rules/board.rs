use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardGeometry {
    rows: usize,
    cols: usize,
    height: f32,
    tile_size: f32,
    half_width_col: f32,
    half_width_row: f32,
}

impl BoardGeometry {
    pub const fn with_rows_and_cols(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, 0.2, 1.0)
    }

    pub const fn new(rows: usize, cols: usize, height: f32, tile_size: f32) -> Self {
        Self {
            rows,
            cols,
            height,
            tile_size,
            half_width_col: half_width(cols, tile_size),
            half_width_row: half_width(rows, tile_size),
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

    /// Returns half width of the column in the board.
    pub const fn half_width_col(&self) -> f32 {
        self.half_width_col
    }

    /// Returns half width of the row in the board.
    pub const fn half_width_row(&self) -> f32 {
        self.half_width_row
    }
}

const fn half_width(unit_num: usize, unit_size: f32) -> f32 {
    (unit_num as f32 - 1.0) * unit_size / 2.0
}
