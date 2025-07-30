use crate::rules::RulesError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Count {
    Infinite,
    Finite(usize),
}

impl Count {
    pub fn decrease(&mut self) -> Result<Self, RulesError> {
        match self {
            Count::Infinite => Ok(*self),
            Count::Finite(count) => {
                if *count > 0 {
                    *count -= 1;
                    Ok(*self)
                } else {
                    Err(RulesError::CountDepleted)
                }
            }
        }
    }
}
