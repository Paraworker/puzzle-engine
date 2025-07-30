use crate::{
    rules::GameRules,
    session::{
        piece_index::PlacedPieceIndex, player::Players, state::SessionState, tile_index::TileIndex,
    },
};
use bevy::prelude::*;

pub mod piece_index;
pub mod player;
pub mod state;
pub mod tile_index;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: SessionState,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
    pub players: Players,
}

impl GameSession {
    pub fn new(rules: &GameRules) -> Self {
        Self {
            state: SessionState::Navigating,
            tiles: TileIndex::new(),
            placed_pieces: PlacedPieceIndex::new(),
            players: Players::from_rules(&rules.players, &rules.pieces),
        }
    }
}
