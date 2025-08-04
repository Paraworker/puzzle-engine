use crate::GameError;
use bevy::ecs::resource::Resource;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    pub rules_path: PathBuf,
}

impl Settings {
    pub fn load<P>(path: P) -> Result<Self, GameError>
    where
        P: AsRef<Path>,
    {
        Ok(from_reader(BufReader::new(File::open(path)?))?)
    }
}
