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

/// Entities associated with a piece.
#[derive(Debug, Clone)]
pub struct PieceEntities {
    root: Entity,
    base_mesh: Entity,
    highlight: Entity,
}

impl PieceEntities {
    /// Creates a new `PieceEntities`.
    pub fn new(root: Entity, base_mesh: Entity, highlight: Entity) -> Self {
        Self {
            root,
            base_mesh,
            highlight,
        }
    }

    /// Returns the piece root entity.
    pub fn root(&self) -> Entity {
        self.root
    }

    /// Returns the base mesh entity.
    pub fn base_mesh(&self) -> Entity {
        self.base_mesh
    }

    /// Returns the highlight entity.
    pub fn highlight(&self) -> Entity {
        self.highlight
    }
}

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
    movable: HashSet<Pos>,
}

impl MovingPiece {
    /// Creates a new moving piece.
    pub fn new(model: PieceModel, color: PieceColor, initial: Pos) -> Self {
        Self {
            model,
            color,
            initial,
            current: initial,
            movable: HashSet::new(),
        }
    }

    /// Collects movable positions based on the given movement expression.
    pub fn collect_movable(
        &mut self,
        session: &GameSession,
        placed_piece_query: Query<&PlacedPiece>,
        tile_query: Query<&Tile>,
        movement: &BoolExpr,
    ) -> Result<(), GameError> {
        for tile in tile_query {
            // Skip source tile
            if self.initial == tile.pos() {
                continue;
            }

            let ctx = MovementContext {
                session,
                placed_piece_query,
                moving_model: self.model,
                moving_color: self.color,
                source_pos: self.initial,
                target_pos: tile.pos(),
            };

            if movement.evaluate(&ctx)? {
                self.movable.insert(tile.pos());
            }
        }

        Ok(())
    }

    pub fn movable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.movable.iter().cloned()
    }

    /// Attempts to set current pos to the given position.
    ///
    /// Returns `true` and updates the current position if the position is valid,
    /// meaning it is either in the set of placeable positions or
    /// the original position (i.e., the piece was not moved).
    ///
    /// Returns `false` if the position is not allowed.
    pub fn set_current_pos(&mut self, pos: Pos) -> bool {
        if !self.movable.contains(&pos) && self.initial != pos {
            return false;
        }

        self.current = pos;

        true
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
}

#[derive(Debug, Resource)]
pub struct PlacingPiece {
    model: PieceModel,
    color: PieceColor,
    to_place: Option<Pos>,
    placeable: HashSet<Pos>,
}

impl PlacingPiece {
    /// Creates a new placing piece.
    pub fn new(model: PieceModel, color: PieceColor) -> Self {
        Self {
            model,
            color,
            to_place: None,
            placeable: HashSet::new(),
        }
    }

    /// Collects placeable positions based on the given placement expression.
    pub fn collect_placeable(
        &mut self,
        session: &GameSession,
        placed_piece_query: Query<&PlacedPiece>,
        tile_query: Query<&Tile>,
        placement: &BoolExpr,
    ) -> Result<(), GameError> {
        for tile in tile_query {
            let ctx = PlacementContext {
                placed_piece_query,
                session,
                to_place_model: self.model,
                to_place_color: self.color,
                to_place_pos: tile.pos(),
            };

            if placement.evaluate(&ctx)? {
                self.placeable.insert(tile.pos());
            }
        }

        Ok(())
    }

    /// Returns the set of placeable positions.
    pub fn placeable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.placeable.iter().cloned()
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
}
