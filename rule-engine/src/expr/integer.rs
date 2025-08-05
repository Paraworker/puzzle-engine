use crate::{RulesError, expr::Context};
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

    /// Query turn info
    ///
    /// - TurnNumber: The current turn number.
    /// - RoundNumber: The current round number.
    TurnNumber,
    RoundNumber,

    /// Movement expression only
    ///
    /// - SourceRow: The source tile row.
    /// - SourceCol: The source tile column.
    /// - TargetRow: The destination tile row.
    /// - TargetCol: The destination tile column.
    SourceRow,
    SourceCol,
    TargetRow,
    TargetCol,

    /// Placement expression only
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
            IntExpr::TurnNumber => ctx.turn_number(),
            IntExpr::RoundNumber => ctx.round_number(),
            IntExpr::SourceRow => ctx.source_row(),
            IntExpr::SourceCol => ctx.source_col(),
            IntExpr::TargetRow => ctx.target_row(),
            IntExpr::TargetCol => ctx.target_col(),
            IntExpr::ToPlaceRow => ctx.to_place_row(),
            IntExpr::ToPlaceCol => ctx.to_place_col(),
        }
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
