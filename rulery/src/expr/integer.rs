use crate::{
    RulesError,
    expr::{Context, boolean::BoolExpr, color::ColorExpr, model::ModelExpr},
    pos::Pos,
    rect::Rect,
    utils::{from_ron_str, to_ron_str},
};
use serde::{Deserialize, Serialize};

/// Integer expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum IntExpr {
    /// A constant integer value.
    Const(i64),

    /// Addition
    Add(Box<IntExpr>, Box<IntExpr>),
    /// Subtraction
    Sub(Box<IntExpr>, Box<IntExpr>),
    /// Multiplication
    Mul(Box<IntExpr>, Box<IntExpr>),
    /// Division
    Div(Box<IntExpr>, Box<IntExpr>),
    /// Absolute value
    Abs(Box<IntExpr>),

    /// Conditional expression
    ///
    /// (condition, then, otherwise)
    If(Box<BoolExpr>, Box<IntExpr>, Box<IntExpr>),

    /// Query the current turn number.
    TurnNumber,
    /// Query the current round number.
    RoundNumber,

    /// Query the row of the last action.
    LastActionRow,
    /// Query the column of the last action.
    LastActionCol,

    /// Query the number of pieces in the given rectangle.
    CountInRect((Box<IntExpr>, Box<IntExpr>), (Box<IntExpr>, Box<IntExpr>)),
    /// Query the number of pieces with the given model and color in the given rectangle.
    CountPieceInRect(
        (Box<ModelExpr>, Box<ColorExpr>),
        (Box<IntExpr>, Box<IntExpr>),
        (Box<IntExpr>, Box<IntExpr>),
    ),

    /// Query the source tile row (Movement only).
    SourceRow,
    /// Query the source tile column (Movement only).
    SourceCol,
    /// Query the destination tile row (Movement only).
    TargetRow,
    /// Query the destination tile column (Movement only).
    TargetCol,

    /// Query the row where the piece is being placed (Placement only).
    ToPlaceRow,
    /// Query the column where the piece is being placed (Placement only).
    ToPlaceCol,
}

impl IntExpr {
    /// Evaluates the arithmetic expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<i64, C::Error>
    where
        C: Context,
    {
        match self {
            IntExpr::Const(n) => Ok(*n),
            IntExpr::Add(lhs, rhs) => Ok(lhs.evaluate(ctx)? + rhs.evaluate(ctx)?),
            IntExpr::Sub(lhs, rhs) => Ok(lhs.evaluate(ctx)? - rhs.evaluate(ctx)?),
            IntExpr::Mul(lhs, rhs) => Ok(lhs.evaluate(ctx)? * rhs.evaluate(ctx)?),
            IntExpr::Div(lhs, rhs) => Self::div(lhs, rhs, ctx),
            IntExpr::Abs(expr) => Ok(expr.evaluate(ctx)?.abs()),
            IntExpr::If(cond, then, otherwise) => Self::conditional(cond, then, otherwise, ctx),
            IntExpr::TurnNumber => ctx.turn_number(),
            IntExpr::RoundNumber => ctx.round_number(),
            IntExpr::LastActionRow => ctx.last_action_row(),
            IntExpr::LastActionCol => ctx.last_action_col(),
            IntExpr::CountInRect((row1, col1), (row2, col2)) => {
                Self::count_in_rect((row1, col1), (row2, col2), ctx)
            }
            IntExpr::CountPieceInRect((model, color), (row1, col1), (row2, col2)) => {
                Self::count_piece_in_rect((model, color), (row1, col1), (row2, col2), ctx)
            }
            IntExpr::SourceRow => ctx.source_row(),
            IntExpr::SourceCol => ctx.source_col(),
            IntExpr::TargetRow => ctx.target_row(),
            IntExpr::TargetCol => ctx.target_col(),
            IntExpr::ToPlaceRow => ctx.to_place_row(),
            IntExpr::ToPlaceCol => ctx.to_place_col(),
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

    /// Evaluates the division operation.
    fn div<C>(lhs: &IntExpr, rhs: &IntExpr, ctx: &C) -> Result<i64, C::Error>
    where
        C: Context,
    {
        let num = lhs.evaluate(ctx)?;
        let denom = rhs.evaluate(ctx)?;

        if denom == 0 {
            Err(RulesError::DivisionByZero.into())
        } else {
            Ok(num / denom)
        }
    }

    fn count_in_rect<C>(
        pos1: (&IntExpr, &IntExpr),
        pos2: (&IntExpr, &IntExpr),
        ctx: &C,
    ) -> Result<i64, C::Error>
    where
        C: Context,
    {
        ctx.count_in_rect(Rect::new(
            Pos::new(pos1.0.evaluate(ctx)?, pos1.1.evaluate(ctx)?),
            Pos::new(pos2.0.evaluate(ctx)?, pos2.1.evaluate(ctx)?),
        ))
    }

    fn count_piece_in_rect<C>(
        piece: (&ModelExpr, &ColorExpr),
        pos1: (&IntExpr, &IntExpr),
        pos2: (&IntExpr, &IntExpr),
        ctx: &C,
    ) -> Result<i64, C::Error>
    where
        C: Context,
    {
        ctx.count_piece_in_rect(
            (piece.0.evaluate(ctx)?, piece.1.evaluate(ctx)?),
            Rect::new(
                Pos::new(pos1.0.evaluate(ctx)?, pos1.1.evaluate(ctx)?),
                Pos::new(pos2.0.evaluate(ctx)?, pos2.1.evaluate(ctx)?),
            ),
        )
    }

    fn conditional<C>(
        condition: &BoolExpr,
        then: &IntExpr,
        otherwise: &IntExpr,
        ctx: &C,
    ) -> Result<i64, C::Error>
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
