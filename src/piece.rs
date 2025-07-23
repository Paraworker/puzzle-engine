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

#[derive(Debug, Clone, Component)]
pub struct PieceInfo {
    model: PieceModel,
    color: PieceColor,
}

impl PieceInfo {
    /// Creates a new piece info.
    pub fn new(model: PieceModel, color: PieceColor) -> Self {
        Self { model, color }
    }

    /// Returns the model of the piece.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the color of the piece.
    pub fn color(&self) -> PieceColor {
        self.color
    }
}

#[derive(Debug, Clone, Component)]
pub struct Placed {
    pos: Pos,
}

impl Placed {
    /// Creates a new placed piece.
    pub fn new(pos: Pos) -> Self {
        Self { pos }
    }

    /// Returns the position of the placed piece.
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Sets the position of the placed piece.
    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}

#[derive(Debug, Clone, Component)]
pub struct Dragged {
    initial: Pos,
    dragged: Pos,
}

impl Dragged {
    /// Creates a new dragged piece.
    pub fn new(initial: Pos) -> Self {
        Self {
            initial,
            dragged: initial,
        }
    }

    /// Checks if the piece has not been moved from its initial position.
    pub fn unmoved(&self) -> bool {
        self.initial == self.dragged
    }

    /// Returns the initial position.
    pub fn initial_pos(&self) -> Pos {
        self.initial
    }

    /// Returns the dragged position.
    pub fn dragged_pos(&self) -> Pos {
        self.dragged
    }

    /// Updates the dragged position.
    pub fn update_pos(&mut self, pos: Pos) {
        self.dragged = pos;
    }
}
