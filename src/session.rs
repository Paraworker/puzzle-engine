use crate::{rules::GameRules, states::playing::Tile};
use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct GameSession {
    rules: GameRules,
    focused_tile: Option<Tile>,
}

impl GameSession {
    pub fn new(rules: GameRules) -> Self {
        Self {
            rules,
            focused_tile: None,
        }
    }

    pub fn rules(&self) -> &GameRules {
        &self.rules
    }
}
