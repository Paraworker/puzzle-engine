use crate::{
    GameError,
    states::playing::{
        piece::PlacedPiece,
        session::{PlacedPieceIndex, turn::TurnController},
    },
};
use bevy::ecs::system::Query;
use rule_engine::{
    piece::{PieceColor, PieceModel},
    pos::Pos,
    rect::Rect,
};

pub mod game_over;
pub mod movement;
pub mod placement;
pub mod win_or_lose;

fn query_pos_occupied(index: &PlacedPieceIndex, pos: Pos) -> Result<bool, GameError> {
    Ok(index.get(&pos).is_some())
}

fn query_has_last_action(last_action: &Option<Pos>) -> Result<bool, GameError> {
    Ok(last_action.is_some())
}

fn query_turn_number(turn: &TurnController) -> Result<i64, GameError> {
    Ok(turn.turn_number())
}

fn query_round_number(turn: &TurnController) -> Result<i64, GameError> {
    Ok(turn.round_number())
}

fn query_last_action_row(last_action: &Option<Pos>) -> Result<i64, GameError> {
    match last_action {
        Some(pos) => Ok(pos.row()),
        None => Err(GameError::NoLastAction),
    }
}

fn query_last_action_col(last_action: &Option<Pos>) -> Result<i64, GameError> {
    match last_action {
        Some(pos) => Ok(pos.col()),
        None => Err(GameError::NoLastAction),
    }
}

fn query_count_in_rect(rect: Rect, index: &PlacedPieceIndex) -> Result<i64, GameError> {
    Ok(index
        .keys()
        .copied()
        .filter(|pos| rect.contains(*pos))
        .count() as i64)
}

fn query_count_piece_in_rect(
    piece: (PieceModel, PieceColor),
    rect: Rect,
    index: &PlacedPieceIndex,
    query: Query<&PlacedPiece>,
) -> Result<i64, GameError> {
    let (want_model, want_color) = piece;

    Ok(index
        .iter()
        .filter(|&(pos, _)| rect.contains(*pos))
        .filter(|&(_, entities)| {
            let placed = query.get(entities.root()).unwrap();
            placed.model() == want_model && placed.color() == want_color
        })
        .count() as i64)
}

fn query_model_at_pos(
    index: &PlacedPieceIndex,
    query: Query<&PlacedPiece>,
    pos: Pos,
) -> Result<PieceModel, GameError> {
    let Some(entities) = index.get(&pos) else {
        return Err(GameError::NoPieceAtPos(pos));
    };

    Ok(query.get(entities.root()).unwrap().model())
}

fn query_color_at_pos(
    index: &PlacedPieceIndex,
    query: Query<&PlacedPiece>,
    pos: Pos,
) -> Result<PieceColor, GameError> {
    let Some(entities) = index.get(&pos) else {
        return Err(GameError::NoPieceAtPos(pos));
    };

    Ok(query.get(entities.root()).unwrap().color())
}
