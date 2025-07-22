use crate::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitialLayout(Vec<Piece>);

impl InitialLayout {
    /// Returns the layout.
    pub fn layout(&self) -> &[Piece] {
        &self.0
    }
}
