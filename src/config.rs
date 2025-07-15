use crate::{rules::GameRules, utils::load_ron};
use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Config {
    saved_rules: HashMap<String, GameRules>,
}

impl Config {
    const CONFIG_PATH: &'static str = "assets/config.ron";

    pub fn load() -> anyhow::Result<Self> {
        load_ron(Self::CONFIG_PATH)
    }
}
