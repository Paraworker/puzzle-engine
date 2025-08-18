use crate::{
    GameError,
    assets::GameAssets,
    states::{
        AppState,
        game_setup::LoadedRules,
        playing::{
            camera::PlayingCamera,
            phases::{GamePhase, GamePhasePlugin},
            piece::{PieceEntities, PlacedPiece, PlacingPiece},
            session::{
                GameSession, PlacedPieceIndex, TileIndex, player::Players, turn::TurnController,
            },
            tile::{Tile, TileEntities},
        },
    },
};
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPrimaryContextPass, PrimaryEguiContext,
    egui::{self, Stroke},
};
use rule_engine::{
    CheckedGameRules,
    piece::{PieceColor, PieceModel},
    pos::Pos,
};
use std::collections::hash_map::Entry;

pub mod camera;
pub mod phases;
pub mod piece;
pub mod session;
pub mod tile;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamePhasePlugin)
            .add_event::<TileEnter>()
            .add_event::<TileOut>()
            .add_event::<PiecePressed>()
            .add_systems(OnEnter(AppState::Playing), on_enter)
            .add_systems(OnExit(AppState::Playing), on_exit)
            .add_systems(
                EguiPrimaryContextPass,
                (top_panel, bottom_panel).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Event)]
pub struct TileEnter(pub Entity);

#[derive(Event)]
pub struct TileOut(pub Entity);

#[derive(Event)]
pub struct PiecePressed(pub Entity, pub PointerButton);

#[derive(Resource)]
struct TopPanelText(String);

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
        place_piece(
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

/// Spawns the board.
fn spawn_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    rules: &CheckedGameRules,
) -> (Entity, TileIndex) {
    fn on_tile_enter(trigger: Trigger<Pointer<Over>>, mut ev: EventWriter<TileEnter>) {
        ev.write(TileEnter(trigger.target()));
    }

    fn on_tile_out(trigger: Trigger<Pointer<Out>>, mut ev: EventWriter<TileOut>) {
        ev.write(TileOut(trigger.target()));
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
            Transform::from_translation(Vec3::new(
                0.0,
                -CheckedGameRules::tile_height() / 2.0,
                0.0,
            )),
            GlobalTransform::default(),
        ))
        .id();

    commands.entity(board_root).add_child(tiles_transform);

    let tile_mesh = meshes.add(Cuboid::new(
        CheckedGameRules::tile_size(),
        CheckedGameRules::tile_height(),
        CheckedGameRules::tile_size(),
    ));

    // Spawn tiles
    for col in 0..rules.board_cols() {
        for row in 0..rules.board_rows() {
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
                    pos_translation(pos, rules),
                    GlobalTransform::default(),
                    Tile::new(pos),
                ))
                .id();

            let base_mesh = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(base_color),
                    Transform::default(),
                    GlobalTransform::default(),
                ))
                .observe(on_tile_enter)
                .observe(on_tile_out)
                .id();

            let source_or_target = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Pickable::IGNORE,
                    Visibility::Hidden,
                ))
                .id();

            let placeable = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(assets.materials.common.highlight_placeable.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Pickable::IGNORE,
                    Visibility::Hidden,
                ))
                .id();

            commands.entity(tiles_transform).add_child(tile_root);

            commands
                .entity(tile_root)
                .add_children(&[base_mesh, source_or_target, placeable]);

            // Add to tile index
            tiles.insert(
                pos,
                TileEntities::new(tile_root, base_mesh, source_or_target, placeable),
            );
        }
    }

    (board_root, tiles)
}

