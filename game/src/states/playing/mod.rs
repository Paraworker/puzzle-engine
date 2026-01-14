use crate::{
    assets::GameAssets,
    states::{
        AppState,
        game_setup::LoadedRules,
        playing::{
            board::spawn_board,
            camera::PlayingCamera,
            phases::GamePhasePlugin,
            piece::place_new_piece,
            session::{GameSession, PlacedPieceIndex, player::Players, turn::TurnController},
            ui::{TopPanelText, bottom_panel, top_panel},
        },
    },
};
use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_egui::{EguiGlobalSettings, EguiPrimaryContextPass, PrimaryEguiContext};

pub mod board;
pub mod camera;
pub mod phases;
pub mod piece;
pub mod session;
pub mod tile;
pub mod ui;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamePhasePlugin)
            .add_message::<TileEnter>()
            .add_message::<TileOut>()
            .add_message::<TileRelease>()
            .add_message::<PiecePress>()
            .add_systems(OnEnter(AppState::Playing), on_enter)
            .add_systems(OnExit(AppState::Playing), on_exit)
            .add_systems(
                EguiPrimaryContextPass,
                (top_panel, bottom_panel).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Message)]
pub struct TileEnter(pub Entity);

#[derive(Message)]
pub struct TileOut(pub Entity);

#[derive(Message)]
pub struct TileRelease(pub Entity, pub PointerButton);

#[derive(Message)]
pub struct PiecePress(pub Entity, pub PointerButton);

#[derive(Component)]
struct PlayingMarker;

fn on_enter(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    rules: Res<LoadedRules>,
) {
    // Disable the automatic creation of a primary context to set it up manually for the camera we need.
    egui_global_settings.auto_create_primary_context = false;

    // Board
    let (board, tiles) = spawn_board(&mut commands, &mut meshes, &assets, &rules);

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

    // World camera
    let cam = PlayingCamera::new();
    commands.spawn((Camera3d::default(), cam.transform(), cam, PlayingMarker));

    // Egui camera
    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        RenderLayers::none(),
        Camera {
            order: 1,
            ..default()
        },
        PlayingMarker,
    ));

    let mut players = Players::new(&rules);
    let mut placed_pieces = PlacedPieceIndex::new();

    // Initial pieces
    for piece in rules.initial_pieces() {
        place_new_piece(
            &mut commands,
            &assets,
            &rules,
            board,
            &mut players,
            &mut placed_pieces,
            piece.model(),
            piece.color(),
            piece.pos(),
        )
        .unwrap();
    }

    // Create game session
    let session = GameSession {
        board,
        tiles,
        placed_pieces,
        players,
        turn: TurnController::new(),
        last_action: None,
    };

    // Insert resources
    commands.insert_resource(TopPanelText(Default::default()));
    commands.insert_resource(session);
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<PlayingMarker>>) {
    // Delete entities
    for entity in entities {
        commands.entity(entity).despawn();
    }

    // Delete related resources
    commands.remove_resource::<TopPanelText>();
    commands.remove_resource::<GameSession>();
    commands.remove_resource::<LoadedRules>();
}
