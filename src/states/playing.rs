use crate::{
    assets::GameAssets,
    piece::{HighlightedPiece, MovingPiece, PieceKind, PlacedPiece, PlacingPiece},
    rules::{
        GameRules,
        board::BoardRuleSet,
        piece::{PieceColor, PieceModel, PieceRuleSet},
        position::Pos,
    },
    session::{
        GameSession,
        piece_index::{PieceEntities, PlacedPieceIndex},
        player::Players,
        state::SessionState,
        tile_index::{TileEntities, TileIndex},
    },
    states::GameState,
    tile::{PlaceableTile, SourceOrTargetTile, Tile},
};
use bevy::{input::mouse::MouseWheel, prelude::*, render::view::RenderLayers};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPrimaryContextPass, PrimaryEguiContext,
    egui::{self, Stroke},
};

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), on_enter)
            .add_systems(
                Update,
                (
                    on_mouse_wheel,
                    on_pointer_drag,
                    on_button_pressed,
                    on_button_released,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), on_exit)
            .add_systems(
                EguiPrimaryContextPass,
                (top_panel, stock_panel).run_if(in_state(GameState::Playing)),
            );
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
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    rules: Res<GameRules>,
) {
    // Disable the automatic creation of a primary context to set it up manually for the camera we need.
    egui_global_settings.auto_create_primary_context = false;

    // Create game session
    let mut session = GameSession::new(&rules);

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

    // Initial pieces
    for piece in rules.initial_layout.layout() {
        spawn_placed_piece(
            &mut commands,
            &mut session.placed_pieces,
            &mut session.players,
            &assets,
            &rules.board,
            PieceKind::new(piece.model(), piece.color()),
            piece.pos(),
        );
    }

    // Insert resources
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

/// A system that triggered on the mouse wheel event.
fn on_mouse_wheel(
    mut scroll_evr: EventReader<MouseWheel>,
    mut egui: EguiContexts,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
    session: Res<GameSession>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    if let SessionState::Selecting = session.state {
        for ev in scroll_evr.read() {
            for (mut transform, mut camera) in &mut query {
                camera.zoom(ev.y);

                // Update transform
                *transform = camera.transform();
            }
        }
    }
}

/// A system that triggered when the pointer is dragged.
fn on_pointer_drag(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut egui: EguiContexts,
    mut camera_query: Query<(&mut Transform, &mut PlayingCamera)>,
    session: Res<GameSession>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    if let SessionState::Selecting = session.state {
        for drag in drag_events.read() {
            for (mut transform, mut cam) in camera_query.iter_mut() {
                cam.drag(drag.delta.x, drag.delta.y);

                // Update transform
                *transform = cam.transform();
            }
        }
    }
}

/// A system that triggered when the primary button is released.
fn on_button_pressed(
    mut pressed: EventReader<Pointer<Pressed>>,
    mut egui: EguiContexts,
    mut commands: Commands,
    mut source_or_target_tile_query: Query<
        &mut Visibility,
        (With<SourceOrTargetTile>, Without<PlaceableTile>),
    >,
    mut placeable_tile_query: Query<
        &mut Visibility,
        (With<PlaceableTile>, Without<SourceOrTargetTile>),
    >,
    assets: Res<GameAssets>,
    rules: Res<GameRules>,
    mut session: ResMut<GameSession>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    if let SessionState::Placing(placing) = &mut session.state {
        for event in pressed.read() {
            if event.button == PointerButton::Primary {
                if let Some(to_place) = placing.to_place_pos() {
                    // If the to place position is already occupied, remove the existing piece (i.e. capture it)
                    despawn_placed_piece(&mut commands, &mut session.placed_pieces, to_place);

                    // Spawn the placed piece at the target position
                    spawn_placed_piece(
                        &mut commands,
                        &mut session.placed_pieces,
                        &mut session.players,
                        &assets,
                        &rules.board,
                        placing.kind(),
                        to_place,
                    );

                    // Unhighlight the to place tile
                    if let Ok(mut visibility) = source_or_target_tile_query
                        .get_mut(session.tiles.get(to_place).unwrap().source_or_target())
                    {
                        *visibility = Visibility::Hidden;
                    }

                    // Switch to the next player
                    // We only switch players if the piece was moved
                    session.players.next();

                    // Update the top panel text to reflect the current player's turn
                    session.top_panel_text.set_turn(session.players.current().0);
                };

                // Unhighlight placeable tiles
                for pos in placing.placeable_tiles() {
                    if let Ok(mut visibility) =
                        placeable_tile_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                    {
                        *visibility = Visibility::Hidden;
                    }
                }

                // Finish placing state
                session.state = SessionState::Selecting;

                // We only handle the first release event
                break;
            }
        }
    }
}

/// A system that triggered when the primary button is released.
fn on_button_released(
    mut released: EventReader<Pointer<Released>>,
    mut egui: EguiContexts,
    mut commands: Commands,
    moving_piece_query: Query<&MovingPiece>,
    mut highlighted_piece_query: Query<
        &mut Visibility,
        (
            With<HighlightedPiece>,
            Without<SourceOrTargetTile>,
            Without<PlaceableTile>,
        ),
    >,
    mut source_or_target_tile_query: Query<
        &mut Visibility,
        (
            With<SourceOrTargetTile>,
            Without<PlaceableTile>,
            Without<HighlightedPiece>,
        ),
    >,
    mut placeable_tile_query: Query<
        &mut Visibility,
        (
            With<PlaceableTile>,
            Without<HighlightedPiece>,
            Without<SourceOrTargetTile>,
        ),
    >,
    mut session: ResMut<GameSession>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    if let SessionState::Moving(entities) = &session.state {
        for event in released.read() {
            if event.button == PointerButton::Primary {
                if let Ok(moving) = moving_piece_query.get(entities.base()) {
                    if moving.moved() {
                        // If the target position is already occupied, remove the existing piece (i.e. capture it)
                        despawn_placed_piece(
                            &mut commands,
                            &mut session.placed_pieces,
                            moving.current_pos(),
                        );

                        // Switch to the next player
                        // We only switch players if the piece was moved
                        session.players.next();

                        // Update the top panel text to reflect the current player's turn
                        session.top_panel_text.set_turn(session.players.current().0);
                    }

                    // Unhighlight the moving piece
                    if let Ok(mut visibility) =
                        highlighted_piece_query.get_mut(entities.highlighted())
                    {
                        *visibility = Visibility::Hidden;
                    }

                    // Unhighlight the move initial tile
                    if let Ok(mut visibility) = source_or_target_tile_query.get_mut(
                        session
                            .tiles
                            .get(moving.initial_pos())
                            .unwrap()
                            .source_or_target(),
                    ) {
                        *visibility = Visibility::Hidden;
                    }

                    // Unhighlight placeable tiles
                    for pos in moving.placeable_tiles() {
                        if let Ok(mut visibility) = placeable_tile_query
                            .get_mut(session.tiles.get(pos).unwrap().placeable())
                        {
                            *visibility = Visibility::Hidden;
                        }
                    }

                    // Update component
                    commands
                        .entity(entities.base())
                        .insert(PlacedPiece::new(moving.kind(), moving.current_pos()))
                        .remove::<MovingPiece>();

                    // Add piece entities to the placed piece index at the current position
                    session
                        .placed_pieces
                        .add(moving.current_pos(), entities.clone());

                    // Finish moving state
                    session.state = SessionState::Selecting;
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
        mut moving_query: Query<(&mut Transform, &mut MovingPiece)>,
        mut tile_query: Query<&Tile>,
        mut source_or_target_query: Query<
            &mut Visibility,
            (With<SourceOrTargetTile>, Without<PlaceableTile>),
        >,
        mut placeable_tile_query: Query<&mut Visibility, With<PlaceableTile>>,
        rules: Res<GameRules>,
        mut session: ResMut<GameSession>,
    ) {
        match &mut session.state {
            SessionState::Moving(entities) => {
                let Ok(tile) = tile_query.get_mut(trigger.target()) else {
                    return;
                };

                let Ok((mut transform, mut moving)) = moving_query.get_mut(entities.base()) else {
                    return;
                };

                // Attempt to move the piece
                if !moving.try_move_to(tile.pos()) {
                    return;
                }

                // Update translation
                transform.translation = piece_pos_to_world(tile.pos(), &rules.board);
            }
            SessionState::Placing(placing) => {
                let Ok(tile) = tile_query.get_mut(trigger.target()) else {
                    return;
                };

                // Attempt to place the piece
                if !placing.try_place_at(tile.pos()) {
                    return;
                }

                let entities = session.tiles.get(tile.pos()).unwrap();

                // Unhighlight placable
                if let Ok(mut visibility) = placeable_tile_query.get_mut(entities.placeable()) {
                    *visibility = Visibility::Hidden;
                }

                // Highlight to place
                if let Ok(mut visibility) =
                    source_or_target_query.get_mut(entities.source_or_target())
                {
                    *visibility = Visibility::Visible;
                }
            }
            _ => {}
        }
    }

    fn on_tile_out(
        _trigger: Trigger<Pointer<Out>>,
        mut source_or_target_query: Query<
            &mut Visibility,
            (With<SourceOrTargetTile>, Without<PlaceableTile>),
        >,
        mut placeable_tile_query: Query<&mut Visibility, With<PlaceableTile>>,
        mut session: ResMut<GameSession>,
    ) {
        let session = session.as_mut();

        if let SessionState::Placing(placing) = &mut session.state {
            if let Some(to_place) = placing.to_place_pos() {
                let entities = session.tiles.get(to_place).unwrap();

                // Highlight placable
                if let Ok(mut visibility) = placeable_tile_query.get_mut(entities.placeable()) {
                    *visibility = Visibility::Visible;
                }

                // Unhighlight to place
                if let Ok(mut visibility) =
                    source_or_target_query.get_mut(entities.source_or_target())
                {
                    *visibility = Visibility::Hidden;
                }

                placing.clear_to_place();
            }
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
                .observe(on_tile_out)
                .id();

            // Spawn source or target entity
            let source_or_target = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        BoardRuleSet::tile_size(),
                        BoardRuleSet::tile_height(),
                        BoardRuleSet::tile_size(),
                    ))),
                    MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Visibility::Hidden,
                    SourceOrTargetTile,
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
                .add_children(&[source_or_target, placeable]);

            // Add to tile index
            session
                .tiles
                .add(pos, TileEntities::new(base, source_or_target, placeable));
        }
    }
}

/// Calculates the world translation for placing a piece to a specific position.
fn piece_pos_to_world(to: Pos, board: &BoardRuleSet) -> Vec3 {
    pos_to_world(to, board, BoardRuleSet::tile_size() / 4.0)
}

/// Spawns a piece on the board at the specified position with the given model and color.
fn spawn_placed_piece(
    commands: &mut Commands,
    placed_pieces: &mut PlacedPieceIndex,
    players: &mut Players,
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
                Without<SourceOrTargetTile>,
                Without<PlaceableTile>,
            ),
        >,
        mut source_or_target_query: Query<
            &mut Visibility,
            (
                With<SourceOrTargetTile>,
                Without<HighlightedPiece>,
                Without<PlaceableTile>,
            ),
        >,
        mut placeable_query: Query<
            &mut Visibility,
            (
                With<PlaceableTile>,
                Without<HighlightedPiece>,
                Without<SourceOrTargetTile>,
            ),
        >,
        mut session: ResMut<GameSession>,
        rules: Res<GameRules>,
    ) {
        if let SessionState::Selecting = session.state {
            // Skip if the pointer event is not primary click
            if trigger.button != PointerButton::Primary {
                return;
            }

            // Try to fetch the selected placed piece
            let Ok(placed) = placed_piece_query.get(trigger.target()) else {
                return;
            };

            // If the piece color does not match the current player's color, do nothing
            if session.players.current().0 != placed.kind().color() {
                return;
            }

            // Try to create a move context from the selected piece
            let Ok(moving) = MovingPiece::new(
                placed.kind(),
                placed.pos(),
                rules.pieces.get(placed.kind().model()).movement(),
                tile_query.iter(),
            ) else {
                return;
            };

            // Take the piece entities from the placed piece index
            let Some(entities) = session.placed_pieces.remove(placed.pos()) else {
                return;
            };

            // Highlight visual elements (non-fatal)
            {
                // Highlight the moving piece
                if let Ok(mut visibility) = highlighted_piece_query.get_mut(entities.highlighted())
                {
                    *visibility = Visibility::Visible;
                }

                // Highlight move initial tile
                if let Ok(mut visibility) = source_or_target_query
                    .get_mut(session.tiles.get(placed.pos()).unwrap().source_or_target())
                {
                    *visibility = Visibility::Visible;
                }

                // Highlight placeable tiles
                for pos in moving.placeable_tiles() {
                    if let Ok(mut visibility) =
                        placeable_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                    {
                        *visibility = Visibility::Visible;
                    }
                }
            }

            // Apply component state change
            commands
                .entity(entities.base())
                .insert(moving)
                .remove::<PlacedPiece>();

            // Enter moving state
            session.state = SessionState::Moving(entities);
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
            MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Hidden,
            HighlightedPiece,
        ))
        .id();

    commands.entity(base).add_child(highlighted);

    // Decrease the piece stock
    players
        .get_mut(kind.color())
        .decrease_stock_of(kind.model())
        .expect("Failed to decrease piece stock");

    // Add to placed piece index
    placed_pieces.add(pos, PieceEntities::new(base, highlighted));
}

