use crate::states::{
    game_setup::LoadedRules,
    playing::{
        TileEnter, TileOut, TileReleased, capture_piece,
        phases::GamePhase,
        piece::{MovingPiece, PiecePos, PlacedPiece},
        pos_translation,
        session::GameSession,
        tile::Tile,
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};
use std::time::Duration;

pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Moving), on_enter)
            .add_systems(
                Update,
                (
                    on_tile_released,
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
    mut tile_enter: Option<ResMut<Events<TileEnter>>>,
    mut tile_out: Option<ResMut<Events<TileOut>>>,
    mut tile_released: Option<ResMut<Events<TileReleased>>>,
    mut pointer_pressed: Option<ResMut<Events<Pointer<Pressed>>>>,
    tile_query: Query<&Tile>,
    mut vis_query: Query<&mut Visibility>,
    rules: Res<LoadedRules>,
    session: Res<GameSession>,
    mut data: ResMut<MovingPiece>,
) {
    // Clear events
    // In case the old events are still in the queue
    if let Some(tile_enter) = &mut tile_enter {
        tile_enter.clear();
    }

    if let Some(tile_out) = &mut tile_out {
        tile_out.clear();
    }

    if let Some(tile_released) = &mut tile_released {
        tile_released.clear();
    }

    if let Some(pointer_pressed) = &mut pointer_pressed {
        pointer_pressed.clear();
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

fn on_tile_released(
    mut released: EventReader<TileReleased>,
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

    let Some(event) = released.read().last() else {
        return;
    };

    if event.1 != PointerButton::Primary {
        return;
    }

    let session = session.as_mut();

    let child = child_query.get(event.0).unwrap();
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

            let tween = Tween::new(
                EaseFunction::CubicInOut,
                Duration::from_millis(200),
                TransformPositionLens { start, end },
            );

            commands
                .entity(data.entities().root())
                .insert(Animator::new(tween));
        };

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
    mut enter: EventReader<TileEnter>,
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

    for event in enter.read() {
        let child = child_query.get(event.0).unwrap();
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
    mut out: EventReader<TileOut>,
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

    for event in out.read() {
        let child = child_query.get(event.0).unwrap();
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
    mut pressed: EventReader<Pointer<Pressed>>,
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

    let Some(event) = pressed.read().last() else {
        return;
    };

    if event.button != PointerButton::Secondary {
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
