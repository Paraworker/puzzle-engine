use crate::{
    RulesError,
    piece::{PieceColor, PieceModel},
    pos::Pos,
    utils::{from_ron_str, to_ron_str},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialPiece {
    model: PieceModel,
    color: PieceColor,
    pos: (i64, i64),
}

impl InitialPiece {
    /// Creates a new initial piece.
    pub fn new(model: PieceModel, color: PieceColor, pos: Pos) -> Self {
        Self {
            model,
            color,
            pos: (pos.row(), pos.col()),
        }
    }

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
        self.pos.into()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitialLayout(Vec<InitialPiece>);

impl InitialLayout {
    /// Add a new initial piece to the layout.
    pub fn add(&mut self, piece: InitialPiece) {
        self.0.push(piece);
    }

    /// Returns the pieces.
    pub fn pieces(&self) -> &[InitialPiece] {
        &self.0
    }

    /// Parses from a ron string.
    pub fn from_ron_str(str: &str) -> Result<Self, RulesError> {
        from_ron_str(str)
    }

    /// Converts into a ron string.
    pub fn to_ron_str(&self) -> Result<String, RulesError> {
        to_ron_str(self)
    }
}