/// Despawns a piece at the specified position.
fn despawn_placed_piece(commands: &mut Commands, placed_pieces: &mut PlacedPieceIndex, pos: Pos) {
    if let Some(entities) = placed_pieces.remove(pos) {
        commands.entity(entities.base()).despawn();
    }
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

fn top_panel(mut egui: EguiContexts, session: Res<GameSession>) {
    egui::TopBottomPanel::top("top_panel")
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_premultiplied(30, 30, 30, 192))
                .stroke(Stroke::NONE)
                .inner_margin(egui::Margin::symmetric(0, 10)),
        )
        .show(egui.ctx_mut().unwrap(), |ui_ctx| {
            ui_ctx.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui_ctx| {
                    ui_ctx.label(
                        egui::RichText::new(session.top_panel_text.as_ref())
                            .strong()
                            .size(24.0),
                    )
                },
            );
        });
}

fn stock_panel(
    mut egui: EguiContexts,
    tile_query: Query<&Tile>,
    mut placeable_query: Query<&mut Visibility, With<PlaceableTile>>,
    rules: Res<GameRules>,
    mut session: ResMut<GameSession>,
) {
    fn enter_placing_state(
        tile_query: &Query<&Tile>,
        placeable_query: &mut Query<&mut Visibility, With<PlaceableTile>>,
        session_state: &mut SessionState,
        tile_index: &TileIndex,
        rules: &PieceRuleSet,
        model: PieceModel,
        color: PieceColor,
    ) {
        let placing = PlacingPiece::new(
            PieceKind::new(model, color),
            rules.get(model).placement(),
            tile_query.iter(),
        )
        .unwrap();

        // Highlight placeable tiles
        for pos in placing.placeable_tiles() {
            if let Ok(mut visibility) =
                placeable_query.get_mut(tile_index.get(pos).unwrap().placeable())
            {
                *visibility = Visibility::Visible;
            }
        }

        // Enter placing state
        *session_state = SessionState::Placing(placing);
    }

    let session = session.as_mut();

    egui::TopBottomPanel::bottom("stock_panel")
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .inner_margin(egui::Margin::symmetric(10, 8)),
        )
        .show(egui.ctx_mut().unwrap(), |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Piece Stock").size(18.0).strong());

                    let (piece_color, player) = session.players.current();

                    for (model, count) in player.piece_stocks() {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{model}: {count}"))
                                .size(18.0)
                                .strong(),
                        );

                        // Enable the button if the session is in selecting state and the count is not depleted
                        let enabled = matches!(session.state, SessionState::Selecting)
                            && !count.is_depleted();

                        if ui.add_enabled(enabled, button).clicked() {
                            enter_placing_state(
                                &tile_query,
                                &mut placeable_query,
                                &mut session.state,
                                &session.tiles,
                                &rules.pieces,
                                model.clone(),
                                piece_color,
                            );
                        }
                    }
                });
            });
        });
}
