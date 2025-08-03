use crate::rules::{
    RulesError,
    expr::{ExprContext, ExprScenario},
};
use serde::{Deserialize, Serialize};

/// Arithmetic expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum ArithExpr {
    /// A constant integer value.
    Const(i64),

    /// Addition, e.g., `a + b`
    Add(Box<ArithExpr>, Box<ArithExpr>),
    /// Subtraction, e.g., `a - b`
    Sub(Box<ArithExpr>, Box<ArithExpr>),
    /// Multiplication, e.g., `a * b`
    Mul(Box<ArithExpr>, Box<ArithExpr>),
    /// Division, e.g., `a / b`
    Div(Box<ArithExpr>, Box<ArithExpr>),

    /// Current turn number.
    TurnNumber,
    /// Current round number.
    RoundNumber,

    /// Variables available only in [`ExprScenario::PieceMovement`]:
    /// - `SourceCol`, `SourceRow`: The source tile position.
    /// - `TargetCol`, `TargetRow`: The destination tile position.
    SourceCol,
    SourceRow,
    TargetCol,
    TargetRow,

    /// Variables available only in [`ExprScenario::PiecePlacement`]:
    /// - `ToPlaceCol`, `ToPlaceRow`: The position where the piece is being placed.
    ToPlaceCol,
    ToPlaceRow,
}

impl ArithExpr {
    /// Evaluates the arithmetic expression.
    pub fn evaluate(&self, ctx: &ExprContext) -> Result<i64, RulesError> {
        match self {
            ArithExpr::Const(n) => Ok(*n),
            ArithExpr::Add(lhs, rhs) => Ok(lhs.evaluate(ctx)? + rhs.evaluate(ctx)?),
            ArithExpr::Sub(lhs, rhs) => Ok(lhs.evaluate(ctx)? - rhs.evaluate(ctx)?),
            ArithExpr::Mul(lhs, rhs) => Ok(lhs.evaluate(ctx)? * rhs.evaluate(ctx)?),
            ArithExpr::Div(lhs, rhs) => Self::evaluate_div(lhs, rhs, ctx),
            ArithExpr::TurnNumber => Ok(ctx.turn_number),
            ArithExpr::RoundNumber => Ok(ctx.round_number),
            ArithExpr::SourceCol => Self::query_source_col(ctx),
            ArithExpr::SourceRow => Self::query_source_row(ctx),
            ArithExpr::TargetCol => Self::query_target_col(ctx),
            ArithExpr::TargetRow => Self::query_target_row(ctx),
            ArithExpr::ToPlaceCol => Self::query_to_place_col(ctx),
            ArithExpr::ToPlaceRow => Self::query_to_place_row(ctx),
        }
    }

    /// Evaluates the division operation.
    fn evaluate_div(lhs: &ArithExpr, rhs: &ArithExpr, ctx: &ExprContext) -> Result<i64, RulesError> {
        let num = lhs.evaluate(ctx)?;
        let denom = rhs.evaluate(ctx)?;

        if denom == 0 {
            Err(RulesError::DivisionByZero)
        } else {
            Ok(num / denom)
        }
    }

    /// Query `SourceCol`
    fn query_source_col(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { source, .. } = ctx.scenario {
            Ok(source.col())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `SourceRow`
    fn query_source_row(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { source, .. } = ctx.scenario {
            Ok(source.row())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `TargetCol`
    fn query_target_col(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { target, .. } = ctx.scenario {
            Ok(target.col())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `TargetRow`
    fn query_target_row(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { target, .. } = ctx.scenario {
            Ok(target.row())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `ToPlaceCol`
    fn query_to_place_col(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PiecePlacement { to_place, .. } = ctx.scenario {
            Ok(to_place.col())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `ToPlaceRow`
    fn query_to_place_row(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PiecePlacement { to_place, .. } = ctx.scenario {
            Ok(to_place.row())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }
}
