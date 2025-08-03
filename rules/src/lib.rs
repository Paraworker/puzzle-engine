use crate::{
    board::BoardRuleSet, conditions::game_over::GameOverCondition, expr::QueryError,
    initial_layout::InitialLayout, piece::PieceRuleSet, player::PlayerRuleSet, utils::load_ron,
};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod board;
pub mod conditions;
pub mod count;
pub mod expr;
pub mod initial_layout;
pub mod piece;
pub mod player;
pub mod position;
pub mod utils;

#[derive(Debug, Error)]
pub enum RulesError {
    #[error("division by zero")]
    DivisionByZero,
    #[error("query variable error: {0}")]
    Query(#[from] QueryError),
    #[error("piece count is depleted")]
    CountDepleted,
    #[error("rules format error: {0}")]
    Format(#[from] SpannedError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rules {
    pub board: BoardRuleSet,
    pub pieces: PieceRuleSet,
    pub players: PlayerRuleSet,
    pub initial_layout: InitialLayout,
    pub game_over_condition: GameOverCondition,
}

impl Rules {
    pub fn load<P>(path: P) -> Result<Self, RulesError>
    where
        P: AsRef<Path>,
    {
        load_ron(path)
    }
}