/// Spawns a piece on the board at the specified position with the given model and color.
fn place_piece(
    commands: &mut Commands,
    assets: &GameAssets,
    rules: &CheckedGameRules,
    board_entity: Entity,
    players: &mut Players,
    placed_pieces: &mut PlacedPieceIndex,
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
) -> Result<(), GameError> {
    fn on_piece_pressed(trigger: Trigger<Pointer<Pressed>>, mut ev: EventWriter<PiecePressed>) {
        ev.write(PiecePressed(trigger.target(), trigger.button));
    }

    let Entry::Vacant(entry) = placed_pieces.entry(pos) else {
        // Try to spawn duplicate piece at one position.
        return Err(GameError::DuplicatePiece(pos));
    };

    // Decrease the piece stock
    players
        .get_by_color_mut(color)
        .piece_mut(model)
        .try_take_stock()?;

    let (mesh, local_transform) = assets.meshes.piece.get(model);

    let piece_root = commands
        .spawn((
            pos_translation(pos, rules),
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
            Pickable::IGNORE,
            Visibility::Hidden,
        ))
        .id();

    commands.entity(board_entity).add_child(piece_root);
    commands
        .entity(piece_root)
        .add_children(&[base_mesh, highlight]);

    // Add to placed piece index
    entry.insert(PieceEntities::new(piece_root, base_mesh, highlight));

    Ok(())
}

/// Despawns a piece at the specified position.
fn capture_piece(
    commands: &mut Commands,
    placed_piece_query: Query<&PlacedPiece>,
    placed_piece_index: &mut PlacedPieceIndex,
    players: &mut Players,
    pos: Pos,
) {
    if let Some(entities) = placed_piece_index.remove(&pos) {
        let placed = placed_piece_query.get(entities.root()).unwrap();

        players
            .get_by_color_mut(placed.color())
            .piece_mut(placed.model())
            .record_capture();

        commands.entity(entities.root()).despawn();
    }
}

/// Converts a logical board position to board space translation.
///
/// (0, 0) is the bottom-left tile on the board.
fn pos_translation(pos: Pos, rules: &CheckedGameRules) -> Transform {
    const fn half_len(cols_or_rows: i64) -> f32 {
        (cols_or_rows as f32 - 1.0) * CheckedGameRules::tile_size() / 2.0
    }

    Transform::from_translation(Vec3::new(
        pos.col() as f32 * CheckedGameRules::tile_size() - half_len(rules.board_cols()),
        0.0,
        half_len(rules.board_rows()) - pos.row() as f32 * CheckedGameRules::tile_size(),
    ))
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

fn bottom_panel(
    mut commands: Commands,
    mut egui: EguiContexts,
    mut session: ResMut<GameSession>,
    next_state: Res<NextState<AppState>>,
    current_phase: Res<State<GamePhase>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_state {
        return;
    }

    let session = session.as_mut();

    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .inner_margin(egui::Margin::symmetric(10, 8)),
        )
        .show(egui.ctx_mut().unwrap(), |ui| {
            let (piece_color, player) = session.players.get_by_index(session.turn.current_player());

            egui::ScrollArea::horizontal().show(ui, |ui| {
                // Row 1: In Stock
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("In Stock").size(18.0).monospace())
                        .on_hover_text("Number of your pieces available for placement");

                    ui.separator();

                    for (model, piece) in player.pieces() {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{} × {}", model, piece.stock()))
                                .size(18.0)
                                .strong(),
                        );

                        // Enable the button if the session is in selecting state and the count is not depleted
                        let enabled = matches!(current_phase.get(), GamePhase::Selecting)
                            && !piece.stock().is_depleted();

                        if ui.add_enabled(enabled, button).clicked() {
                            // Avoid duplicate transition
                            if let NextState::Unchanged = *next_phase {
                                // Enter placing state
                                commands.insert_resource(PlacingPiece::new(model, piece_color));
                                next_phase.set(GamePhase::Placing);
                            }
                        }
                    }
                });

                ui.separator();

                // Row 2: Captured
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Captured").size(18.0).monospace())
                        .on_hover_text("Number of your pieces that have been captured");

                    ui.separator();

                    for (model, piece) in player.pieces() {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{} × {}", model, piece.captured()))
                                .size(18.0),
                        );

                        ui.add_enabled(false, button);
                    }
                });
            });
        });
}
