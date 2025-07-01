use bevy::{
    asset::Handle, ecs::resource::Resource, pbr::StandardMaterial, scene::Scene,
    state::state::States,
};

pub mod loading;
pub mod menu;
pub mod playing;
pub mod startup;

#[derive(Resource)]
pub struct ActiveScene(Handle<Scene>);

#[derive(Resource)]
pub struct TileNormalMaterial(Handle<StandardMaterial>);

#[derive(Resource)]
pub struct TileHoverMaterial(Handle<StandardMaterial>);

#[derive(Resource)]
pub struct TilePressedMaterial(Handle<StandardMaterial>);

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Startup,
    Menu,
    Loading,
    Playing,
}
