use crate::{
    GameError,
    expr_contexts::{
        query_color_at_pos_equal, query_has_last_action, query_last_action_col,
        query_last_action_row, query_model_at_pos_equal, query_pos_occupied, query_round_number,
        query_turn_number,
    },
    piece::PlacedPiece,
    session::GameSession,
};
use bevy::prelude::*;
use rule_engine::{
    expr::Context,
    piece::{PieceColor, PieceModel},
    player::PlayerState,
    position::Pos,
};

#[derive(Debug)]
pub struct GameOverContext<'s, 'world, 'state, 'data> {
    pub session: &'s GameSession,
    pub query: Query<'world, 'state, &'data PlacedPiece>,
}

impl Context for GameOverContext<'_, '_, '_, '_> {
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
        query_model_at_pos_equal(&self.session.placed_pieces, self.query, pos, model)
    }

    fn color_at_pos_equal(&self, pos: Pos, color: PieceColor) -> Result<bool, Self::Error> {
        query_color_at_pos_equal(&self.session.placed_pieces, self.query, pos, color)
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

    fn player_state_equal(
        &self,
        color: PieceColor,
        state: PlayerState,
    ) -> Result<bool, Self::Error> {
        Ok(self.session.players.get_by_color(color).state() == state)
    }
}
