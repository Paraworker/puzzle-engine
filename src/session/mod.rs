use crate::session::{state::SessionState, tiles::TileIndex};
use bevy::prelude::*;

pub mod state;
pub mod tiles;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: SessionState,
    pub tiles: TileIndex,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            state: SessionState::Navigating,
            tiles: TileIndex::new(),
        }
    }
}
