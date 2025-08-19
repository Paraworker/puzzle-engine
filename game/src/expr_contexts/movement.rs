use crate::{
    GameError,
    expr_contexts::{
        query_color_at_pos, query_count_in_rect, query_count_piece_in_rect, query_has_last_action,
        query_last_action_col, query_last_action_row, query_model_at_pos, query_pos_occupied,
        query_round_number, query_turn_number,
    },
    states::playing::session::GameSession,
};
use rule_engine::{
    expr::Context,
    piece::{PieceColor, PieceModel},
    pos::Pos,
    rect::Rect,
};

#[derive(Debug)]
pub struct MovementContext<'s> {
    pub session: &'s GameSession,
    pub moving_model: PieceModel,
    pub moving_color: PieceColor,
    pub source_pos: Pos,
    pub target_pos: Pos,
}

impl Context for MovementContext<'_> {
    type Error = GameError;

    fn pos_occupied(&self, pos: Pos) -> Result<bool, Self::Error> {
        query_pos_occupied(&self.session.placed_pieces, pos)
    }

    fn has_last_action(&self) -> Result<bool, Self::Error> {
        query_has_last_action(&self.session.last_action)
    }

    fn turn_number(&self) -> Result<i64, Self::Error> {
        query_turn_number(&self.session.turn)
    }

    fn round_number(&self) -> Result<i64, Self::Error> {
        query_round_number(&self.session.turn)
    }

    fn last_action_row(&self) -> Result<i64, Self::Error> {
        query_last_action_row(&self.session.last_action)
    }

    fn last_action_col(&self) -> Result<i64, Self::Error> {
        query_last_action_col(&self.session.last_action)
    }

    fn count_in_rect(&self, rect: Rect) -> Result<i64, Self::Error> {
        query_count_in_rect(rect, &self.session.placed_pieces)
    }

    fn count_piece_in_rect(
        &self,
        piece: (PieceModel, PieceColor),
        rect: Rect,
    ) -> Result<i64, Self::Error> {
        query_count_piece_in_rect(piece, rect, &self.session.placed_pieces)
    }

    fn model_at_pos(&self, pos: Pos) -> Result<PieceModel, Self::Error> {
        query_model_at_pos(&self.session.placed_pieces, pos)
    }

    fn color_at_pos(&self, pos: Pos) -> Result<PieceColor, Self::Error> {
        query_color_at_pos(&self.session.placed_pieces, pos)
    }

    fn moving_model(&self) -> Result<PieceModel, Self::Error> {
        Ok(self.moving_model)
    }

    fn moving_color(&self) -> Result<PieceColor, Self::Error> {
        Ok(self.moving_color)
    }

    fn source_row(&self) -> Result<i64, Self::Error> {
        Ok(self.source_pos.row())
    }

    fn source_col(&self) -> Result<i64, Self::Error> {
        Ok(self.source_pos.col())
    }

    fn target_row(&self) -> Result<i64, Self::Error> {
        Ok(self.target_pos.row())
    }

    fn target_col(&self) -> Result<i64, Self::Error> {
        Ok(self.target_pos.col())
    }
}
