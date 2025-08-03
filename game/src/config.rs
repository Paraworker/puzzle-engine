use crate::GameError;
use bevy::ecs::resource::Resource;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Config {
    rules_path: PathBuf,
}

impl Config {
    const CONFIG_PATH: &'static str = "game/assets/config.ron";

    pub fn load() -> Result<Self, GameError> {
        Ok(from_reader(BufReader::new(File::open(Self::CONFIG_PATH)?))?)
    }
}
