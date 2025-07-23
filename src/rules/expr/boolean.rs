use crate::rules::expr::{ExprError, arith::ArithCmpExpr};

/// Boolean expression.
#[derive(Debug)]
pub enum BoolExpr {
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    Not(Box<BoolExpr>),
    ArithCmp(ArithCmpExpr),
}

impl BoolExpr {
    /// Evaluates the boolean expression.
    pub fn evaluate(&self) -> Result<bool, ExprError> {
        match self {
            BoolExpr::And(lhs, rhs) => Ok(lhs.evaluate()? && rhs.evaluate()?),
            BoolExpr::Or(lhs, rhs) => Ok(lhs.evaluate()? || rhs.evaluate()?),
            BoolExpr::Not(expr) => Ok(!expr.evaluate()?),
            BoolExpr::ArithCmp(expr) => expr.evaluate(),
        }
    }
}
