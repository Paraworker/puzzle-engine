use crate::{
    assets::{GameAssets, materials::piece::PieceColor, meshes::piece::PieceModel},
    rules::board::BoardGeometry,
    session::GameSession,
    states::GameState,
    utils::half_width,
};
use bevy::{
    input::{ButtonState, keyboard::KeyboardInput, mouse::MouseWheel},
    prelude::*,
};

#[derive(Component)]
struct PlayingMarker;

#[derive(Component)]
struct PlayingCamera {
    /// Distance from focus point
    radius: f32,
    /// Horizontal rotation angle (in radians)
    azimuth: f32,
    /// Vertical rotation angle (in radians)
    elevation: f32,
}

impl PlayingCamera {
    const FOCUS: Vec3 = Vec3::ZERO;

    fn new() -> Self {
        Self {
            radius: 10.0,
            azimuth: 0.0,
            elevation: std::f32::consts::FRAC_PI_6, // 30°,
        }
    }

    fn transform(&self) -> Transform {
        let x = self.radius * self.elevation.cos() * self.azimuth.sin();
        let y = self.radius * self.elevation.sin();
        let z = self.radius * self.elevation.cos() * self.azimuth.cos();

        Transform::from_translation(Vec3::new(x, y, z)).looking_at(Self::FOCUS, Vec3::Y)
    }
}

#[derive(Debug, Clone)]
pub struct TilePos(usize, usize);

impl TilePos {
    /// Returns the row index of the tile position.
    pub const fn row(&self) -> usize {
        self.0
    }

    /// Returns the column index of the tile position.
    pub const fn col(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone, Component)]
pub struct Tile {
    pos: TilePos,
    color: Handle<StandardMaterial>,
}

impl Tile {
    pub const fn pos(&self) -> &TilePos {
        &self.pos
    }

    pub fn color(&self) -> Handle<StandardMaterial> {
        self.color.clone()
    }
}

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), on_enter)
            .add_systems(
                Update,
                (zoom, orbit_camera, test_spawn_piece).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), on_exit);
    }
}

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    session: Res<GameSession>,
) {
    // Board
    spawn_board(
        &mut commands,
        &mut meshes,
        &assets,
        session.rules().board_geometry(),
    );

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
}

/// A system that zooms the camera when the mouse wheel is scrolled.
fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
) {
    const ZOOM_SPEED: f32 = 0.2;
    const MIN_DISTANCE: f32 = 5.0;
    const MAX_DISTANCE: f32 = 40.0;

    for ev in scroll_evr.read() {
        for (mut transform, mut camera) in &mut query {
            // Update radius
            camera.radius -= ev.y * ZOOM_SPEED;
            camera.radius = camera.radius.clamp(MIN_DISTANCE, MAX_DISTANCE);

            // Update transform
            *transform = camera.transform();
        }
    }
}

// A system that orbits the camera around the board center.
fn orbit_camera(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
) {
    for drag in drag_events.read() {
        for (mut transform, mut cam) in &mut query {
            cam.azimuth -= drag.delta.x * 0.01;
            cam.elevation += drag.delta.y * 0.01;
            cam.elevation = cam.elevation.clamp(0.1, std::f32::consts::FRAC_PI_2 - 0.05);

            let x = cam.radius * cam.elevation.cos() * cam.azimuth.sin();
            let y = cam.radius * cam.elevation.sin();
            let z = cam.radius * cam.elevation.cos() * cam.azimuth.cos();

            transform.translation = PlayingCamera::FOCUS + Vec3::new(x, y, z);
            transform.look_at(PlayingCamera::FOCUS, Vec3::Y);
        }
    }
}

fn on_tile_hovered(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    if let Ok(mut material) = query.get_mut(trigger.target()) {
        material.0 = assets.materials().common().tile_hover.clone();
    }
}

fn on_tile_out(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
) {
    if let Ok((mut material, tile)) = query.get_mut(trigger.target()) {
        // Restore the tile's original color
        material.0 = tile.color();
    }
}

fn on_tile_pressed(
    trigger: Trigger<Pointer<Pressed>>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    if let Ok(mut material) = query.get_mut(trigger.target()) {
        material.0 = assets.materials().common().tile_pressed.clone();
    }
}

fn on_tile_released(
    trigger: Trigger<Pointer<Released>>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
) {
    if let Ok((mut material, tile)) = query.get_mut(trigger.target()) {
        // Restore the tile's original color
        material.0 = tile.color();
    }
}

fn spawn_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    board: &BoardGeometry,
) {
    fn tile_transform(pos: &TilePos, board: &BoardGeometry) -> Transform {
        let half_width_col = half_width(board.cols(), board.tile_size());
        let half_width_row = half_width(board.rows(), board.tile_size());

        Transform::from_translation(Vec3::new(
            pos.col() as f32 * board.tile_size() - half_width_col,
            -board.height() / 2.0,
            pos.row() as f32 * board.tile_size() - half_width_row,
        ))
    }

    for col in 0..board.cols() {
        for row in 0..board.rows() {
            // Tile position
            let pos = TilePos(row, col);

            // Choose color based on position
            let color = if (col + row) % 2 == 0 {
                assets.materials().common().tile_white.clone()
            } else {
                assets.materials().common().tile_black.clone()
            };

            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        board.tile_size(),
                        board.height(),
                        board.tile_size(),
                    ))),
                    MeshMaterial3d(color.clone()),
                    tile_transform(&pos, board),
                    GlobalTransform::default(),
                    Tile { pos, color },
                    PlayingMarker,
                ))
                .observe(on_tile_hovered)
                .observe(on_tile_out)
                .observe(on_tile_pressed)
                .observe(on_tile_released);
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    assets: &GameAssets,
    board: &BoardGeometry,
    model: PieceModel,
    color: PieceColor,
    pos: TilePos,
) {
    fn piece_transform(pos: TilePos, board: &BoardGeometry) -> Transform {
        let half_width_col = half_width(board.cols(), board.tile_size());
        let half_width_row = half_width(board.rows(), board.tile_size());

        Transform {
            translation: Vec3::new(
                pos.col() as f32 * board.tile_size() - half_width_col,
                0.25,
                pos.row() as f32 * board.tile_size() - half_width_row,
            ),
            scale: Vec3::splat(board.tile_size() * 0.5),
            ..default()
        }
    }

    let model = assets.meshes().piece().get(model).clone();
    let color = assets.materials().piece().get(color).clone();

    commands.spawn((
        Mesh3d(model),
        MeshMaterial3d(color),
        piece_transform(pos, board),
    ));
}

pub fn test_spawn_piece(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    assets: Res<GameAssets>,
    session: Res<GameSession>,
) {
    for ev in key_evr.read() {
        if ev.key_code == KeyCode::KeyP && ev.state == ButtonState::Pressed {
            spawn_piece(
                &mut commands,
                &assets,
                session.rules().board_geometry(),
                PieceModel::Cube,
                PieceColor::Black,
                TilePos(0, 0),
            );

            spawn_piece(
                &mut commands,
                &assets,
                session.rules().board_geometry(),
                PieceModel::Sphere,
                PieceColor::White,
                TilePos(1, 1),
            );

            spawn_piece(
                &mut commands,
                &assets,
                session.rules().board_geometry(),
                PieceModel::Cylinder,
                PieceColor::Black,
                TilePos(2, 2),
            );
        }
    }
}
