use bevy::prelude::*;

#[derive(Debug)]
pub enum SessionState {
    Navigating,
    Dragging(Entity),
}
