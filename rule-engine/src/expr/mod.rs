use crate::{
    RulesError,
    piece::{PieceColor, PieceModel},
    player::PlayerState,
    position::Pos,
};

pub mod boolean;
pub mod integer;

/// Context for evaluating expressions.
pub trait Context {
    type Error: From<RulesError>;

    /// Query the current turn number.
    fn turn_number(&self) -> Result<i64, Self::Error>;

    /// Query the current round number.
    fn round_number(&self) -> Result<i64, Self::Error>;

    /// Query whether a position is occupied.
    fn pos_occupied(&self, pos: Pos) -> Result<bool, Self::Error>;

    /// Query whether a position is occupied by a piece of a specific model.
    fn model_at_pos_equal(&self, pos: Pos, model: PieceModel) -> Result<bool, Self::Error>;

    /// Query whether a position is occupied by a piece of a specific color.
    fn color_at_pos_equal(&self, pos: Pos, color: PieceColor) -> Result<bool, Self::Error>;

    /// Query whether the first action has been performed.
    fn has_last_action(&self) -> Result<bool, Self::Error>;

    /// Query the row of the last action position.
    ///
    /// If no last action has been performed, return an error.
    fn last_action_row(&self) -> Result<i64, Self::Error>;

    /// Query the column of the last action position.
    ///
    /// If no last action has been performed, return an error.
    fn last_action_col(&self) -> Result<i64, Self::Error>;

    /// Query whether the moving piece is equal to a specific model.
    ///
    /// Only support in movement.
    fn moving_model_equal(&self, _model: PieceModel) -> Result<bool, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query whether the moving piece is equal to a specific color.
    ///
    /// Only support in movement.
    fn moving_color_equal(&self, _color: PieceColor) -> Result<bool, Self::Error> {
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
    fn to_place_model_equal(&self, _model: PieceModel) -> Result<bool, Self::Error> {
        Err(RulesError::UnsupportedVariable.into())
    }

    /// Query the color of the piece to place.
    ///
    /// Only support in placement.
    fn to_place_color_equal(&self, _color: PieceColor) -> Result<bool, Self::Error> {
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
