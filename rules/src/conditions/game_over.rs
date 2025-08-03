use crate::{expr::boolean::BoolExpr, piece::PieceColor, player::PlayerState};
use serde::{Deserialize, Serialize};

pub type GameOverCondition = BoolExpr<GameOverBool, GameOverInt>;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameOverBool {
    /// Check if the player's state is equal to the given state.
    PlayerStateEqual(PieceColor, PlayerState),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameOverInt {
    /// The current turn number.
    TurnNumber,
    /// The current round number.
    RoundNumber,
}
