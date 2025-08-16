use crate::pos::Pos;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    row_min: i64,
    col_min: i64,
    row_max: i64,
    col_max: i64,
}

impl Rect {
    pub fn new(p1: Pos, p2: Pos) -> Self {
        Self {
            row_min: min(p1.row(), p2.row()),
            col_min: min(p1.col(), p2.col()),
            row_max: max(p1.row(), p2.row()),
            col_max: max(p1.col(), p2.col()),
        }
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.row() >= self.row_min
            && pos.row() <= self.row_max
            && pos.col() >= self.col_min
            && pos.col() <= self.col_max
    }

    pub fn iter(&self) -> impl Iterator<Item = Pos> {
        (self.row_min..=self.row_max)
            .flat_map(move |row| (self.col_min..=self.col_max).map(move |col| Pos::new(row, col)))
    }
}
