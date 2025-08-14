use crate::{
    GameError,
    expr_contexts::{
        query_color_at_pos_equal, query_has_last_action, query_last_action_col,
        query_last_action_row, query_model_at_pos_equal, query_pos_occupied, query_round_number,
        query_turn_number,
    },
    states::playing::{piece::PlacedPiece, session::GameSession},
};
use bevy::prelude::*;
use rule_engine::{
    expr::Context,
    piece::{PieceColor, PieceModel},
    position::Pos,
};

#[derive(Debug)]
pub struct PlacementContext<'s, 'world, 'state, 'data> {
    pub session: &'s GameSession,
    pub placed_piece_query: Query<'world, 'state, &'data PlacedPiece>,
    pub to_place_model: PieceModel,
    pub to_place_color: PieceColor,
    pub to_place_pos: Pos,
}

impl Context for PlacementContext<'_, '_, '_, '_> {
    type Error = GameError;

    fn turn_number(&self) -> Result<i64, Self::Error> {
        query_turn_number(&self.session.turn)
    }

    fn round_number(&self) -> Result<i64, Self::Error> {
        query_round_number(&self.session.turn)
    }

    fn pos_occupied(&self, pos: Pos) -> Result<bool, Self::Error> {
        query_pos_occupied(&self.session.placed_pieces, pos)
    }

    fn model_at_pos_equal(&self, pos: Pos, model: PieceModel) -> Result<bool, Self::Error> {
        query_model_at_pos_equal(
            &self.session.placed_pieces,
            self.placed_piece_query,
            pos,
            model,
        )
    }

    fn color_at_pos_equal(&self, pos: Pos, color: PieceColor) -> Result<bool, Self::Error> {
        query_color_at_pos_equal(
            &self.session.placed_pieces,
            self.placed_piece_query,
            pos,
            color,
        )
    }

    fn has_last_action(&self) -> std::result::Result<bool, Self::Error> {
        query_has_last_action(&self.session.last_action)
    }

    fn last_action_row(&self) -> std::result::Result<i64, Self::Error> {
        query_last_action_row(&self.session.last_action)
    }

    fn last_action_col(&self) -> std::result::Result<i64, Self::Error> {
        query_last_action_col(&self.session.last_action)
    }

    fn to_place_model_equal(&self, model: PieceModel) -> Result<bool, Self::Error> {
        Ok(self.to_place_model == model)
    }

    fn to_place_color_equal(&self, color: PieceColor) -> Result<bool, Self::Error> {
        Ok(self.to_place_color == color)
    }

    fn to_place_row(&self) -> Result<i64, Self::Error> {
        Ok(self.to_place_pos.row())
    }

    fn to_place_col(&self) -> Result<i64, Self::Error> {
        Ok(self.to_place_pos.col())
    }
}
