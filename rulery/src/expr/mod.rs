use crate::{
    RulesError,
    piece::{PieceColor, PieceModel},
    player::PlayerState,
    pos::Pos,
    rect::Rect,
};

pub mod boolean;
pub mod color;
pub mod integer;
pub mod model;

/// Context for evaluating expressions.
pub trait Context {
    type Error: From<RulesError>;

    /// Query whether a position is occupied.
    fn pos_occupied(&self, pos: Pos) -> Result<bool, Self::Error>;

    /// Query whether the first action has been performed.
    fn has_last_action(&self) -> Result<bool, Self::Error>;

    /// Query the current turn number.
    fn turn_number(&self) -> Result<i64, Self::Error>;

    /// Query the current round number.
    fn round_number(&self) -> Result<i64, Self::Error>;

    /// Query the row of the last action position.
    ///
    /// If no last action has been performed, return an error.
    fn last_action_row(&self) -> Result<i64, Self::Error>;

    /// Query the column of the last action position.
    ///
    /// If no last action has been performed, return an error.
    fn last_action_col(&self) -> Result<i64, Self::Error>;

    /// Query the number of pieces in a rectangle defined by two positions.
    fn count_in_rect(&self, rect: Rect) -> Result<i64, Self::Error>;

    /// Query the number of pieces of a specific model and color in a rectangle defined by two positions.
    fn count_piece_in_rect(
        &self,
        piece: (PieceModel, PieceColor),
        rect: Rect,
    ) -> Result<i64, Self::Error>;

    /// Query the model of the piece at a specific position.
    fn model_at_pos(&self, pos: Pos) -> Result<PieceModel, Self::Error>;

    /// Query the color of the piece at a specific position.
    fn color_at_pos(&self, pos: Pos) -> Result<PieceColor, Self::Error>;

    /// Query the model of the piece being moved.
    ///
    /// Only support in movement.
    fn moving_model(&self) -> Result<PieceModel, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the color of the piece being moved.
    ///
    /// Only support in movement.
    fn moving_color(&self) -> Result<PieceColor, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the row of the source position.
    ///
    /// Only support in movement.
    fn source_row(&self) -> Result<i64, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the column of the source position.
    ///
    /// Only support in movement.
    fn source_col(&self) -> Result<i64, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the row of the target position.
    ///
    /// Only support in movement.
    fn target_row(&self) -> Result<i64, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the column of the target position.
    ///
    /// Only support in movement.
    fn target_col(&self) -> Result<i64, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the model of the piece to place.
    ///
    /// Only support in placement.
    fn to_place_model(&self) -> Result<PieceModel, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the color of the piece to place.
    ///
    /// Only support in placement.
    fn to_place_color(&self) -> Result<PieceColor, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the row of the piece to place.
    ///
    /// Only support in placement.
    fn to_place_row(&self) -> Result<i64, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the column of the piece to place.
    ///
    /// Only support in placement.
    fn to_place_col(&self) -> Result<i64, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the state of the player with the given color.
    ///
    /// Only support in game over.
    fn player_state_equal(
        &self,
        _color: PieceColor,
        _state: PlayerState,
    ) -> Result<bool, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }
}
