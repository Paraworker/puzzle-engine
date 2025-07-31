use crate::{
    rules::{
        expr::{ExprContext, ExprScenario, boolean::BoolExpr},
        piece::{PieceColor, PieceModel},
        position::Pos,
    },
    tile::Tile,
};
use bevy::prelude::*;
use std::collections::HashSet;

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
pub struct PlacedPiece {
    kind: PieceKind,
    pos: Pos,
}

impl PlacedPiece {
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
pub struct MovingPiece {
    kind: PieceKind,
    initial: Pos,
    current: Pos,
    placeable: HashSet<Pos>,
}

impl MovingPiece {
    /// Creates a new moving piece.
    pub fn new<'a, I>(
        kind: PieceKind,
        initial: Pos,
        movement: &BoolExpr,
        tiles: I,
    ) -> anyhow::Result<Self>
    where
        I: Iterator<Item = &'a Tile>,
    {
        Ok(Self {
            kind,
            initial,
            current: initial,
            placeable: Self::collect_placeable(kind, initial, movement, tiles)?,
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
    pub fn moved(&self) -> bool {
        self.initial != self.current
    }

    /// Attempts to move this piece to the given position.
    ///
    /// Returns `true` and updates the current position if the position is valid,
    /// meaning it is either in the set of placeable positions or
    /// the original position (i.e., the piece was not moved).
    ///
    /// Returns `false` if the position is not allowed.
    pub fn try_move_to(&mut self, pos: Pos) -> bool {
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
        movement: &BoolExpr,
        tiles: I,
    ) -> anyhow::Result<HashSet<Pos>>
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
                scenario: ExprScenario::PieceMovement {
                    kind,
                    source,
                    target: tile.pos(),
                },
            };

            if movement.evaluate(&ctx)? {
                placeable.insert(tile.pos());
            }
        }

        Ok(placeable)
    }
}

#[derive(Debug, Component)]
pub struct HighlightedPiece;

#[derive(Debug)]
pub struct PlacingPiece {
    kind: PieceKind,
    to_place: Option<Pos>,
    placeable: HashSet<Pos>,
}

impl PlacingPiece {
    /// Creates a new placing piece.
    pub fn new<'a, I>(kind: PieceKind, placement: &BoolExpr, tiles: I) -> anyhow::Result<Self>
    where
        I: Iterator<Item = &'a Tile>,
    {
        Ok(Self {
            kind,
            to_place: None,
            placeable: Self::collect_placeable(kind, placement, tiles)?,
        })
    }

    /// Returns the piece kind.
    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    /// Attempts to place this piece to the given position.
    ///
    /// Returns `true` and updates the to place pos if the position is valid,
    /// meaning it is in the set of placeable positions.
    ///
    /// Returns `false` if the position is not allowed.
    pub fn try_place_at(&mut self, pos: Pos) -> bool {
        if !self.placeable.contains(&pos) {
            return false;
        }

        self.to_place = Some(pos);

        true
    }

    /// Clears the position where the piece is to be placed.
    pub fn clear_to_place(&mut self) {
        self.to_place = None;
    }

    /// Returns the position where the piece is to be placed.
    pub fn to_place_pos(&self) -> Option<Pos> {
        self.to_place
    }

    /// Returns the set of placeable positions.
    pub fn placeable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.placeable.iter().cloned()
    }

    fn collect_placeable<'a, I>(
        kind: PieceKind,
        placement: &BoolExpr,
        tiles: I,
    ) -> anyhow::Result<HashSet<Pos>>
    where
        I: Iterator<Item = &'a Tile>,
    {
        let mut placeable = HashSet::new();

        for tile in tiles {
            let ctx = ExprContext {
                scenario: ExprScenario::PiecePlacement {
                    kind,
                    to_place: tile.pos(),
                },
            };

            if placement.evaluate(&ctx)? {
                placeable.insert(tile.pos());
            }
        }

        Ok(placeable)
    }
}
