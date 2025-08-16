use crate::{
    GameError,
    expr_contexts::{movement::MovementContext, placement::PlacementContext},
    states::playing::{session::GameSession, tile::Tile},
};
use bevy::prelude::*;
use rule_engine::{
    expr::boolean::BoolExpr,
    piece::{PieceColor, PieceModel},
    pos::Pos,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Component)]
pub struct PlacedPiece {
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
}

impl PlacedPiece {
    /// Creates a new placed piece.
    pub fn new(model: PieceModel, color: PieceColor, pos: Pos) -> Self {
        Self { model, color, pos }
    }

    /// Returns the piece model.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the piece color.
    pub fn color(&self) -> PieceColor {
        self.color
    }

    /// Returns the position of the placed piece.
    pub fn pos(&self) -> Pos {
        self.pos
    }
}

#[derive(Debug, Clone, Component)]
pub struct MovingPiece {
    model: PieceModel,
    color: PieceColor,
    initial: Pos,
    current: Pos,
    placeable: HashSet<Pos>,
}

impl MovingPiece {
    /// Creates a new moving piece.
    pub fn new(
        model: PieceModel,
        color: PieceColor,
        initial: Pos,
        session: &mut GameSession,
        placed_piece_query: Query<&PlacedPiece>,
        movement: &BoolExpr,
        tile_query: Query<&Tile>,
    ) -> Result<Self, GameError> {
        Ok(Self {
            model,
            color,
            initial,
            current: initial,
            placeable: Self::collect_placeable(
                model,
                color,
                initial,
                session,
                placed_piece_query,
                movement,
                tile_query,
            )?,
        })
    }

    /// Returns the piece model.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the piece color.
    pub fn color(&self) -> PieceColor {
        self.color
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

    fn collect_placeable(
        model: PieceModel,
        color: PieceColor,
        source_pos: Pos,
        session: &GameSession,
        placed_piece_query: Query<&PlacedPiece>,
        movement: &BoolExpr,
        tile_query: Query<&Tile>,
    ) -> Result<HashSet<Pos>, GameError> {
        let mut placeable = HashSet::new();

        for tile in tile_query {
            // Skip source tile
            if source_pos == tile.pos() {
                continue;
            }

            let ctx = MovementContext {
                session,
                placed_piece_query,
                moving_model: model,
                moving_color: color,
                source_pos,
                target_pos: tile.pos(),
            };

            if movement.evaluate(&ctx)? {
                placeable.insert(tile.pos());
            }
        }

        Ok(placeable)
    }
}

#[derive(Debug)]
pub struct PlacingPiece {
    model: PieceModel,
    color: PieceColor,
    to_place: Option<Pos>,
    placeable: HashSet<Pos>,
}

impl PlacingPiece {
    /// Creates a new placing piece.
    pub fn new(
        model: PieceModel,
        color: PieceColor,
        session: &GameSession,
        placed_piece_query: Query<&PlacedPiece>,
        placement: &BoolExpr,
        tile_query: Query<&Tile>,
    ) -> Result<Self, GameError> {
        Ok(Self {
            model,
            color,
            to_place: None,
            placeable: Self::collect_placeable(
                model,
                color,
                session,
                placed_piece_query,
                placement,
                tile_query,
            )?,
        })
    }

    /// Returns the piece model.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the piece color.
    pub fn color(&self) -> PieceColor {
        self.color
    }

    /// Sets the position where the piece is to be placed.
    ///
    /// Returns `true` and updates the to place pos if the position is valid,
    /// meaning it is in the set of placeable positions.
    ///
    /// Returns `false` if the position is not allowed.
    pub fn set_to_place_pos(&mut self, pos: Pos) -> bool {
        if !self.placeable.contains(&pos) {
            return false;
        }

        self.to_place = Some(pos);

        true
    }

    /// Clears the to place position and returns it.
    pub fn clear_to_place_pos(&mut self) -> Option<Pos> {
        self.to_place.take()
    }

    /// Returns the position where the piece is to be placed.
    pub fn to_place_pos(&self) -> Option<Pos> {
        self.to_place
    }

    /// Returns the set of placeable positions.
    pub fn placeable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.placeable.iter().cloned()
    }

    fn collect_placeable(
        model: PieceModel,
        color: PieceColor,
        session: &GameSession,
        placed_piece_query: Query<&PlacedPiece>,
        placement: &BoolExpr,
        tile_query: Query<&Tile>,
    ) -> Result<HashSet<Pos>, GameError> {
        let mut placeable = HashSet::new();

        for tile in tile_query {
            let ctx = PlacementContext {
                placed_piece_query,
                session,
                to_place_model: model,
                to_place_color: color,
                to_place_pos: tile.pos(),
            };

            if placement.evaluate(&ctx)? {
                placeable.insert(tile.pos());
            }
        }

        Ok(placeable)
    }
}
