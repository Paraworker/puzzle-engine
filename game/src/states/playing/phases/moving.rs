use crate::states::{
    game_setup::LoadedRules,
    playing::{
        TileEnter, TileOut, TileRelease,
        board::pos_translation,
        phases::GamePhase,
        piece::{MovingPiece, PiecePos, PlacedPiece, capture_piece},
        session::{GameSession, PlacedPieceIndex, player::Players, turn::TurnController},
        tile::Tile,
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_tweening::{AnimTarget, Lens, Tween, TweenAnim};
use rulery::pos::Pos;
use std::{collections::hash_map::Entry, time::Duration};

pub fn start_move_piece(
    commands: &mut Commands,
    placed_piece_index: &mut PlacedPieceIndex,
    players: &Players,
    turn: &TurnController,
    next_phase: &mut NextState<GamePhase>,
    at: Pos,
) {
    let Entry::Occupied(entry) = placed_piece_index.entry(at) else {
        panic!("No placed piece at position: {:?}", at);
    };

    // If the piece color does not match the current player's color, do nothing
    if players.get_by_index(turn.current_player()).0 != entry.get().color() {
        return;
    }

    // Remove the record from the placed piece index
    let placed = entry.remove();

    // Enter moving state
    commands.insert_resource(MovingPiece::new(
        placed.model(),
        placed.color(),
        placed.pos(),
        placed.entities().clone(),
    ));

    next_phase.set(GamePhase::Moving);
}

pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Moving), on_enter)
            .add_systems(
                Update,
                (
                    on_tile_release,
                    on_tile_enter,
                    on_tile_out,
                    on_secondary_cancel,
                    on_escape_cancel,
                )
                    .run_if(in_state(GamePhase::Moving)),
            )
            .add_systems(OnExit(GamePhase::Moving), on_exit);
    }
}

fn on_enter(
    mut tile_enter: Option<ResMut<Messages<TileEnter>>>,
    mut tile_out: Option<ResMut<Messages<TileOut>>>,
    mut tile_release: Option<ResMut<Messages<TileRelease>>>,
    mut pointer_press: Option<ResMut<Messages<Pointer<Press>>>>,
    tile_query: Query<&Tile>,
    mut vis_query: Query<&mut Visibility>,
    rules: Res<LoadedRules>,
    session: Res<GameSession>,
    mut data: ResMut<MovingPiece>,
) {
    // Clear messages
    // In case the old messages are still in the queue
    if let Some(tile_enter) = &mut tile_enter {
        tile_enter.clear();
    }

    if let Some(tile_out) = &mut tile_out {
        tile_out.clear();
    }

    if let Some(tile_release) = &mut tile_release {
        tile_release.clear();
    }

    if let Some(pointer_press) = &mut pointer_press {
        pointer_press.clear();
    }

    let rules = rules.get_piece(data.model()).unwrap();

    // Collect movable tiles
    data.collect_movable(&session, tile_query, rules).unwrap();

    // Highlight the moving piece
    vis_query
        .get_mut(data.entities().highlight())
        .unwrap()
        .set_if_neq(Visibility::Visible);

    // Highlight movable tiles
    for pos in data.movable_tiles() {
        vis_query
            .get_mut(session.tiles.get(&pos).unwrap().placeable())
            .unwrap()
            .set_if_neq(Visibility::Visible);
    }
}

fn on_exit(
    mut commands: Commands,
    mut vis_query: Query<&mut Visibility>,
    session: Res<GameSession>,
    data: Res<MovingPiece>,
) {
    // Unhighlight moving piece
    vis_query
        .get_mut(data.entities().highlight())
        .unwrap()
        .set_if_neq(Visibility::Hidden);

    // Unhighlight tiles
    for pos in data.movable_tiles() {
        let entities = session.tiles.get(&pos).unwrap();

        vis_query
            .get_mut(entities.placeable())
            .unwrap()
            .set_if_neq(Visibility::Hidden);

        vis_query
            .get_mut(entities.source_or_target())
            .unwrap()
            .set_if_neq(Visibility::Hidden);
    }

    commands.remove_resource::<MovingPiece>();
}

