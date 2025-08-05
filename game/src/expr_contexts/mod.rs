use crate::{
    GameError,
    piece::PlacedPiece,
    session::{piece_index::PlacedPieceIndex, turn::TurnController},
};
use bevy::prelude::*;
use rule_engine::{
    piece::{PieceColor, PieceModel},
    position::Pos,
};

pub mod game_over;
pub mod movement;
pub mod placement;
pub mod win_or_lose;

fn query_turn_number(turn: &TurnController) -> Result<i64, GameError> {
    Ok(turn.turn_number())
}

fn query_round_number(turn: &TurnController) -> Result<i64, GameError> {
    Ok(turn.round_number())
}

fn query_pos_occupied(index: &PlacedPieceIndex, pos: Pos) -> Result<bool, GameError> {
    Ok(index.get(pos).is_some())
}

fn query_model_at_pos_equal(
    index: &PlacedPieceIndex,
    query: Query<&PlacedPiece>,
    pos: Pos,
    model: PieceModel,
) -> Result<bool, GameError> {
    let Some(entities) = index.get(pos) else {
        return Ok(false);
    };

    let placed = query.get(entities.base()).unwrap();

    Ok(placed.model() == model)
}

fn query_color_at_pos_equal(
    index: &PlacedPieceIndex,
    query: Query<&PlacedPiece>,
    pos: Pos,
    color: PieceColor,
) -> Result<bool, GameError> {
    let Some(entities) = index.get(pos) else {
        return Ok(false);
    };

    let placed = query.get(entities.base()).unwrap();

    Ok(placed.color() == color)
}
