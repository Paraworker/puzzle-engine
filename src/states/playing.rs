use std::ops::DerefMut;

use crate::{
    assets::GameAssets,
    piece::{Dragged, PieceColor, PieceInfo, PieceModel, Placed},
    position::Pos,
    rules::{GameRules, board::BoardRuleSet},
    session::{GameSession, PlayState},
    states::GameState,
    tile::Tile,
};
use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), on_enter)
            .add_systems(
                Update,
                (zoom, orbit, finish_dragging).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), on_exit);
    }
}

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

    fn zoom(&mut self, delta: f32) {
        const ZOOM_SPEED: f32 = 0.2;
        const MIN_DISTANCE: f32 = 5.0;
        const MAX_DISTANCE: f32 = 40.0;

        self.radius -= delta * ZOOM_SPEED;
        self.radius = self.radius.clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    fn drag(&mut self, delta_x: f32, delta_y: f32) {
        self.azimuth -= delta_x * 0.01;
        self.elevation += delta_y * 0.01;
        self.elevation = self
            .elevation
            .clamp(0.1, std::f32::consts::FRAC_PI_2 - 0.05);
    }
}

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    rules: Res<GameRules>,
) {
    let mut session = GameSession::new();

    // Board
    spawn_board(
        &mut commands,
        &mut meshes,
        &mut session,
        &assets,
        &rules.board,
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

    // Initial pieces
    for piece in rules.initial_layout.layout() {
        spawn_piece(
            &mut commands,
            &assets,
            &rules.board,
            piece.model(),
            piece.color(),
            piece.pos(),
        );
    }

    // Insert the game session to resource
    commands.insert_resource(session);
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

/// A system that zooms the camera when the mouse wheel is scrolled.
fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
    session: Res<GameSession>,
) {
    if let PlayState::Navigating = session.state {
        for ev in scroll_evr.read() {
            for (mut transform, mut camera) in &mut query {
                camera.zoom(ev.y);

                // Update transform
                *transform = camera.transform();
            }
        }
    }
}

/// A system that orbits the camera around the focus point when dragged.
fn orbit(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut camera_query: Query<(&mut Transform, &mut PlayingCamera)>,
    session: Res<GameSession>,
) {
    if let PlayState::Navigating = session.state {
        for drag in drag_events.read() {
            for (mut transform, mut cam) in camera_query.iter_mut() {
                cam.drag(drag.delta.x, drag.delta.y);

                // Update transform
                *transform = cam.transform();
            }
        }
    }
}

/// A system that finishes dragging a piece when the primary button is released.
fn finish_dragging(
    mut released: EventReader<Pointer<Released>>,
    mut commands: Commands,
    mut piece_query: Query<
        (&mut MeshMaterial3d<StandardMaterial>, &PieceInfo, &Dragged),
        Without<Tile>,
    >,
    mut tile_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
    mut session: ResMut<GameSession>,
    assets: Res<GameAssets>,
) {
    if let PlayState::Dragging(entity) = session.state {
        for event in released.read() {
            if event.button == PointerButton::Primary {
                if let Ok((mut piece_material, piece_info, dragged)) = piece_query.get_mut(entity) {
                    // Reset the dragged piece's material to its original color
                    piece_material.0 = assets.materials.piece.get(piece_info.color()).clone();

                    // Restore the color of start tile
                    if let Ok((mut tile_material, tile)) =
                        tile_query.get_mut(session.tiles.get(dragged.start_pos()).unwrap())
                    {
                        tile_material.0 = tile.color().clone();
                    }

                    // Update component
                    commands
                        .entity(entity)
                        .insert(Placed::new(dragged.current_pos()))
                        .remove::<Dragged>();

                    // Finish dragging state
                    session.state = PlayState::Navigating;
                }

                // We only handle the first release event
                break;
            }
        }
    }
}

/// Spawns the board.
fn spawn_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    session: &mut GameSession,
    assets: &GameAssets,
    board: &BoardRuleSet,
) {
    fn on_tile_hovered(
        trigger: Trigger<Pointer<Over>>,
        mut dragged_query: Query<(&mut Transform, &mut Dragged)>,
        mut tile_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
        rules: Res<GameRules>,
        session: Res<GameSession>,
    ) {
        if let PlayState::Dragging(entity) = session.state {
            if let Ok((_, tile)) = tile_query.get_mut(trigger.target()) {
                if let Ok((mut transform, mut dragged)) = dragged_query.get_mut(entity) {
                    // Update translation
                    transform.translation = place_piece(tile.pos(), &rules.board);

                    // Update dragged position
                    dragged.update_pos(tile.pos());
                }
            }
        }
    }

    for col in 0..board.cols() {
        for row in 0..board.rows() {
            // Tile position
            let pos = Pos::new(row, col);

            // Choose color based on position
            let color = if (col + row) % 2 == 0 {
                assets.materials.common.tile_white.clone()
            } else {
                assets.materials.common.tile_black.clone()
            };

            let entity = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        BoardRuleSet::tile_size(),
                        BoardRuleSet::tile_height(),
                        BoardRuleSet::tile_size(),
                    ))),
                    MeshMaterial3d(color.clone()),
                    Transform::from_translation(pos_to_world(
                        pos,
                        board,
                        -BoardRuleSet::tile_height() / 2.0,
                    )),
                    GlobalTransform::default(),
                    Tile::new(pos, color),
                    PlayingMarker,
                ))
                .observe(on_tile_hovered)
                .id();

            // Add to tile index
            session.tiles.add(pos, entity);
        }
    }
}

