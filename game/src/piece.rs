use crate::{
    GameError,
    expr_contexts::{movement::MovementContext, placement::PlacementContext},
    session::GameSession,
    tile::Tile,
};
use bevy::prelude::*;
use rule_engine::{
    conditions::{movement::MovementCondition, placement::PlacementCondition},
    piece::{PieceColor, PieceModel},
    position::Pos,
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

    /// Sets the position of the placed piece.
    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
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
    pub fn new<'a, I>(
        model: PieceModel,
        color: PieceColor,
        initial: Pos,
        session: &mut GameSession,
        movement: &MovementCondition,
        tiles: I,
    ) -> Result<Self, GameError>
    where
        I: Iterator<Item = &'a Tile>,
    {
        Ok(Self {
            model,
            color,
            initial,
            current: initial,
            placeable: Self::collect_placeable(model, color, initial, session, movement, tiles)?,
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

    fn collect_placeable<'a, I>(
        model: PieceModel,
        color: PieceColor,
        source: Pos,
        session: &GameSession,
        movement: &MovementCondition,
        tiles: I,
    ) -> Result<HashSet<Pos>, GameError>
    where
        I: Iterator<Item = &'a Tile>,
    {
        let mut placeable = HashSet::new();

        let turn_number = session.turn_controller.turn_number();
        let round_number = session.turn_controller.round_number();

        for tile in tiles {
            // Skip source tile
            if source == tile.pos() {
                continue;
            }

            let ctx = MovementContext {
                model,
                color,
                turn_number,
                round_number,
                source,
                target: tile.pos(),
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
    model: PieceModel,
    color: PieceColor,
    to_place: Option<Pos>,
    placeable: HashSet<Pos>,
}

impl PlacingPiece {
    /// Creates a new placing piece.
    pub fn new<'a, I>(
        model: PieceModel,
        color: PieceColor,
        session: &GameSession,
        placement: &PlacementCondition,
        tiles: I,
    ) -> Result<Self, GameError>
    where
        I: Iterator<Item = &'a Tile>,
    {
        Ok(Self {
            model,
            color,
            to_place: None,
            placeable: Self::collect_placeable(model, color, session, placement, tiles)?,
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
        model: PieceModel,
        color: PieceColor,
        session: &GameSession,
        placement: &PlacementCondition,
        tiles: I,
    ) -> Result<HashSet<Pos>, GameError>
    where
        I: Iterator<Item = &'a Tile>,
    {
        let mut placeable = HashSet::new();

        let turn_number = session.turn_controller.turn_number();
        let round_number = session.turn_controller.round_number();

        for tile in tiles {
            let ctx = PlacementContext {
                model,
                color,
                turn_number,
                round_number,
                to_place: tile.pos(),
            };

            if placement.evaluate(&ctx)? {
                placeable.insert(tile.pos());
            }
        }

        Ok(placeable)
    }
}
