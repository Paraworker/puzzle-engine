use crate::{
    RulesError,
    expr::Context,
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

    /// Query turn information
    ///
    /// - TurnNumber: The current turn number.
    /// - RoundNumber: The current round number.
    TurnNumber,
    RoundNumber,

    /// Query last action information
    ///
    /// - LastActionCol: The column of the last action.
    /// - LastActionRow: The row of the last action.
    LastActionCol,
    LastActionRow,

    /// Movement expression only variables
    ///
    /// - SourceRow: The source tile row.
    /// - SourceCol: The source tile column.
    /// - TargetRow: The destination tile row.
    /// - TargetCol: The destination tile column.
    SourceRow,
    SourceCol,
    TargetRow,
    TargetCol,

    /// Placement expression only variables
    ///
    /// - ToPlaceRow: The row where the piece is being placed.
    /// - ToPlaceCol: The column where the piece is being placed.
    ToPlaceRow,
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
            IntExpr::TurnNumber => ctx.turn_number(),
            IntExpr::RoundNumber => ctx.round_number(),
            IntExpr::LastActionRow => ctx.last_action_row(),
            IntExpr::LastActionCol => ctx.last_action_col(),
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
}
