use crate::{expr::boolean::BoolExpr, piece::PieceColor};
use serde::{Deserialize, Serialize};

pub type WinOrLoseCondition = BoolExpr<WinOrLoseBool, WinOrLoseInt>;

#[derive(Debug, Serialize, Deserialize)]
pub enum WinOrLoseBool {
    /// Checks if the player's color is equal to the given color.
    PlayerColorEqual(PieceColor),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WinOrLoseInt {
    /// The current turn number.
    TurnNumber,
    /// The current round number.
    RoundNumber,
}
