use crate::{
    RulesError,
    expr::{Context, integer::IntExpr},
};
use serde::{Deserialize, Serialize};

/// Boolean expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum BoolExpr<B, I> {
    /// Literal
    True,
    False,

    /// Logical operators
    And(Box<BoolExpr<B, I>>, Box<BoolExpr<B, I>>),
    Or(Box<BoolExpr<B, I>>, Box<BoolExpr<B, I>>),
    Not(Box<BoolExpr<B, I>>),

    /// Arithmetic comparison operators
    Equal(IntExpr<I>, IntExpr<I>),
    NotEqual(IntExpr<I>, IntExpr<I>),
    LessThan(IntExpr<I>, IntExpr<I>),
    GreaterThan(IntExpr<I>, IntExpr<I>),
    LessOrEqual(IntExpr<I>, IntExpr<I>),
    GreaterOrEqual(IntExpr<I>, IntExpr<I>),

    /// Query from variable
    Query(B),
}

impl<B, I> BoolExpr<B, I> {
    /// Evaluates the boolean expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<bool, RulesError>
    where
        C: Context<BoolVar = B, IntVar = I>,
    {
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
            BoolExpr::Query(var) => Ok(ctx.query_bool(var)?),
        }
    }
}