/// Calculates the world translation for placing a piece to a specific position.
fn place_piece(to: Pos, board: &BoardRuleSet) -> Vec3 {
    pos_to_world(to, board, BoardRuleSet::tile_size() / 4.0)
}

/// Spawns a piece on the board at the specified position with the given model and color.
fn spawn_piece(
    commands: &mut Commands,
    assets: &GameAssets,
    board: &BoardRuleSet,
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
) {
    fn on_piece_pressed(
        trigger: Trigger<Pointer<Pressed>>,
        mut commands: Commands,
        mut piece_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Placed), Without<Tile>>,
        mut tile_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
        mut session: ResMut<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }

        if let PlayState::Navigating = session.state {
            if let Ok((mut piece_material, placed)) = piece_query.get_mut(trigger.target()) {
                // Change the piece material to indicate dragging
                piece_material.0 = assets.materials.common.piece_dragged.clone();

                // Highlight the tile where the drag started
                if let Ok((mut tile_material, _)) =
                    tile_query.get_mut(session.tiles.get(placed.pos()).unwrap())
                {
                    tile_material.0 = assets.materials.common.tile_drag_start.clone();
                }

                // Update component
                commands
                    .entity(trigger.target())
                    .insert(Dragged::new(placed.pos()))
                    .remove::<Placed>();

                // Start dragging state
                session.state = PlayState::Dragging(trigger.target());
            }
        }
    }

    commands
        .spawn((
            Mesh3d(assets.meshes.piece.get(model).clone()),
            MeshMaterial3d(assets.materials.piece.get(color).clone()),
            Transform {
                translation: place_piece(pos, board),
                scale: Vec3::splat(BoardRuleSet::tile_size() * 0.5),
                ..default()
            },
            PieceInfo::new(model, color),
            Placed::new(pos),
        ))
        .observe(on_piece_pressed);
}

/// Converts a logical board position to world space translation.
///
/// (0, 0) is the bottom-left tile on the board.
/// `y` is the vertical translation and should be provided.
fn pos_to_world(pos: Pos, board: &BoardRuleSet, y: f32) -> Vec3 {
    const fn half_len(cols_or_rows: usize) -> f32 {
        (cols_or_rows as f32 - 1.0) * BoardRuleSet::tile_size() / 2.0
    }

    Vec3::new(
        pos.col() as f32 * BoardRuleSet::tile_size() - half_len(board.cols()),
        y,
        half_len(board.rows()) - pos.row() as f32 * BoardRuleSet::tile_size(),
    )
}
