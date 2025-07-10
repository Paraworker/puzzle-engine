use crate::utils::load_ron;
use bevy::ecs::resource::Resource;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Resource)]
pub struct Config {
    boards: HashMap<String, BoardMeta>,
}

impl Config {
    const CONFIG_PATH: &str = "assets/config.ron";

    pub fn load() -> anyhow::Result<Self> {
        load_ron(Self::CONFIG_PATH)
    }

    pub fn board_meta(&self, name: &str) -> Option<&BoardMeta> {
        self.boards.get(name)
    }
}

#[derive(Debug, Deserialize)]
pub struct BoardMeta {
    display_name: String,
    description: String,
}

impl BoardMeta {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
