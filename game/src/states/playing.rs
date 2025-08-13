use crate::{
    GameError,
    assets::GameAssets,
    expr_contexts::{game_over::GameOverContext, win_or_lose::WinOrLoseContext},
    piece::{MovingPiece, PlacedPiece, PlacingPiece},
    session::{
        GameSession,
        piece_index::{Entry, PieceEntities, PlacedPieceIndex},
        player::Players,
        state::SessionState,
        tile_index::{TileEntities, TileIndex},
        turn::TurnController,
    },
    states::{GameState, game_setup::LoadedRules},
    tile::Tile,
};
use bevy::{input::mouse::MouseWheel, prelude::*, render::view::RenderLayers};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPrimaryContextPass, PrimaryEguiContext,
    egui::{self, Stroke},
};
use rule_engine::{
    board::BoardRuleSet,
    piece::{PieceColor, PieceModel},
    player::PlayerState,
    position::Pos,
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

#[derive(Resource)]
struct TopPanelText(String);

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
    rules: Res<LoadedRules>,
) {
    // Disable the automatic creation of a primary context to set it up manually for the camera we need.
    egui_global_settings.auto_create_primary_context = false;

    // Board
    let (board, tiles) = spawn_board(&mut commands, &mut meshes, &assets, &rules.board);

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

    let mut players = Players::new(&rules.players, &rules.pieces);
    let mut placed_pieces = PlacedPieceIndex::new();

    // Initial pieces
    for piece in rules.initial_layout.pieces() {
        spawn_placed_piece(
            &mut commands,
            &assets,
            &rules.board,
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
        state: SessionState::Selecting,
        board,
        tiles,
        placed_pieces,
        players,
        turn: TurnController::new(),
        last_action: None,
    };

    // Insert resources
    commands.insert_resource(TopPanelText(session.turn.turn_message(&session.players)));
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

    match session.state {
        SessionState::Selecting | SessionState::Reviewing => {
            for ev in scroll_evr.read() {
                for (mut transform, mut camera) in &mut query {
                    camera.zoom(ev.y);

                    // Update transform
                    *transform = camera.transform();
                }
            }
        }
        _ => {}
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

    match session.state {
        SessionState::Selecting | SessionState::Reviewing => {
            for drag in drag_events.read() {
                for (mut transform, mut cam) in camera_query.iter_mut() {
                    cam.drag(drag.delta.x, drag.delta.y);

                    // Update transform
                    *transform = cam.transform();
                }
            }
        }
        _ => {}
    }
}

/// A system that triggered when the primary button is pressed.
fn on_button_pressed(
    mut pressed: EventReader<Pointer<Pressed>>,
    mut egui: EguiContexts,
    mut commands: Commands,
    mut visibility_query: Query<&mut Visibility>,
    placed_piece_query: Query<&PlacedPiece>,
    assets: Res<GameAssets>,
    rules: Res<LoadedRules>,
    mut session: ResMut<GameSession>,
    mut top_panel_text: ResMut<TopPanelText>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    if let SessionState::Placing(placing) = &mut session.state {
        for event in pressed.read() {
            if event.button == PointerButton::Primary {
                // Unhighlight placeable tiles
                for pos in placing.placeable_tiles() {
                    if let Ok(mut visibility) =
                        visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                    {
                        *visibility = Visibility::Hidden;
                    }
                }

                if let Some(to_place) = placing.to_place_pos() {
                    // If the to place position is already occupied, remove the existing piece (i.e. capture it)
                    despawn_placed_piece(&mut commands, &mut session.placed_pieces, to_place);

                    // Spawn the placed piece at the target position
                    spawn_placed_piece(
                        &mut commands,
                        &assets,
                        &rules.board,
                        session.board,
                        &mut session.players,
                        &mut session.placed_pieces,
                        placing.model(),
                        placing.color(),
                        to_place,
                    )
                    .unwrap();

                    // Unhighlight the to place tile
                    if let Ok(mut visibility) = visibility_query
                        .get_mut(session.tiles.get(to_place).unwrap().source_or_target())
                    {
                        *visibility = Visibility::Hidden;
                    }

                    // Update last action position
                    session.last_action = Some(to_place);

                    // Finish this turn
                    finish_turn(&rules, session, placed_piece_query, &mut top_panel_text);
                } else {
                    // Placement cancelled.
                    session.state = SessionState::Selecting;
                }

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
    placed_piece_query: Query<&PlacedPiece>,
    moving_piece_query: Query<&MovingPiece>,
    mut visibility_query: Query<&mut Visibility>,
    rules: Res<LoadedRules>,
    mut session: ResMut<GameSession>,
    mut top_panel_text: ResMut<TopPanelText>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    if let SessionState::Moving(entities) = &session.state {
        for event in released.read() {
            if event.button == PointerButton::Primary {
                if let Ok(moving) = moving_piece_query.get(entities.root()) {
                    // Unhighlight the moving piece
                    if let Ok(mut visibility) = visibility_query.get_mut(entities.highlight()) {
                        *visibility = Visibility::Hidden;
                    }

                    // Unhighlight the move initial tile
                    if let Ok(mut visibility) = visibility_query.get_mut(
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
                        if let Ok(mut visibility) =
                            visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                        {
                            *visibility = Visibility::Hidden;
                        }
                    }

                    // If the target position is already occupied, remove the existing piece (i.e. capture it)
                    despawn_placed_piece(
                        &mut commands,
                        &mut session.placed_pieces,
                        moving.current_pos(),
                    );

                    // Update component
                    commands
                        .entity(entities.root())
                        .insert(PlacedPiece::new(
                            moving.model(),
                            moving.color(),
                            moving.current_pos(),
                        ))
                        .remove::<MovingPiece>();

                    // Add piece entities to the placed piece index at the current position
                    session
                        .placed_pieces
                        .add(moving.current_pos(), entities.clone());

                    if moving.moved() {
                        // Update last action position
                        session.last_action = Some(moving.current_pos());

                        // Finish this turn
                        finish_turn(&rules, session, placed_piece_query, &mut top_panel_text);
                    } else {
                        // Movement cancelled.
                        session.state = SessionState::Selecting;
                    }
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
    assets: &GameAssets,
    board: &BoardRuleSet,
) -> (Entity, TileIndex) {
    fn on_tile_hovered(
        trigger: Trigger<Pointer<Over>>,
        mut moving_query: Query<(&mut Transform, &mut MovingPiece)>,
        mut tile_query: Query<&Tile>,
        mut visibility_query: Query<&mut Visibility>,
        rules: Res<LoadedRules>,
        mut session: ResMut<GameSession>,
    ) {
        match &mut session.state {
            SessionState::Moving(entities) => {
                let Ok(tile) = tile_query.get_mut(trigger.target()) else {
                    return;
                };

                let Ok((mut transform, mut moving)) = moving_query.get_mut(entities.root()) else {
                    return;
                };

                // Attempt to move the piece
                if !moving.try_move_to(tile.pos()) {
                    return;
                }

                // Update transform
                *transform = pos_to_world(tile.pos(), &rules.board);
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
                if let Ok(mut visibility) = visibility_query.get_mut(entities.placeable()) {
                    *visibility = Visibility::Hidden;
                }

                // Highlight to place
                if let Ok(mut visibility) = visibility_query.get_mut(entities.source_or_target()) {
                    *visibility = Visibility::Visible;
                }
            }
            _ => {}
        }
    }

    fn on_tile_out(
        _trigger: Trigger<Pointer<Out>>,
        mut visibility_query: Query<&mut Visibility>,
        mut session: ResMut<GameSession>,
    ) {
        let session = session.as_mut();

        if let SessionState::Placing(placing) = &mut session.state {
            if let Some(to_place) = placing.to_place_pos() {
                let entities = session.tiles.get(to_place).unwrap();

                // Highlight placable
                if let Ok(mut visibility) = visibility_query.get_mut(entities.placeable()) {
                    *visibility = Visibility::Visible;
                }

                // Unhighlight to place
                if let Ok(mut visibility) = visibility_query.get_mut(entities.source_or_target()) {
                    *visibility = Visibility::Hidden;
                }

                placing.clear_to_place();
            }
        }
    }

    let mut tiles = TileIndex::new();

    // Spawn board root
    let board_root = commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            PlayingMarker,
        ))
        .id();

    // Spawn tiles transform
    let tiles_transform = commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, -BoardRuleSet::tile_height() / 2.0, 0.0)),
            GlobalTransform::default(),
        ))
        .id();

    commands.entity(board_root).add_child(tiles_transform);

    let tile_mesh = meshes.add(Cuboid::new(
        BoardRuleSet::tile_size(),
        BoardRuleSet::tile_height(),
        BoardRuleSet::tile_size(),
    ));

    // Spawn tiles
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

            let tile_root = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(base_color),
                    pos_to_world(pos, board),
                    GlobalTransform::default(),
                    Tile::new(pos),
                ))
                .observe(on_tile_hovered)
                .observe(on_tile_out)
                .id();

            let source_or_target = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Visibility::Hidden,
                ))
                .id();

            let placeable = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(assets.materials.common.highlight_placeable.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Visibility::Hidden,
                ))
                .id();

            commands.entity(tiles_transform).add_child(tile_root);

            commands
                .entity(tile_root)
                .add_children(&[source_or_target, placeable]);

            // Add to tile index
            tiles.add(
                pos,
                TileEntities::new(tile_root, source_or_target, placeable),
            );
        }
    }

    (board_root, tiles)
}

