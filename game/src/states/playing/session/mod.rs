use crate::states::playing::session::{
    piece_index::PlacedPieceIndex, player::Players, tile_index::TileIndex, turn::TurnController,
};
use bevy::prelude::*;
use rule_engine::pos::Pos;

pub mod piece_index;
pub mod player;
pub mod tile_index;
pub mod turn;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub board: Entity,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
    pub players: Players,
    pub turn: TurnController,
    pub last_action: Option<Pos>,
}
