use bevy::prelude::*;

#[derive(Debug)]
pub enum PlayState {
    Navigating,
    Dragging(DragState),
}

#[derive(Debug)]
pub struct DragState {
    piece: Entity,
}

impl DragState {
    /// Creates a new drag state.
    pub fn new(piece: Entity) -> Self {
        Self { piece }
    }

    /// Returns the entity of the piece being dragged.
    pub fn piece(&self) -> Entity {
        self.piece
    }
}

#[derive(Debug, Resource)]
pub struct GameSession {
    pub state: PlayState,
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            state: PlayState::Navigating,
        }
    }
}
