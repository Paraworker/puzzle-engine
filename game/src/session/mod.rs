use crate::{
    session::{
        piece_index::PlacedPieceIndex, player::Players, state::SessionState, tile_index::TileIndex,
        turn::TurnController,
    },
    states::game_setup::LoadedRules,
};
use bevy::prelude::*;

pub mod piece_index;
pub mod player;
pub mod state;
pub mod tile_index;
pub mod turn;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: SessionState,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
    pub players: Players,
    pub turn: TurnController,
}

impl GameSession {
    pub fn new(rules: &LoadedRules) -> Self {
        Self {
            state: SessionState::Selecting,
            tiles: TileIndex::new(),
            placed_pieces: PlacedPieceIndex::new(),
            players: Players::new(&rules.players, &rules.pieces),
            turn: TurnController::new(),
        }
    }
}
