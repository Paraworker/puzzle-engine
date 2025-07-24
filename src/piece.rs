use crate::{
    GameError,
    position::Pos,
    rules::expr::{ExprContext, boolean::BoolExpr},
    tile::Tile,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy)]
pub struct PieceKind {
    model: PieceModel,
    color: PieceColor,
}

impl PieceKind {
    /// Creates new piece kind.
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
    kind: PieceKind,
    pos: Pos,
}

impl Placed {
    /// Creates a new placed piece.
    pub fn new(kind: PieceKind, pos: Pos) -> Self {
        Self { kind, pos }
    }

    /// Returns the piece kind.
    pub fn kind(&self) -> PieceKind {
        self.kind
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
    kind: PieceKind,
    initial: Pos,
    current: Pos,
    placeable: HashSet<Pos>,
}

impl Dragged {
    /// Creates a new dragged piece.
    pub fn new<'a, I>(
        kind: PieceKind,
        initial: Pos,
        placement: &BoolExpr,
        tiles: I,
    ) -> Result<Self, GameError>
    where
        I: Iterator<Item = &'a Tile>,
    {
        Ok(Self {
            kind,
            initial,
            current: initial,
            placeable: Self::collect_placeable(kind, initial, placement, tiles)?,
        })
    }

    /// Returns the piece kind.
    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    /// Returns the initial position.
    pub fn initial_pos(&self) -> Pos {
        self.initial
    }

    /// Returns the current position.
    pub fn current_pos(&self) -> Pos {
        self.current
    }

    /// Checks if the piece has not been moved from its initial position.
    pub fn unmoved(&self) -> bool {
        self.initial == self.current
    }

    /// Attempts to place the dragged piece at the given position.
    ///
    /// Returns `true` and updates the current position if the position is valid,
    /// meaning it is either in the set of placeable positions or
    /// the original position (i.e., the piece was not moved).
    ///
    /// Returns `false` if the position is not allowed.
    pub fn try_place_at(&mut self, pos: Pos) -> bool {
        if !self.placeable.contains(&pos) && self.initial != pos {
            return false;
        }

        self.current = pos;

        true
    }

    pub fn placeable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.placeable.iter().cloned()
    }

    fn collect_placeable<'a, I>(
        kind: PieceKind,
        source: Pos,
        placement: &BoolExpr,
        tiles: I,
    ) -> Result<HashSet<Pos>, GameError>
    where
        I: Iterator<Item = &'a Tile>,
    {
        let mut placeable = HashSet::new();

        for tile in tiles {
            // Skip source tile
            if source == tile.pos() {
                continue;
            }

            let ctx = ExprContext {
                kind,
                source,
                target: tile.pos(),
            };

            if placement.evaluate(&ctx).unwrap() {
                placeable.insert(tile.pos());
            }
        }

        Ok(placeable)
    }
}
