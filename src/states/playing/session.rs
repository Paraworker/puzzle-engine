use crate::states::playing::piece::DraggedPiece;
use bevy::ecs::resource::Resource;

#[derive(Debug)]
pub enum PlayState {
    Navigating,
    Dragging(DraggedPiece),
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
