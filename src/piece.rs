use crate::position::Pos;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceModel {
    Cube,
    Sphere,
    Cylinder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Piece {
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
}

impl Piece {
    /// Creates a new piece.
    pub fn new(model: PieceModel, color: PieceColor, pos: Pos) -> Self {
        Self { model, color, pos }
    }

    /// Returns the model of the piece.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the color of the piece.
    pub fn color(&self) -> PieceColor {
        self.color
    }

    /// Returns the position of the piece on the board.
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Sets the position of the piece on the board.
    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}
