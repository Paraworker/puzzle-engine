use crate::{config::BoardName, session::GameSession, states::GameState};
use bevy::{asset::AssetServer, ecs::system::Commands, state::state::NextState};
use ron::de::from_reader;
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader, path::Path};

pub fn load_ron<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(from_reader(BufReader::new(File::open(path)?))?)
}

pub fn new_session(
    commands: &mut Commands,
    asset_server: &AssetServer,
    next_state: &mut NextState<GameState>,
    name: BoardName,
) {
    commands.insert_resource(
        GameSession::new(name, asset_server).expect("Unable to create game session"),
    );

    next_state.set(GameState::Loading);
}
