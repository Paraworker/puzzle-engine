use crate::{
    expr::boolean::BoolExpr,
    piece::{PieceColor, PieceModel},
};
use serde::{Deserialize, Serialize};

pub type MovementCondition = BoolExpr<MovementBool, MovementInt>;

#[derive(Debug, Serialize, Deserialize)]
pub enum MovementBool {
    /// Checks if the moving piece's model is equal to the given model.
    MovingModelEqual(PieceModel),
    /// Checks if the moving piece's color is equal to the given color.
    MovingColorEqual(PieceColor),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MovementInt {
    /// The current turn number.
    TurnNumber,
    /// The current round number.
    RoundNumber,
    /// The source tile column.
    SourceCol,
    /// The source tile row.
    SourceRow,
    /// The destination tile column.
    TargetCol,
    /// The destination tile row.
    TargetRow,
}