/// Spawns a piece on the board at the specified position with the given model and color.
fn spawn_placed_piece(
    commands: &mut Commands,
    assets: &GameAssets,
    board_rule_set: &BoardRuleSet,
    board_entity: Entity,
    players: &mut Players,
    placed_pieces: &mut PlacedPieceIndex,
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
) -> Result<(), GameError> {
    fn on_piece_pressed(
        trigger: Trigger<Pointer<Pressed>>,
        mut commands: Commands,
        child_query: Query<&ChildOf>,
        placed_piece_query: Query<&PlacedPiece>,
        tile_query: Query<&Tile>,
        mut visibility_query: Query<&mut Visibility>,
        mut session: ResMut<GameSession>,
        rules: Res<LoadedRules>,
    ) {
        if let SessionState::Selecting = session.state {
            // Skip if the pointer event is not primary click
            if trigger.button != PointerButton::Primary {
                return;
            }

            // Try to fetch the child component of the pressed entity
            let Ok(child) = child_query.get(trigger.target()) else {
                return;
            };

            // Try to fetch the selected placed piece
            let Ok(placed) = placed_piece_query.get(child.parent()) else {
                return;
            };

            // If the piece color does not match the current player's color, do nothing
            if session
                .players
                .get_by_index(session.turn.current_player())
                .0
                != placed.color()
            {
                return;
            }

            // Try to create a move context from the selected piece
            let Ok(moving) = MovingPiece::new(
                placed.model(),
                placed.color(),
                placed.pos(),
                &mut session,
                placed_piece_query,
                rules.pieces.get_by_model(placed.model()).movement(),
                tile_query,
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
                if let Ok(mut visibility) = visibility_query.get_mut(entities.highlight()) {
                    *visibility = Visibility::Visible;
                }

                // Highlight move initial tile
                if let Ok(mut visibility) = visibility_query
                    .get_mut(session.tiles.get(placed.pos()).unwrap().source_or_target())
                {
                    *visibility = Visibility::Visible;
                }

                // Highlight placeable tiles
                for pos in moving.placeable_tiles() {
                    if let Ok(mut visibility) =
                        visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                    {
                        *visibility = Visibility::Visible;
                    }
                }
            }

            // Apply component state change
            commands
                .entity(entities.root())
                .insert(moving)
                .remove::<PlacedPiece>();

            // Enter moving state
            session.state = SessionState::Moving(entities);
        }
    }

    let Entry::Vacant(entry) = placed_pieces.entry(pos) else {
        // Try to spawn duplicate piece at one position.
        return Err(GameError::DuplicatePiece(pos));
    };

    // Decrease the piece stock
    players.get_by_color_mut(color).decrease_stock(model)?;

    let (mesh, local_transform) = assets.meshes.piece.get(model);

    let piece_root = commands
        .spawn((
            pos_to_world(pos, board_rule_set),
            GlobalTransform::default(),
            PlacedPiece::new(model, color, pos),
        ))
        .id();

    let base_mesh = commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(assets.materials.piece.get(color).clone()),
            local_transform.clone(),
            GlobalTransform::default(),
        ))
        .observe(on_piece_pressed)
        .id();

    let highlight = commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
            local_transform.clone(),
            GlobalTransform::default(),
            Visibility::Hidden,
        ))
        .id();

    commands.entity(board_entity).add_child(piece_root);
    commands.entity(piece_root).add_child(base_mesh);
    commands.entity(base_mesh).add_child(highlight);

    // Add to placed piece index
    entry.insert(PieceEntities::new(piece_root, base_mesh, highlight));

    Ok(())
}

