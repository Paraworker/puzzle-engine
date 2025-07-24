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
    /// Creates new piece info.
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
    start: Pos,
    current: Pos,
}

impl Dragged {
    /// Creates a new dragged piece.
    pub fn new(start: Pos) -> Self {
        Self {
            start,
            current: start,
        }
    }

    /// Checks if the piece has not been moved from its initial position.
    pub fn unmoved(&self) -> bool {
        self.start == self.current
    }

    /// Returns the initial position.
    pub fn start_pos(&self) -> Pos {
        self.start
    }

    /// Returns the current position.
    pub fn current_pos(&self) -> Pos {
        self.current
    }

    /// Updates the current dragged position.
    pub fn update_pos(&mut self, pos: Pos) {
        self.current = pos;
    }
}
