use crate::{
    assets::GameAssets,
    piece::Piece,
    position::Pos,
    rules::{GameRules, board::BoardRuleSet},
    session::{DragState, GameSession, PlayState},
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
    // Create the game session
    commands.insert_resource(GameSession::new());

    // Board
    spawn_board(&mut commands, &mut meshes, &assets, &rules.board);

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
        spawn_piece(&mut commands, &assets, &rules.board, piece.clone());
    }
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
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
    mut session: ResMut<GameSession>,
    assets: Res<GameAssets>,
) {
    if let PlayState::Dragging(drag) = &mut session.state {
        for event in released.read() {
            if event.button == PointerButton::Primary {
                // Reset the dragged piece's material to its original color
                if let Ok((mut material, piece)) = query.get_mut(drag.piece()) {
                    material.0 = assets.materials.piece.get(piece.color()).clone();
                }

                // Finish dragging state
                session.state = PlayState::Navigating;

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
    assets: &GameAssets,
    board: &BoardRuleSet,
) {
    fn tile_transform(pos: Pos, board: &BoardRuleSet) -> Transform {
        Transform::from_translation(Vec3::new(
            pos.col() as f32 * BoardRuleSet::tile_size() - board.half_width_col(),
            -BoardRuleSet::tile_height() / 2.0,
            pos.row() as f32 * BoardRuleSet::tile_size() - board.half_width_row(),
        ))
    }

    fn on_tile_hovered(
        trigger: Trigger<Pointer<Over>>,
        mut dragged_query: Query<(&mut Transform, &mut Piece)>,
        mut tile_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
        assets: Res<GameAssets>,
        rules: Res<GameRules>,
        session: Res<GameSession>,
    ) {
        match &session.state {
            PlayState::Navigating => {
                if let Ok((mut material, _)) = tile_query.get_mut(trigger.target()) {
                    material.0 = assets.materials.common.tile_hover.clone();
                }
            }
            PlayState::Dragging(drag) => {
                if let Ok((_, tile)) = tile_query.get_mut(trigger.target()) {
                    if let Ok((mut transform, mut piece)) = dragged_query.get_mut(drag.piece()) {
                        // Update translation
                        transform.translation = place_piece(tile.pos(), &rules.board);

                        // Update position
                        piece.set_pos(tile.pos());
                    }
                }
            }
        }
    }

    fn on_tile_out(
        trigger: Trigger<Pointer<Out>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
        session: Res<GameSession>,
    ) {
        if let PlayState::Navigating = session.state {
            if let Ok((mut material, tile)) = query.get_mut(trigger.target()) {
                // Restore the tile's original color
                material.0 = tile.color();
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

            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        BoardRuleSet::tile_size(),
                        BoardRuleSet::tile_height(),
                        BoardRuleSet::tile_size(),
                    ))),
                    MeshMaterial3d(color.clone()),
                    tile_transform(pos, board),
                    GlobalTransform::default(),
                    Tile::new(pos, color),
                    PlayingMarker,
                ))
                .observe(on_tile_hovered)
                .observe(on_tile_out);
        }
    }
}

/// Calculates the world translation for placing a piece to a specific tile position.
fn place_piece(to: Pos, board: &BoardRuleSet) -> Vec3 {
    Vec3::new(
        to.col() as f32 * BoardRuleSet::tile_size() - board.half_width_col(),
        BoardRuleSet::tile_size() / 4.0,
        to.row() as f32 * BoardRuleSet::tile_size() - board.half_width_row(),
    )
}

/// Spawns a piece on the board at the specified position with the given model and color.
fn spawn_piece(commands: &mut Commands, assets: &GameAssets, board: &BoardRuleSet, piece: Piece) {
    fn on_piece_hovered(
        trigger: Trigger<Pointer<Over>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
        session: Res<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if let PlayState::Navigating = session.state {
            if let Ok((mut material, _)) = query.get_mut(trigger.target()) {
                material.0 = assets.materials.common.piece_hover.clone();
            }
        }
    }

    fn on_piece_out(
        trigger: Trigger<Pointer<Out>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
        session: Res<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if let PlayState::Navigating = session.state {
            if let Ok((mut material, piece)) = query.get_mut(trigger.target()) {
                // Restore the piece's original color
                material.0 = assets.materials.piece.get(piece.color()).clone();
            }
        }
    }

    fn on_piece_pressed(
        trigger: Trigger<Pointer<Pressed>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
        mut session: ResMut<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }

        if let PlayState::Navigating = session.state {
            if let Ok((mut material, _)) = query.get_mut(trigger.target()) {
                material.0 = assets.materials.common.piece_dragged.clone();

                // Start dragging state
                session.state = PlayState::Dragging(DragState::new(trigger.target()));
            }
        }
    }

    commands
        .spawn((
            Mesh3d(assets.meshes.piece.get(piece.model()).clone()),
            MeshMaterial3d(assets.materials.piece.get(piece.color()).clone()),
            Transform {
                translation: place_piece(piece.pos(), board),
                scale: Vec3::splat(BoardRuleSet::tile_size() * 0.5),
                ..default()
            },
            piece,
        ))
        .observe(on_piece_hovered)
        .observe(on_piece_out)
        .observe(on_piece_pressed);
}
