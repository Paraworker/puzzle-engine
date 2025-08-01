use crate::{
    rules::GameRules,
    session::{
        piece_index::PlacedPieceIndex, player::Players, state::SessionState, text::TopPanelText,
        tile_index::TileIndex,
    },
};
use bevy::prelude::*;

pub mod piece_index;
pub mod player;
pub mod state;
pub mod text;
pub mod tile_index;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: SessionState,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
    pub players: Players,
    pub top_panel_text: TopPanelText,
}

impl GameSession {
    pub fn new(rules: &GameRules) -> Self {
        let players = Players::from_rules(&rules.players, &rules.pieces);
        let top_panel_text = TopPanelText::turn(players.current().0);

        Self {
            state: SessionState::Selecting,
            tiles: TileIndex::new(),
            placed_pieces: PlacedPieceIndex::new(),
            players,
            top_panel_text,
        }
    }
}
