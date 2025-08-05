use thiserror::Error;

pub mod boolean;
pub mod integer;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct QueryError(pub String);

/// Context for evaluating expressions.
pub trait Context {
    /// Boolean variable type associated with the context.
    type BoolVar;
    /// Integer variable type associated with the context.
    type IntVar;

    /// Query the value of a boolean variable.
    fn query_bool(&self, var: &Self::BoolVar) -> Result<bool, QueryError>;

    /// Query the value of an int variable.
    fn query_int(&self, var: &Self::IntVar) -> Result<i64, QueryError>;
}
