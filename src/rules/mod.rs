use crate::rules::{
    board::BoardRuleSet, expr::boolean::BoolExpr, initial_layout::InitialLayout,
    piece::PieceRuleSet, player::PlayerRuleSet, utils::load_ron,
};
use bevy::ecs::resource::Resource;
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
    #[error("variable is not supported in this scenario")]
    UnsupportedVariable,
    #[error("piece count is depleted")]
    CountDepleted,
    #[error("rules format error: {0}")]
    Format(#[from] SpannedError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct GameRules {
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
