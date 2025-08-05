use crate::{RulesError, expr::Context};
use serde::{Deserialize, Serialize};

/// Integer expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum IntExpr<I> {
    /// A constant integer value.
    Const(i64),

    /// Addition
    Add(Box<IntExpr<I>>, Box<IntExpr<I>>),
    /// Subtraction
    Sub(Box<IntExpr<I>>, Box<IntExpr<I>>),
    /// Multiplication
    Mul(Box<IntExpr<I>>, Box<IntExpr<I>>),
    /// Division
    Div(Box<IntExpr<I>>, Box<IntExpr<I>>),

    /// Query from variable
    Query(I),
}

impl<I> IntExpr<I> {
    /// Evaluates the arithmetic expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<i64, RulesError>
    where
        C: Context<IntVar = I>,
    {
        match self {
            IntExpr::Const(n) => Ok(*n),
            IntExpr::Add(lhs, rhs) => Ok(lhs.evaluate(ctx)? + rhs.evaluate(ctx)?),
            IntExpr::Sub(lhs, rhs) => Ok(lhs.evaluate(ctx)? - rhs.evaluate(ctx)?),
            IntExpr::Mul(lhs, rhs) => Ok(lhs.evaluate(ctx)? * rhs.evaluate(ctx)?),
            IntExpr::Div(lhs, rhs) => Self::div(lhs, rhs, ctx),
            IntExpr::Query(var) => Ok(ctx.query_int(var)?),
        }
    }

    /// Evaluates the division operation.
    fn div<C>(lhs: &IntExpr<I>, rhs: &IntExpr<I>, ctx: &C) -> Result<i64, RulesError>
    where
        C: Context<IntVar = I>,
    {
        let num = lhs.evaluate(ctx)?;
        let denom = rhs.evaluate(ctx)?;

        if denom == 0 {
            Err(RulesError::DivisionByZero)
        } else {
            Ok(num / denom)
        }
    }
}
