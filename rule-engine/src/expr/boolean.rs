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
    Equal(Box<IntExpr>, Box<IntExpr>),
    NotEqual(Box<IntExpr>, Box<IntExpr>),
    LessThan(Box<IntExpr>, Box<IntExpr>),
    GreaterThan(Box<IntExpr>, Box<IntExpr>),
    LessOrEqual(Box<IntExpr>, Box<IntExpr>),
    GreaterOrEqual(Box<IntExpr>, Box<IntExpr>),

    /// Conditional expression
    ///
    /// (condition, then, otherwise)
    If(Box<BoolExpr>, Box<BoolExpr>, Box<BoolExpr>),

    /// Color operators
    ColorEqual(Box<ColorExpr>, Box<ColorExpr>),

    /// Model operators
    ModelEqual(Box<ModelExpr>, Box<ModelExpr>),

    /// Query board state
    ///
    /// - PosOccupied: If the given position is occupied by any piece.
    PosOccupied(Box<IntExpr>, Box<IntExpr>),

    /// Query last action information
    ///
    /// - HasLastAction: If the last action has been performed.
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
