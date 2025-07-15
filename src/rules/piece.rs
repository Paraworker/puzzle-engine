use crate::rules::direction::Directions;

#[derive(Debug)]
pub struct Piece {
    directions: Directions,
}

impl Piece {
    pub fn new(directions: Directions) -> Self {
        Self { directions }
    }

    pub fn directions(&self) -> &Directions {
        &self.directions
    }
}
