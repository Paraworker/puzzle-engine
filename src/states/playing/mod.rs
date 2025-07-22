use crate::{
    assets::GameAssets,
    rules::GameRules,
    states::{
        GameState,
        playing::{
            board::spawn_board,
            camera::{PlayingCamera, orbit, zoom},
            piece::{finish_dragging, test_spawn_piece},
            session::GameSession,
        },
    },
};
use bevy::prelude::*;

pub mod board;
pub mod camera;
pub mod piece;
pub mod session;

#[derive(Component)]
struct PlayingMarker;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), on_enter)
            .add_systems(
                Update,
                (zoom, orbit, finish_dragging, test_spawn_piece)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), on_exit);
    }
}

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    rules: Res<GameRules>,
) {
    // Board
    spawn_board(&mut commands, &mut meshes, &assets, rules.board_geometry());

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0),
        PlayingMarker,
    ));

    // Camera
    let cam = PlayingCamera::new();
    commands.spawn((Camera3d::default(), cam.transform(), cam, PlayingMarker));
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<PlayingMarker>>) {
    // Delete entities
    for entity in entities {
        commands.entity(entity).despawn();
    }

    // Delete related resources
    commands.remove_resource::<GameSession>();
    commands.remove_resource::<GameRules>();
}
