use crate::{
    RulesError,
    expr::{Context, color::ColorExpr, integer::IntExpr, model::ModelExpr},
    piece::PieceColor,
    player::PlayerState,
    pos::Pos,
    utils::{from_ron_str, to_ron_str},
};
use serde::{Deserialize, Serialize};

/// Boolean expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum BoolExpr {
    /// Literal
    True,
    False,

    /// Logical operators
    And(Vec<BoolExpr>),
    Or(Vec<BoolExpr>),
    Not(Box<BoolExpr>),

    /// Arithmetic comparison operators
    Equal(IntExpr, IntExpr),
    NotEqual(IntExpr, IntExpr),
    LessThan(IntExpr, IntExpr),
    GreaterThan(IntExpr, IntExpr),
    LessOrEqual(IntExpr, IntExpr),
    GreaterOrEqual(IntExpr, IntExpr),

    /// Color operators
    ColorEqual(ColorExpr, ColorExpr),

    /// Model operators
    ModelEqual(ModelExpr, ModelExpr),

    /// Query board state
    ///
    /// - PosOccupied: If the given position is occupied by any piece.
    PosOccupied(IntExpr, IntExpr),

    /// Query last action information
    ///
    /// - HasLastAction: If the first action has been performed.
    HasLastAction,

    /// Game over only
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
            BoolExpr::And(vec) => Self::and(vec, ctx),
            BoolExpr::Or(vec) => Self::or(vec, ctx),
            BoolExpr::Not(expr) => Ok(!expr.evaluate(ctx)?),
            BoolExpr::Equal(lhs, rhs) => Ok(lhs.evaluate(ctx)? == rhs.evaluate(ctx)?),
            BoolExpr::NotEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? != rhs.evaluate(ctx)?),
            BoolExpr::LessThan(lhs, rhs) => Ok(lhs.evaluate(ctx)? < rhs.evaluate(ctx)?),
            BoolExpr::GreaterThan(lhs, rhs) => Ok(lhs.evaluate(ctx)? > rhs.evaluate(ctx)?),
            BoolExpr::LessOrEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? <= rhs.evaluate(ctx)?),
            BoolExpr::GreaterOrEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? >= rhs.evaluate(ctx)?),
            BoolExpr::ColorEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? == rhs.evaluate(ctx)?),
            BoolExpr::ModelEqual(lhs, rhs) => Ok(lhs.evaluate(ctx)? == rhs.evaluate(ctx)?),
            BoolExpr::PosOccupied(row, col) => {
                ctx.pos_occupied(Pos::new(row.evaluate(ctx)?, col.evaluate(ctx)?))
            }
            BoolExpr::HasLastAction => ctx.has_last_action(),
            BoolExpr::PlayerStateEqual(color, state) => ctx.player_state_equal(*color, *state),
        }
    }

    /// Parses from a ron string.
    pub fn from_ron_str(str: &str) -> Result<Self, RulesError> {
        from_ron_str(str)
    }

    /// Converts into a ron string.
    pub fn to_ron_str(&self) -> Result<String, RulesError> {
        to_ron_str(self)
    }

    fn and<C>(vec: &Vec<BoolExpr>, ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        if vec.len() < 2 {
            return Err(RulesError::AndInvalidArity.into());
        }

        for expr in vec {
            if !expr.evaluate(ctx)? {
                // Short-circuit
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn or<C>(vec: &Vec<BoolExpr>, ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        if vec.len() < 2 {
            return Err(RulesError::OrInvalidArity.into());
        }

        for expr in vec {
            if expr.evaluate(ctx)? {
                // Short-circuit
                return Ok(true);
            }
        }

        Ok(false)
    }
}
