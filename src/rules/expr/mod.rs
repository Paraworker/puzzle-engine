use thiserror::Error;

pub mod arith;
pub mod boolean;

#[derive(Debug, Error)]
pub enum ExprError {
    #[error("division by zero")]
    DivisionByZero,
}
