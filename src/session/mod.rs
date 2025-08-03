use crate::{
    rules::GameRules,
    session::{
        piece_index::PlacedPieceIndex, state::SessionState, tile_index::TileIndex,
        turn::TurnController,
    },
};
use bevy::prelude::*;

pub mod piece_index;
pub mod state;
pub mod tile_index;
pub mod turn;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: SessionState,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
    pub turn_controller: TurnController,
}

impl GameSession {
    pub fn new(rules: &GameRules) -> Self {
        Self {
            state: SessionState::Selecting,
            tiles: TileIndex::new(),
            placed_pieces: PlacedPieceIndex::new(),
            turn_controller: TurnController::new(&rules.players, &rules.pieces),
        }
    }
}
