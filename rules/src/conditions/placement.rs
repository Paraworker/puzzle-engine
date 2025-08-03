use crate::{
    expr::boolean::BoolExpr,
    piece::{PieceColor, PieceModel},
};
use serde::{Deserialize, Serialize};

pub type PlacementCondition = BoolExpr<PlacementBool, PlacementInt>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlacementBool {
    /// Checks if the model of the piece being placed is equal to the given model.
    ToPlaceModelEqual(PieceModel),
    /// Checks if the color of the piece being placed is equal to the given color.
    ToPlaceColorEqual(PieceColor),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlacementInt {
    /// The current turn number.
    TurnNumber,
    /// The current round number.
    RoundNumber,
    /// The column where the piece is being placed.
    ToPlaceCol,
    /// The row where the piece is being placed.
    ToPlaceRow,
}
