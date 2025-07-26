use crate::{
    assets::GameAssets,
    piece::{DraggedPiece, HighlightedPiece, PieceKind, PlacedPiece},
    rules::{GameRules, board::BoardRuleSet, position::Pos},
    session::{GameSession, pieces::PieceEntities, state::SessionState, tiles::TileEntities},
    states::GameState,
    tile::{DragInitialTile, PlaceableTile, Tile},
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
            &mut session,
            &assets,
            &rules.board,
            PieceKind::new(piece.model(), piece.color()),
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
    if let SessionState::Navigating = session.state {
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
    if let SessionState::Navigating = session.state {
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
    dragged_piece_query: Query<&DraggedPiece>,
    mut highlighted_piece_query: Query<
        &mut Visibility,
        (
            With<HighlightedPiece>,
            Without<DragInitialTile>,
            Without<PlaceableTile>,
        ),
    >,
    mut drag_initial_tile_query: Query<
        &mut Visibility,
        (
            With<DragInitialTile>,
            Without<PlaceableTile>,
            Without<HighlightedPiece>,
        ),
    >,
    mut placeable_tile_query: Query<
        &mut Visibility,
        (
            With<PlaceableTile>,
            Without<HighlightedPiece>,
            Without<DragInitialTile>,
        ),
    >,
    mut session: ResMut<GameSession>,
) {
    let session = session.as_mut();

    if let SessionState::Dragging(entities) = &session.state {
        for event in released.read() {
            if event.button == PointerButton::Primary {
                if let Ok(dragged) = dragged_piece_query.get(entities.base()) {
                    // Unhighlight the dragged piece
                    if let Ok(mut visibility) =
                        highlighted_piece_query.get_mut(entities.highlighted())
                    {
                        *visibility = Visibility::Hidden;
                    }

                    // Unhighlight the drag initial tiles
                    if let Ok(mut visibility) = drag_initial_tile_query.get_mut(
                        session
                            .tiles
                            .get(dragged.initial_pos())
                            .unwrap()
                            .drag_initial(),
                    ) {
                        *visibility = Visibility::Hidden;
                    }

                    // Unhighlight the placeable tiles
                    for pos in dragged.placeable_tiles() {
                        if let Ok(mut visibility) = placeable_tile_query
                            .get_mut(session.tiles.get(pos).unwrap().placeable())
                        {
                            *visibility = Visibility::Hidden;
                        }
                    }

                    // Update component
                    commands
                        .entity(entities.base())
                        .insert(PlacedPiece::new(dragged.kind(), dragged.current_pos()))
                        .remove::<DraggedPiece>();

                    // Add piece entities to the placed piece index at the current position
                    session
                        .placed_pieces
                        .add(dragged.current_pos(), entities.clone());

                    // Finish dragging state
                    session.state = SessionState::Navigating;
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
        mut dragged_query: Query<(&mut Transform, &mut DraggedPiece)>,
        mut tile_query: Query<&Tile>,
        rules: Res<GameRules>,
        session: Res<GameSession>,
    ) {
        if let SessionState::Dragging(entities) = &session.state {
            let Ok(tile) = tile_query.get_mut(trigger.target()) else {
                return;
            };

            let Ok((mut transform, mut dragged)) = dragged_query.get_mut(entities.base()) else {
                return;
            };

            // Attempt to move the piece
            if !dragged.try_move_to(tile.pos()) {
                return;
            }

            // Update translation
            transform.translation = piece_pos_to_world(tile.pos(), &rules.board);
        }
    }

    for col in 0..board.cols() {
        for row in 0..board.rows() {
            // Tile position
            let pos = Pos::new(row, col);

            // Choose color based on position
            let base_color = if (col + row) % 2 == 0 {
                assets.materials.common.tile_white.clone()
            } else {
                assets.materials.common.tile_black.clone()
            };

            // Spawn base tile entity
            let base = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        BoardRuleSet::tile_size(),
                        BoardRuleSet::tile_height(),
                        BoardRuleSet::tile_size(),
                    ))),
                    MeshMaterial3d(base_color),
                    Transform::from_translation(pos_to_world(
                        pos,
                        board,
                        -BoardRuleSet::tile_height() / 2.0,
                    )),
                    GlobalTransform::default(),
                    Tile::new(pos),
                    PlayingMarker,
                ))
                .observe(on_tile_hovered)
                .id();

            // Spawn drag initial entity
            let drag_initial = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        BoardRuleSet::tile_size(),
                        BoardRuleSet::tile_height(),
                        BoardRuleSet::tile_size(),
                    ))),
                    MeshMaterial3d(assets.materials.common.highlight_source.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Visibility::Hidden,
                    DragInitialTile,
                ))
                .id();

            // Spawn placeable entity
            let placeable = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        BoardRuleSet::tile_size(),
                        BoardRuleSet::tile_height(),
                        BoardRuleSet::tile_size(),
                    ))),
                    MeshMaterial3d(assets.materials.common.highlight_placeable.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Visibility::Hidden,
                    PlaceableTile,
                ))
                .id();

            commands
                .entity(base)
                .add_children(&[drag_initial, placeable]);

            // Add to tile index
            session
                .tiles
                .add(pos, TileEntities::new(base, drag_initial, placeable));
        }
    }
}

