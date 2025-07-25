use crate::session::{pieces::PlacedPieceIndex, state::SessionState, tiles::TileIndex};
use bevy::prelude::*;

pub mod pieces;
pub mod state;
pub mod tiles;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: SessionState,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            state: SessionState::Navigating,
            tiles: TileIndex::new(),
            placed_pieces: PlacedPieceIndex::new(),
        }
    }
}
