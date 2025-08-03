use crate::{
    piece::{PieceColor, PieceModel},
    position::Pos,
};

pub mod arith;
pub mod boolean;

#[derive(Debug)]
pub enum ExprScenario {
    /// A piece is being moved from one tile to another.
    PieceMovement {
        /// The model of the piece.
        model: PieceModel,
        /// The color of the piece.
        color: PieceColor,
        /// The source position of the piece.
        source: Pos,
        /// The target position of the piece.
        target: Pos,
    },
    /// A new piece is being placed onto the board.
    PiecePlacement {
        /// The model of the piece.
        model: PieceModel,
        /// The color of the piece.
        color: PieceColor,
        /// The position where the piece is to be placed.
        to_place: Pos,
    },
    /// Check whether a player should win.
    PlayerWinCondition {
        /// Piece color of the player.
        piece_color: PieceColor,
    },
    /// Check whether a player should lose.
    PlayerLoseCondition {
        /// Piece color of the player.
        piece_color: PieceColor,
    },
    /// Check whether the game should over.
    GameOverCondition,
}

#[derive(Debug)]
pub struct ExprContext {
    pub turn_number: i64,
    pub round_number: i64,
    pub scenario: ExprScenario,
}
