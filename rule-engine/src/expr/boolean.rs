use crate::{
    expr::{Context, integer::IntExpr},
    piece::{PieceColor, PieceModel},
    player::PlayerState,
    position::Pos,
};
use serde::{Deserialize, Serialize};

/// Boolean expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum BoolExpr {
    /// Literal
    True,
    False,

    /// Logical operators
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    Not(Box<BoolExpr>),

    /// Arithmetic comparison operators
    Equal(IntExpr, IntExpr),
    NotEqual(IntExpr, IntExpr),
    LessThan(IntExpr, IntExpr),
    GreaterThan(IntExpr, IntExpr),
    LessOrEqual(IntExpr, IntExpr),
    GreaterOrEqual(IntExpr, IntExpr),

    /// Query board state
    ///
    /// - PosOccupied: If the given position is occupied by any piece.
    /// - ModelAtPosEqual: If the piece at the given position is equal to the given model.
    /// - ColorAtPosEqual: If the piece at the given position is equal to the given color.
    PosOccupied(IntExpr, IntExpr),
    ModelAtPosEqual((IntExpr, IntExpr), PieceModel),
    ColorAtPosEqual((IntExpr, IntExpr), PieceColor),

    /// Movement expression only
    ///
    /// - MovingModelEqual: If the moving piece's model is equal to the given model.
    /// - MovingColorEqual: If the moving piece's color is equal to the given color.
    MovingModelEqual(PieceModel),
    MovingColorEqual(PieceColor),

    /// Placement expression only
    ///
    /// - ToPlaceModelEqual: If the model of the piece being placed is equal to the given model.
    /// - ToPlaceColorEqual: If the color of the piece being placed is equal to the given color.
    ToPlaceModelEqual(PieceModel),
    ToPlaceColorEqual(PieceColor),

    /// Game over expression only
    ///
    /// - PlayerStateEqual: If the player's state is equal to the given state.
    PlayerStateEqual(PieceColor, PlayerState),
}

impl BoolExpr {
    /// Evaluates the boolean expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        match self {
            BoolExpr::True => Ok(true),
            BoolExpr::False => Ok(false),
            BoolExpr::And(lhs, rhs) => Ok(lhs.evaluate(ctx)? && rhs.evaluate(ctx)?),
            BoolExpr::Or(lhs, rhs) => Ok(lhs.evaluate(ctx)? || rhs.evaluate(ctx)?),
            BoolExpr::Not(expr) => Ok(!expr.evaluate(ctx)?),
            BoolExpr::Equal(lhs, rhs) => Ok(lhs.evaluate(ctx)? == rhs.evaluate(ctx)?),
            BoolExpr::NotEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? != rhs.evaluate(ctx)?),
            BoolExpr::LessThan(lhs, rhs) => Ok(lhs.evaluate(ctx)? < rhs.evaluate(ctx)?),
            BoolExpr::GreaterThan(lhs, rhs) => Ok(lhs.evaluate(ctx)? > rhs.evaluate(ctx)?),
            BoolExpr::LessOrEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? <= rhs.evaluate(ctx)?),
            BoolExpr::GreaterOrEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? >= rhs.evaluate(ctx)?),
            BoolExpr::PosOccupied(row, col) => {
                ctx.pos_occupied(Pos::new(row.evaluate(ctx)?, col.evaluate(ctx)?))
            }
            BoolExpr::ModelAtPosEqual((row, col), model) => {
                ctx.model_at_pos_equal(Pos::new(row.evaluate(ctx)?, col.evaluate(ctx)?), *model)
            }
            BoolExpr::ColorAtPosEqual((row, col), color) => {
                ctx.color_at_pos_equal(Pos::new(row.evaluate(ctx)?, col.evaluate(ctx)?), *color)
            }
            BoolExpr::MovingModelEqual(model) => ctx.moving_model_equal(*model),
            BoolExpr::MovingColorEqual(color) => ctx.moving_color_equal(*color),
            BoolExpr::ToPlaceModelEqual(model) => ctx.to_place_model_equal(*model),
            BoolExpr::ToPlaceColorEqual(color) => ctx.to_place_color_equal(*color),
            BoolExpr::PlayerStateEqual(color, state) => ctx.player_state_equal(*color, *state),
        }
    }
}
