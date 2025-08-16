use crate::{
    GameError,
    expr_contexts::{
        query_color_at_pos, query_count_in_rect, query_count_piece_in_rect, query_has_last_action,
        query_last_action_col, query_last_action_row, query_model_at_pos, query_pos_occupied,
        query_round_number, query_turn_number,
    },
    states::playing::{
        piece::PlacedPiece,
        session::{piece_index::PlacedPieceIndex, turn::TurnController},
    },
};
use bevy::ecs::system::Query;
use rule_engine::{
    expr::Context,
    piece::{PieceColor, PieceModel},
    pos::Pos,
    rect::Rect,
};

#[derive(Debug)]
pub struct WinOrLoseContext<'t, 'l, 'i, 'world, 'state, 'data> {
    pub turn: &'t TurnController,
    pub last_action: &'l Option<Pos>,
    pub placed_piece_index: &'i PlacedPieceIndex,
    pub placed_piece_query: Query<'world, 'state, &'data PlacedPiece>,
}

impl Context for WinOrLoseContext<'_, '_, '_, '_, '_, '_> {
    type Error = GameError;

    fn pos_occupied(&self, pos: Pos) -> Result<bool, Self::Error> {
        query_pos_occupied(&self.placed_piece_index, pos)
    }

    fn has_last_action(&self) -> Result<bool, Self::Error> {
        query_has_last_action(self.last_action)
    }

    fn turn_number(&self) -> Result<i64, Self::Error> {
        query_turn_number(&self.turn)
    }

    fn round_number(&self) -> Result<i64, Self::Error> {
        query_round_number(&self.turn)
    }

    fn last_action_row(&self) -> Result<i64, Self::Error> {
        query_last_action_row(self.last_action)
    }

    fn last_action_col(&self) -> Result<i64, Self::Error> {
        query_last_action_col(self.last_action)
    }

    fn count_in_rect(&self, rect: Rect) -> Result<i64, Self::Error> {
        query_count_in_rect(rect, &self.placed_piece_index)
    }

    fn count_piece_in_rect(
        &self,
        piece: (PieceModel, PieceColor),
        rect: Rect,
    ) -> Result<i64, Self::Error> {
        query_count_piece_in_rect(
            piece,
            rect,
            &self.placed_piece_index,
            self.placed_piece_query,
        )
    }

    fn model_at_pos(&self, pos: Pos) -> Result<PieceModel, Self::Error> {
        query_model_at_pos(&self.placed_piece_index, self.placed_piece_query, pos)
    }

    fn color_at_pos(&self, pos: Pos) -> Result<PieceColor, Self::Error> {
        query_color_at_pos(&self.placed_piece_index, self.placed_piece_query, pos)
    }
}
