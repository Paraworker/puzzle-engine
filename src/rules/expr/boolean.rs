use crate::{
    GameError,
    rules::expr::{ExprContext, arith::ArithExpr},
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

    /// Arithmetic operators
    Equal(ArithExpr, ArithExpr),
    NotEqual(ArithExpr, ArithExpr),
    LessThan(ArithExpr, ArithExpr),
    GreaterThan(ArithExpr, ArithExpr),
    LessOrEqual(ArithExpr, ArithExpr),
    GreaterOrEqual(ArithExpr, ArithExpr),
}

impl BoolExpr {
    /// Evaluates the boolean expression.
    pub fn evaluate(&self, ctx: &ExprContext) -> Result<bool, GameError> {
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
        }
    }
}
