use crate::position::Pos;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum PlayState {
    Navigating,
    Dragging(Entity),
}

/// A index of tile entities by their position.
#[derive(Debug)]
pub struct TileIndex(HashMap<Pos, Entity>);

impl TileIndex {
    pub fn new() -> Self {
        TileIndex(HashMap::new())
    }

    pub fn add(&mut self, pos: Pos, entity: Entity) {
        self.0.insert(pos, entity);
    }

    pub fn get(&self, pos: Pos) -> Option<Entity> {
        self.0.get(&pos).cloned()
    }
}

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: PlayState,
    pub tiles: TileIndex,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            state: PlayState::Navigating,
            tiles: TileIndex::new(),
        }
    }
}
