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
    /// Literal true value.
    True,
    /// Literal false value.
    False,

    /// Logical AND.
    And(Vec<BoolExpr>),
    /// Logical OR.
    Or(Vec<BoolExpr>),
    /// Logical NOT.
    Not(Box<BoolExpr>),

    /// Compare if two integers are equal.
    Equal(Box<IntExpr>, Box<IntExpr>),
    /// Compare if two integers are not equal.
    NotEqual(Box<IntExpr>, Box<IntExpr>),
    /// Compare if the first integer is less than the second.
    LessThan(Box<IntExpr>, Box<IntExpr>),
    /// Compare if the first integer is greater than the second.
    GreaterThan(Box<IntExpr>, Box<IntExpr>),
    /// Compare if the first integer is less than or equal to the second.
    LessOrEqual(Box<IntExpr>, Box<IntExpr>),
    /// Compare if the first integer is greater than or equal to the second.
    GreaterOrEqual(Box<IntExpr>, Box<IntExpr>),

    /// Conditional expression
    ///
    /// (condition, then, otherwise)
    If(Box<BoolExpr>, Box<BoolExpr>, Box<BoolExpr>),

    /// Compare if two colors are equal.
    ColorEqual(Box<ColorExpr>, Box<ColorExpr>),

    /// Compare if two models are equal.
    ModelEqual(Box<ModelExpr>, Box<ModelExpr>),

    /// Query if the given position is occupied by any piece.
    PosOccupied(Box<IntExpr>, Box<IntExpr>),

    /// Query if the last action has been performed.
    HasLastAction,

    /// Query if the player's state is equal to the given state (Game over only).
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
            BoolExpr::If(cond, then, otherwise) => Self::conditional(cond, then, otherwise, ctx),
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

    fn and<C>(xs: &[BoolExpr], ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        if xs.len() < 2 {
            return Err(RulesError::AndInvalidArity.into());
        }

        for expr in xs {
            if !expr.evaluate(ctx)? {
                // Short-circuit
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn or<C>(xs: &[BoolExpr], ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        if xs.len() < 2 {
            return Err(RulesError::OrInvalidArity.into());
        }

        for expr in xs {
            if expr.evaluate(ctx)? {
                // Short-circuit
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn conditional<C>(
        condition: &BoolExpr,
        then: &BoolExpr,
        otherwise: &BoolExpr,
        ctx: &C,
    ) -> Result<bool, C::Error>
    where
        C: Context,
    {
        if condition.evaluate(ctx)? {
            then.evaluate(ctx)
        } else {
            otherwise.evaluate(ctx)
        }
    }
}
