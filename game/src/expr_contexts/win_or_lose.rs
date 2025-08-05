use crate::{
    GameError,
    expr_contexts::{
        query_color_at_pos_equal, query_model_at_pos_equal, query_pos_occupied, query_round_number,
        query_turn_number,
    },
    piece::PlacedPiece,
    session::{piece_index::PlacedPieceIndex, turn::TurnController},
};
use bevy::prelude::*;
use rule_engine::{
    expr::Context,
    piece::{PieceColor, PieceModel},
    position::Pos,
};

#[derive(Debug)]
pub struct WinOrLoseContext<'t, 'i, 'world, 'state, 'data> {
    pub turn: &'t TurnController,
    pub placed_piece_index: &'i PlacedPieceIndex,
    pub placed_piece_query: Query<'world, 'state, &'data PlacedPiece>,
}

impl Context for WinOrLoseContext<'_, '_, '_, '_, '_> {
    type Error = GameError;

    fn turn_number(&self) -> Result<i64, Self::Error> {
        query_turn_number(&self.turn)
    }

    fn round_number(&self) -> Result<i64, Self::Error> {
        query_round_number(&self.turn)
    }

    fn pos_occupied(&self, pos: Pos) -> Result<bool, Self::Error> {
        query_pos_occupied(&self.placed_piece_index, pos)
    }

    fn model_at_pos_equal(&self, pos: Pos, model: PieceModel) -> Result<bool, Self::Error> {
        query_model_at_pos_equal(
            &self.placed_piece_index,
            self.placed_piece_query,
            pos,
            model,
        )
    }

    fn color_at_pos_equal(&self, pos: Pos, color: PieceColor) -> Result<bool, Self::Error> {
        query_color_at_pos_equal(
            &self.placed_piece_index,
            self.placed_piece_query,
            pos,
            color,
        )
    }
}