/// Despawns a piece at the specified position.
fn despawn_placed_piece(commands: &mut Commands, index: &mut PlacedPieceIndex, pos: Pos) {
    if let Some(entities) = index.remove(pos) {
        commands.entity(entities.root()).despawn();
    }
}

/// Converts a logical board position to world space translation.
///
/// (0, 0) is the bottom-left tile on the board.
fn pos_to_world(pos: Pos, board: &BoardRuleSet) -> Transform {
    const fn half_len(cols_or_rows: i64) -> f32 {
        (cols_or_rows as f32 - 1.0) * BoardRuleSet::tile_size() / 2.0
    }

    Transform::from_translation(Vec3::new(
        pos.col() as f32 * BoardRuleSet::tile_size() - half_len(board.cols()),
        0.0,
        half_len(board.rows()) - pos.row() as f32 * BoardRuleSet::tile_size(),
    ))
}

/// Finishes the current turn, evaluates win/loss conditions, and prepares for the next turn or ends the game.
fn finish_turn(
    rules: &LoadedRules,
    session: &mut GameSession,
    placed_piece_query: Query<&PlacedPiece>,
    top_panel_text: &mut TopPanelText,
) {
    // Check win and lose condition for each active player.
    for (piece_color, player) in session
        .players
        .iter_mut()
        .filter(|(_, player)| player.state() == PlayerState::Active)
    {
        let player_rules = rules.players.get_by_color(piece_color);

        let ctx = WinOrLoseContext {
            turn: &session.turn,
            last_action: &session.last_action,
            placed_piece_index: &session.placed_pieces,
            placed_piece_query,
        };

        // Check win condition
        if player_rules.win_condition().evaluate(&ctx).unwrap() {
            player.set_state(PlayerState::Won);

            // If the player has won, we don't check it's lose condition.
            continue;
        }

        // Check lose condition
        if player_rules.lose_condition().evaluate(&ctx).unwrap() {
            player.set_state(PlayerState::Lost);
        }
    }

    let ctx = GameOverContext {
        session,
        query: placed_piece_query,
    };

    // Check game over condition
    if rules.game_over_condition.evaluate(&ctx).unwrap() {
        // Update top panel text.
        top_panel_text.0 = format!("Game Over: {}", session.players.player_states_message());

        // Switch to [`SessionSate::Reviewing`].
        session.state = SessionState::Reviewing;
    } else {
        // Advance the turn
        match session.turn.advance_turn(&session.players) {
            Ok(()) => {
                top_panel_text.0 = session.turn.turn_message(&session.players);

                // Switch to [`SessionSate::Selecting`].
                session.state = SessionState::Selecting;
            }
            Err(GameError::NoActivePlayer) => {
                top_panel_text.0 = format!(
                    "No Active Player: {}",
                    session.players.player_states_message()
                );

                // Switch to [`SessionSate::Reviewing`].
                session.state = SessionState::Reviewing;
            }
            Err(_) => {
                panic!("Unexpected error occurred");
            }
        }
    }
}