fn on_tile_release(
    mut release: MessageReader<TileRelease>,
    mut commands: Commands,
    mut egui: EguiContexts,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    mut piece_query: Query<(&Transform, &mut PiecePos)>,
    rules: Res<LoadedRules>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    data: Res<MovingPiece>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let Some(msg) = release.read().last() else {
        return;
    };

    if msg.1 != PointerButton::Primary {
        return;
    }

    let session = session.as_mut();

    let child = child_query.get(msg.0).unwrap();
    let tile = tile_query.get(child.parent()).unwrap();

    if data.can_move_to(tile.pos()) {
        // If the target position is already occupied, capture it.
        capture_piece(
            &mut commands,
            &mut session.placed_pieces,
            &mut session.players,
            tile.pos(),
        );

        let (transform, mut piece_pos) = piece_query.get_mut(data.entities().root()).unwrap();

        // Animation
        {
            let start = transform.translation;
            let end = pos_translation(tile.pos(), &rules).translation;

            // Determine the height of the parabola
            // The height is 25% of the distance between start and end
            let dist = start.distance(end);
            let height = dist * 0.25;

            let tween = Tween::new(
                EaseFunction::SineInOut,
                Duration::from_millis(220),
                TransformParabolaLens {
                    start,
                    end,
                    up: Vec3::Y,
                    height,
                },
            );

            commands.spawn((
                TweenAnim::new(tween),
                AnimTarget::component::<Transform>(data.entities().root()),
            ));
        }

        // Update piece pos
        piece_pos.0 = tile.pos();

        // Add record to the placed piece index at the current position
        session.placed_pieces.insert(
            tile.pos(),
            PlacedPiece::new(
                data.model(),
                data.color(),
                tile.pos(),
                data.entities().clone(),
            ),
        );

        // Update last action position
        session.last_action = Some(tile.pos());

        // Finish this turn
        next_phase.set(GamePhase::TurnEnd);
    }
}

fn on_tile_enter(
    mut enter: MessageReader<TileEnter>,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    mut vis_query: Query<&mut Visibility>,
    session: Res<GameSession>,
    data: Res<MovingPiece>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    for msg in enter.read() {
        let child = child_query.get(msg.0).unwrap();
        let tile = tile_query.get(child.parent()).unwrap();

        if data.can_move_to(tile.pos()) {
            let entities = session.tiles.get(&tile.pos()).unwrap();

            vis_query
                .get_mut(entities.placeable())
                .unwrap()
                .set_if_neq(Visibility::Hidden);

            vis_query
                .get_mut(entities.source_or_target())
                .unwrap()
                .set_if_neq(Visibility::Visible);
        }
    }
}

fn on_tile_out(
    mut out: MessageReader<TileOut>,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    mut vis_query: Query<&mut Visibility>,
    data: Res<MovingPiece>,
    session: Res<GameSession>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    for msg in out.read() {
        let child = child_query.get(msg.0).unwrap();
        let tile = tile_query.get(child.parent()).unwrap();

        if data.can_move_to(tile.pos()) {
            let entities = session.tiles.get(&tile.pos()).unwrap();

            vis_query
                .get_mut(entities.placeable())
                .unwrap()
                .set_if_neq(Visibility::Visible);

            vis_query
                .get_mut(entities.source_or_target())
                .unwrap()
                .set_if_neq(Visibility::Hidden);
        }
    }
}

fn on_secondary_cancel(
    mut press: MessageReader<Pointer<Press>>,
    mut egui: EguiContexts,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    data: Res<MovingPiece>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let Some(msg) = press.read().last() else {
        return;
    };

    if msg.button != PointerButton::Secondary {
        return;
    }

    cancel_move(&mut session, &mut next_phase, &data);
}

fn on_escape_cancel(
    keys: Res<ButtonInput<KeyCode>>,
    mut egui: EguiContexts,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    data: Res<MovingPiece>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    if egui.ctx_mut().unwrap().wants_keyboard_input() {
        return;
    }

    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    cancel_move(&mut session, &mut next_phase, &data);
}

fn cancel_move(session: &mut GameSession, next: &mut NextState<GamePhase>, data: &MovingPiece) {
    session.placed_pieces.insert(
        data.source_pos(),
        PlacedPiece::new(
            data.model(),
            data.color(),
            data.source_pos(),
            data.entities().clone(),
        ),
    );

    next.set(GamePhase::Selecting);
}

#[derive(Clone, Copy)]
struct TransformParabolaLens {
    start: Vec3,
    end: Vec3,
    up: Vec3,
    height: f32,
}

impl Lens<Transform> for TransformParabolaLens {
    fn lerp(&mut self, mut target: Mut<'_, Transform>, ratio: f32) {
        let p = self.start.lerp(self.end, ratio);
        let bump = self.up.normalize() * (self.height * 4.0 * ratio * (1.0 - ratio));
        target.translation = p + bump;
    }
}
