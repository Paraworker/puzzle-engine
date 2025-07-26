use crate::{piece::PieceKind, rules::position::Pos};

pub mod arith;
pub mod boolean;

#[derive(Debug)]
pub enum ExprScenario {
    /// A piece is being moved from one tile to another.
    PieceMovement {
        /// The piece kind being moved.
        kind: PieceKind,
        /// The source position of the piece.
        source: Pos,
        /// The target position of the piece.
        target: Pos,
    },
    /// A new piece is being placed onto the board.
    PiecePlacement {
        /// The piece kind being placed.
        kind: PieceKind,
        /// The position where the piece is to be placed.
        to_place: Pos,
    },
    /// A win condition is being checked.
    WinCondition,
}

#[derive(Debug)]
pub struct ExprContext {
    pub scenario: ExprScenario,
}
