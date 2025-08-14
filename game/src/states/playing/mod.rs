use crate::{
    GameError,
    assets::GameAssets,
    states::{
        AppState,
        game_setup::LoadedRules,
        playing::{
            camera::PlayingCamera,
            phases::{GamePhase, GamePhasePlugin, placing::PlacingData},
            piece::{PlacedPiece, PlacingPiece},
            session::{
                GameSession,
                piece_index::{Entry, PieceEntities, PlacedPieceIndex},
                player::Players,
                tile_index::{TileEntities, TileIndex},
                turn::TurnController,
            },
            tile::Tile,
        },
    },
};
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPrimaryContextPass, PrimaryEguiContext,
    egui::{self, Stroke},
};
use rule_engine::{
    board::BoardRuleSet,
    piece::{PieceColor, PieceModel},
    position::Pos,
};

pub mod camera;
pub mod phases;
pub mod piece;
pub mod session;
pub mod tile;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamePhasePlugin)
            .add_event::<PlayingEvent>()
            .add_systems(OnEnter(AppState::Playing), on_enter)
            .add_systems(OnExit(AppState::Playing), on_exit)
            .add_systems(
                EguiPrimaryContextPass,
                (top_panel, stock_panel).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Event)]
pub enum PlayingEvent {
    TileHovered(Entity),
    TileOut(Entity),
    PiecePressed(Entity, PointerButton),
}

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
    board: &BoardRuleSet,
) -> (Entity, TileIndex) {
    fn on_tile_hovered(trigger: Trigger<Pointer<Over>>, mut ev: EventWriter<PlayingEvent>) {
        ev.write(PlayingEvent::TileHovered(trigger.target()));
    }

    fn on_tile_out(trigger: Trigger<Pointer<Out>>, mut ev: EventWriter<PlayingEvent>) {
        ev.write(PlayingEvent::TileOut(trigger.target()));
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
                    pos_translation(pos, board),
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
                .observe(on_tile_hovered)
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
            tiles.add(
                pos,
                TileEntities::new(tile_root, base_mesh, source_or_target, placeable),
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
    fn on_piece_pressed(trigger: Trigger<Pointer<Pressed>>, mut ev: EventWriter<PlayingEvent>) {
        ev.write(PlayingEvent::PiecePressed(trigger.target(), trigger.button));
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
            pos_translation(pos, board_rule_set),
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
fn despawn_placed_piece(commands: &mut Commands, index: &mut PlacedPieceIndex, pos: Pos) {
    if let Some(entities) = index.remove(pos) {
        commands.entity(entities.root()).despawn();
    }
}

/// Converts a logical board position to board space translation.
///
/// (0, 0) is the bottom-left tile on the board.
fn pos_translation(pos: Pos, board: &BoardRuleSet) -> Transform {
    const fn half_len(cols_or_rows: i64) -> f32 {
        (cols_or_rows as f32 - 1.0) * BoardRuleSet::tile_size() / 2.0
    }

    Transform::from_translation(Vec3::new(
        pos.col() as f32 * BoardRuleSet::tile_size() - half_len(board.cols()),
        0.0,
        half_len(board.rows()) - pos.row() as f32 * BoardRuleSet::tile_size(),
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

fn stock_panel(
    mut commands: Commands,
    mut egui: EguiContexts,
    tile_query: Query<&Tile>,
    placed_piece_query: Query<&PlacedPiece>,
    mut visibility_query: Query<&mut Visibility>,
    rules: Res<LoadedRules>,
    mut session: ResMut<GameSession>,
    phase: Res<State<GamePhase>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
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
                        let enabled =
                            matches!(phase.get(), GamePhase::Selecting) && !count.is_depleted();

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
                            commands.insert_resource(PlacingData(placing));
                            next_phase.set(GamePhase::Placing);
                        }
                    }
                });
            });
        });
}
