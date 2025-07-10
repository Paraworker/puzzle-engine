use crate::{
    config::{BoardName, Config},
    tile::TileTopology,
};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::resource::Resource,
    scene::Scene,
};

#[derive(Debug, Resource)]
pub struct GameSession {
    board_name: BoardName,
    scene: Handle<Scene>,
    tile_topology: TileTopology,
}

impl GameSession {
    pub fn new(board_name: BoardName, asset_server: &AssetServer) -> anyhow::Result<Self> {
        let scene_path = Config::board_scene_path(&board_name);
        let topology_path = Config::board_topology_path(&board_name);

        Ok(Self {
            board_name,
            scene: asset_server.load(scene_path),
            tile_topology: TileTopology::load(topology_path)?,
        })
    }

    pub fn board_name(&self) -> &BoardName {
        &self.board_name
    }

    pub fn scene(&self) -> &Handle<Scene> {
        &self.scene
    }

    pub fn tile_topology(&self) -> &TileTopology {
        &self.tile_topology
    }
}
