use crate::rules::expr::ExprError;

/// Arithmetic expression.
#[derive(Debug)]
pub enum ArithExpr {
    Const(i64),
    Add(Box<ArithExpr>, Box<ArithExpr>),
    Sub(Box<ArithExpr>, Box<ArithExpr>),
    Mul(Box<ArithExpr>, Box<ArithExpr>),
    Div(Box<ArithExpr>, Box<ArithExpr>),
}

impl ArithExpr {
    /// Evaluates the arithmetic expression.
    pub fn evaluate(&self) -> Result<i64, ExprError> {
        match self {
            ArithExpr::Const(n) => Ok(*n),
            ArithExpr::Add(lhs, rhs) => Ok(lhs.evaluate()? + rhs.evaluate()?),
            ArithExpr::Sub(lhs, rhs) => Ok(lhs.evaluate()? - rhs.evaluate()?),
            ArithExpr::Mul(lhs, rhs) => Ok(lhs.evaluate()? * rhs.evaluate()?),
            ArithExpr::Div(lhs, rhs) => {
                let num = lhs.evaluate()?;
                let denom = rhs.evaluate()?;

                if denom == 0 {
                    Err(ExprError::DivisionByZero)
                } else {
                    Ok(num / denom)
                }
            }
        }
    }
}

/// Arithmetic comparison operator.
#[derive(Debug, Clone, Copy)]
pub enum ArithCmpOp {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug)]
pub struct ArithCmpExpr {
    lhs: ArithExpr,
    cmp: ArithCmpOp,
    rhs: ArithExpr,
}

impl ArithCmpExpr {
    /// Evaluates the arithmetic comparison expression.
    pub fn evaluate(&self) -> Result<bool, ExprError> {
        let lhs = self.lhs.evaluate()?;
        let rhs = self.rhs.evaluate()?;

        Ok(match self.cmp {
            ArithCmpOp::Equal => lhs == rhs,
            ArithCmpOp::NotEqual => lhs != rhs,
            ArithCmpOp::LessThan => lhs < rhs,
            ArithCmpOp::LessThanOrEqual => lhs <= rhs,
            ArithCmpOp::GreaterThan => lhs > rhs,
            ArithCmpOp::GreaterThanOrEqual => lhs >= rhs,
        })
    }
}
