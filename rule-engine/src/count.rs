use crate::RulesError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Count {
    Infinite,
    Finite(usize),
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Count::Infinite => write!(f, "Unlimited"),
            Count::Finite(n) => write!(f, "{n}"),
        }
    }
}

impl Count {
    pub fn decrease(&mut self) -> Result<(), RulesError> {
        match self {
            Count::Infinite => Ok(()),
            Count::Finite(count) => {
                if *count > 0 {
                    *count -= 1;
                    Ok(())
                } else {
                    Err(RulesError::CountDepleted)
                }
            }
        }
    }

    pub fn is_depleted(&self) -> bool {
        match self {
            Count::Infinite => false,
            Count::Finite(n) => *n == 0,
        }
    }
}
