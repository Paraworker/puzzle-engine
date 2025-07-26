use crate::rules::{
    piece::{PieceColor, PieceModel},
    position::Pos,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialPiece {
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
}

impl InitialPiece {
    /// Returns the model of the piece.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the color of the piece.
    pub fn color(&self) -> PieceColor {
        self.color
    }

    /// Returns the position of the piece.
    pub fn pos(&self) -> Pos {
        self.pos
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitialLayout(Vec<InitialPiece>);

impl InitialLayout {
    /// Returns the layout.
    pub fn layout(&self) -> &[InitialPiece] {
        &self.0
    }
}