/// Calculates the world translation for placing a piece to a specific position.
fn piece_pos_to_world(to: Pos, board: &BoardRuleSet) -> Vec3 {
    pos_to_world(to, board, BoardRuleSet::tile_size() / 4.0)
}

/// Spawns a piece on the board at the specified position with the given model and color.
fn spawn_piece(
    commands: &mut Commands,
    session: &mut GameSession,
    assets: &GameAssets,
    board: &BoardRuleSet,
    kind: PieceKind,
    pos: Pos,
) {
    fn on_piece_pressed(
        trigger: Trigger<Pointer<Pressed>>,
        mut commands: Commands,
        placed_piece_query: Query<&PlacedPiece>,
        tile_query: Query<&Tile>,
        mut highlighted_piece_query: Query<
            &mut Visibility,
            (
                With<HighlightedPiece>,
                Without<DragInitialTile>,
                Without<PlaceableTile>,
            ),
        >,
        mut drag_initial_query: Query<
            &mut Visibility,
            (
                With<DragInitialTile>,
                Without<HighlightedPiece>,
                Without<PlaceableTile>,
            ),
        >,
        mut placeable_query: Query<
            &mut Visibility,
            (
                With<PlaceableTile>,
                Without<HighlightedPiece>,
                Without<DragInitialTile>,
            ),
        >,
        mut session: ResMut<GameSession>,
        rules: Res<GameRules>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }

        if let SessionState::Navigating = session.state {
            if let Ok(placed) = placed_piece_query.get(trigger.target()) {
                let dragged = DraggedPiece::new(
                    placed.kind(),
                    placed.pos(),
                    rules.pieces.get(placed.kind().model()).unwrap().movement(),
                    tile_query.iter(),
                )
                .unwrap();

                // Take the piece entities from the placed piece index
                let entities = session.placed_pieces.remove(placed.pos()).unwrap();

                // Highlight the dragging piece
                if let Ok(mut visibility) = highlighted_piece_query.get_mut(entities.highlighted())
                {
                    *visibility = Visibility::Visible;
                }

                // Highlight drag initial tile
                if let Ok(mut visibility) = drag_initial_query
                    .get_mut(session.tiles.get(placed.pos()).unwrap().drag_initial())
                {
                    *visibility = Visibility::Visible;
                }

                // Highlight placeable tiles
                for pos in dragged.placeable_tiles() {
                    if let Ok(mut visibility) =
                        placeable_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                    {
                        *visibility = Visibility::Visible;
                    }
                }

                // Update component
                commands
                    .entity(entities.base())
                    .insert(dragged)
                    .remove::<PlacedPiece>();

                // Start dragging state
                session.state = SessionState::Dragging(entities);
            }
        }
    }

    let mesh = assets.meshes.piece.get(kind.model());

    let base = commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(assets.materials.piece.get(kind.color()).clone()),
            Transform {
                translation: piece_pos_to_world(pos, board),
                scale: Vec3::splat(BoardRuleSet::tile_size() * 0.5),
                ..default()
            },
            GlobalTransform::default(),
            PlacedPiece::new(kind, pos),
        ))
        .observe(on_piece_pressed)
        .id();

    let highlighted = commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(assets.materials.common.highlight_source.clone()),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Hidden,
            HighlightedPiece,
        ))
        .id();

    commands.entity(base).add_child(highlighted);

    // Add to placed piece index
    session
        .placed_pieces
        .add(pos, PieceEntities::new(base, highlighted));
}

/// Converts a logical board position to world space translation.
///
/// (0, 0) is the bottom-left tile on the board.
/// `y` is the vertical translation and should be provided.
fn pos_to_world(pos: Pos, board: &BoardRuleSet, y: f32) -> Vec3 {
    const fn half_len(cols_or_rows: i64) -> f32 {
        (cols_or_rows as f32 - 1.0) * BoardRuleSet::tile_size() / 2.0
    }

    Vec3::new(
        pos.col() as f32 * BoardRuleSet::tile_size() - half_len(board.cols()),
        y,
        half_len(board.rows()) - pos.row() as f32 * BoardRuleSet::tile_size(),
    )
}
