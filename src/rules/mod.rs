use crate::rules::board::BoardGeometry;
use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

pub mod board;
pub mod condition;
pub mod direction;
pub mod distance;
pub mod piece;

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct GameRules {
    board_geometry: BoardGeometry,
}

impl GameRules {
    pub fn new(board_geometry: BoardGeometry) -> Self {
        GameRules { board_geometry }
    }

    pub fn board_geometry(&self) -> &BoardGeometry {
        &self.board_geometry
    }
}