fn top_panel(mut egui: EguiContexts, text: Res<TopPanelText>) {
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
                |ui_ctx| ui_ctx.label(egui::RichText::new(&text.0).strong().size(24.0)),
            );
        });
}

fn stock_panel(
    mut egui: EguiContexts,
    tile_query: Query<&Tile>,
    placed_piece_query: Query<&PlacedPiece>,
    mut visibility_query: Query<&mut Visibility>,
    rules: Res<LoadedRules>,
    mut session: ResMut<GameSession>,
) {
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

                    let (piece_color, player) =
                        session.players.get_by_index(session.turn.current_player());

                    for (model, count) in player.stocks() {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{model}: {count}"))
                                .size(18.0)
                                .strong(),
                        );

                        // Enable the button if the session is in selecting state and the count is not depleted
                        let enabled = matches!(session.state, SessionState::Selecting)
                            && !count.is_depleted();

                        if ui.add_enabled(enabled, button).clicked() {
                            let placing = PlacingPiece::new(
                                model,
                                piece_color,
                                session,
                                placed_piece_query,
                                rules.pieces.get_by_model(model).placement(),
                                tile_query,
                            )
                            .unwrap();

                            // Highlight placeable tiles
                            for pos in placing.placeable_tiles() {
                                if let Ok(mut visibility) = visibility_query
                                    .get_mut(session.tiles.get(pos).unwrap().placeable())
                                {
                                    *visibility = Visibility::Visible;
                                }
                            }

                            // Enter placing state
                            session.state = SessionState::Placing(placing);
                        }
                    }
                });
            });
        });
}
