use crate::states::playing::{
    piece::PlacedPiece,
    session::{player::Players, turn::TurnController},
    tile::TileEntities,
};
use bevy::prelude::*;
use rule_engine::pos::Pos;
use std::collections::HashMap;

pub mod player;
pub mod turn;

/// Indexes for tiles.
pub type TileIndex = HashMap<Pos, TileEntities>;

/// Indexes for placed pieces.
pub type PlacedPieceIndex = HashMap<Pos, PlacedPiece>;

#[derive(Debug, Resource)]
pub struct GameSession {
    pub board: Entity,
    pub tiles: TileIndex,
    pub placed_pieces: PlacedPieceIndex,
    pub players: Players,
    pub turn: TurnController,
    pub last_action: Option<Pos>,
}
