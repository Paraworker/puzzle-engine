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
            ArithExpr::Div(lhs, rhs) => Self::div(lhs, rhs, ctx),
            ArithExpr::TurnNumber => Ok(ctx.session.turn_controller.turn_number()),
            ArithExpr::RoundNumber => Ok(ctx.session.turn_controller.round_number()),
            ArithExpr::SourceCol => Self::source_col(ctx),
            ArithExpr::SourceRow => Self::source_row(ctx),
            ArithExpr::TargetCol => Self::target_col(ctx),
            ArithExpr::TargetRow => Self::target_row(ctx),
            ArithExpr::ToPlaceCol => Self::to_place_col(ctx),
            ArithExpr::ToPlaceRow => Self::to_place_row(ctx),
        }
    }

    /// Evaluates the division operation.
    fn div(lhs: &ArithExpr, rhs: &ArithExpr, ctx: &ExprContext) -> Result<i64, RulesError> {
        let num = lhs.evaluate(ctx)?;
        let denom = rhs.evaluate(ctx)?;

        if denom == 0 {
            Err(RulesError::DivisionByZero)
        } else {
            Ok(num / denom)
        }
    }

    /// Query `SourceCol`
    fn source_col(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { source, .. } = ctx.scenario {
            Ok(source.col())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `SourceRow`
    fn source_row(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { source, .. } = ctx.scenario {
            Ok(source.row())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `TargetCol`
    fn target_col(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { target, .. } = ctx.scenario {
            Ok(target.col())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `TargetRow`
    fn target_row(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PieceMovement { target, .. } = ctx.scenario {
            Ok(target.row())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `ToPlaceCol`
    fn to_place_col(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PiecePlacement { to_place, .. } = ctx.scenario {
            Ok(to_place.col())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }

    /// Query `ToPlaceRow`
    fn to_place_row(ctx: &ExprContext) -> Result<i64, RulesError> {
        if let ExprScenario::PiecePlacement { to_place, .. } = ctx.scenario {
            Ok(to_place.row())
        } else {
            Err(RulesError::UnsupportedVariable)
        }
    }
}
