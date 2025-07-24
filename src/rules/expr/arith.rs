use crate::{GameError, rules::expr::ExprContext};
use serde::{Deserialize, Serialize};

/// Arithmetic expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum ArithExpr {
    /// Operator
    Add(Box<ArithExpr>, Box<ArithExpr>),
    Sub(Box<ArithExpr>, Box<ArithExpr>),
    Mul(Box<ArithExpr>, Box<ArithExpr>),
    Div(Box<ArithExpr>, Box<ArithExpr>),

    /// Const number
    Const(i64),

    /// Variables
    SourceCol,
    SourceRow,
    TargetCol,
    TargetRow,
}

impl ArithExpr {
    /// Evaluates the arithmetic expression.
    pub fn evaluate(&self, ctx: &ExprContext) -> Result<i64, GameError> {
        match self {
            ArithExpr::Add(lhs, rhs) => Ok(lhs.evaluate(ctx)? + rhs.evaluate(ctx)?),
            ArithExpr::Sub(lhs, rhs) => Ok(lhs.evaluate(ctx)? - rhs.evaluate(ctx)?),
            ArithExpr::Mul(lhs, rhs) => Ok(lhs.evaluate(ctx)? * rhs.evaluate(ctx)?),
            ArithExpr::Div(lhs, rhs) => Self::div(lhs, rhs, ctx),
            ArithExpr::Const(n) => Ok(*n),
            ArithExpr::SourceCol => Ok(ctx.source.col()),
            ArithExpr::SourceRow => Ok(ctx.source.row()),
            ArithExpr::TargetCol => Ok(ctx.target.col()),
            ArithExpr::TargetRow => Ok(ctx.target.row()),
        }
    }

    fn div(lhs: &ArithExpr, rhs: &ArithExpr, ctx: &ExprContext) -> Result<i64, GameError> {
        let num = lhs.evaluate(ctx)?;
        let denom = rhs.evaluate(ctx)?;

        if denom == 0 {
            Err(GameError::ExprEval)
        } else {
            Ok(num / denom)
        }
    }
}
