use crate::{
    rules::{
        board::BoardRuleSet, initial_layout::InitialLayout, piece::PieceRuleSet,
        player::PlayerRuleSet,
    },
    utils::load_ron,
};
use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod board;
pub mod expr;
pub mod initial_layout;
pub mod piece;
pub mod player;

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct GameRules {
    pub board: BoardRuleSet,
    pub pieces: PieceRuleSet,
    pub players: PlayerRuleSet,
    pub initial_layout: InitialLayout,
}

impl GameRules {
    pub fn load<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        load_ron(path)
    }
}
