use crate::utils::load_ron;
use bevy::ecs::{component::Component, resource::Resource};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Resource)]
pub struct Config {
    boards: HashMap<BoardName, BoardMeta>,
}

impl Config {
    const CONFIG_PATH: &'static str = "assets/config.ron";

    pub fn load() -> anyhow::Result<Self> {
        load_ron(Self::CONFIG_PATH)
    }

    pub fn board(&self, name: &BoardName) -> Option<&BoardMeta> {
        self.boards.get(name)
    }

    pub fn boards(&self) -> impl Iterator<Item = (&BoardName, &BoardMeta)> {
        self.boards.iter()
    }

    pub fn board_scene_path(name: &BoardName) -> String {
        format!("boards/{}/scene.glb#Scene0", name.0)
    }

    pub fn board_topology_path(name: &BoardName) -> String {
        format!("assets/boards/{}/topology.ron", name.0)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Component)]
pub struct BoardName(String);

impl BoardName {
    pub fn name(&self) -> &str {
        &self.0
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
