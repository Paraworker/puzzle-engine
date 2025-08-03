use crate::{GameError, rules::GameRules};
use bevy::ecs::resource::Resource;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Config {
    saved_rules: HashMap<String, GameRules>,
}

impl Config {
    const CONFIG_PATH: &'static str = "assets/config.ron";

    pub fn load() -> Result<Self, GameError> {
        Ok(from_reader(BufReader::new(File::open(Self::CONFIG_PATH)?))?)
    }
}
