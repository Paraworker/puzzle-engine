use crate::{
    board::BoardRuleSet, expr::boolean::BoolExpr, initial_layout::InitialLayout,
    piece::PieceRuleSet, player::PlayerRuleSet, utils::load_ron,
};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod board;
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
    #[error("unsupported variable")]
    UnsupportedVariable,
    #[error("piece count is depleted")]
    CountDepleted,
    #[error("rules format error: {0}")]
    Format(#[from] SpannedError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Rules for a game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub name: String,
    pub board: BoardRuleSet,
    pub pieces: PieceRuleSet,
    pub players: PlayerRuleSet,
    pub initial_layout: InitialLayout,
    pub game_over_condition: BoolExpr,
}

impl GameRules {
    pub fn load<P>(path: P) -> Result<Self, RulesError>
    where
        P: AsRef<Path>,
    {
        load_ron(path)
    }
}
