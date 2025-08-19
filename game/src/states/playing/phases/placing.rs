use crate::{
    assets::GameAssets,
    states::{
        game_setup::LoadedRules,
        playing::{
            TileEnter, TileOut, TileReleased, capture_piece, phases::GamePhase,
            piece::PlacingPiece, place_piece, session::GameSession, tile::Tile,
        },
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Placing), on_enter)
            .add_systems(
                Update,
                (
                    on_tile_released,
                    on_tile_enter,
                    on_tile_out,
                    on_secondary_cancel,
                    on_escape_cancel,
                )
                    .run_if(in_state(GamePhase::Placing)),
            )
            .add_systems(OnExit(GamePhase::Placing), on_exit);
    }
}

fn on_enter(
    mut tile_enter: Option<ResMut<Events<TileEnter>>>,
    mut tile_out: Option<ResMut<Events<TileOut>>>,
    mut tile_released: Option<ResMut<Events<TileReleased>>>,
    mut pointer_pressed: Option<ResMut<Events<Pointer<Pressed>>>>,
    mut vis_query: Query<&mut Visibility>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    session: Res<GameSession>,
    mut data: ResMut<PlacingPiece>,
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

    let placement = rules.get_piece(data.model()).unwrap();

    // Collect placeable tiles
    data.collect_placeable(&session, tile_query, placement)
        .unwrap();

    // Highlight placeable tiles
    for pos in data.placeable_tiles() {
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
    data: Res<PlacingPiece>,
) {
    // Unhighlight tiles
    for pos in data.placeable_tiles() {
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

    commands.remove_resource::<PlacingPiece>();
}

fn on_tile_released(
    mut pressed: EventReader<TileReleased>,
    mut egui: EguiContexts,
    mut commands: Commands,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    assets: Res<GameAssets>,
    rules: Res<LoadedRules>,
    data: Res<PlacingPiece>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
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

    if event.1 != PointerButton::Primary {
        return;
    }

    let session = session.as_mut();

    let child = child_query.get(event.0).unwrap();
    let tile = tile_query.get(child.parent()).unwrap();

    if data.can_place_at(tile.pos()) {
        // If the to place position is already occupied, remove the existing piece (i.e. capture it)
        capture_piece(
            &mut commands,
            &mut session.placed_pieces,
            &mut session.players,
            tile.pos(),
        );

        // Spawn the placed piece at the target position
        place_piece(
            &mut commands,
            &assets,
            &rules,
            session.board,
            &mut session.players,
            &mut session.placed_pieces,
            data.model(),
            data.color(),
            tile.pos(),
        )
        .unwrap();

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
    data: Res<PlacingPiece>,
    session: Res<GameSession>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    for event in enter.read() {
        let child = child_query.get(event.0).unwrap();
        let tile = tile_query.get(child.parent()).unwrap();

        if data.can_place_at(tile.pos()) {
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
    data: Res<PlacingPiece>,
    session: Res<GameSession>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    for event in out.read() {
        let child = child_query.get(event.0).unwrap();
        let tile = tile_query.get(child.parent()).unwrap();

        if data.can_place_at(tile.pos()) {
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
    mut next_phase: ResMut<NextState<GamePhase>>,
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

    cancel_place(&mut next_phase);
}

fn on_escape_cancel(
    keys: Res<ButtonInput<KeyCode>>,
    mut egui: EguiContexts,
    mut next_phase: ResMut<NextState<GamePhase>>,
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

    cancel_place(&mut next_phase);
}

fn cancel_place(next: &mut NextState<GamePhase>) {
    next.set(GamePhase::Selecting);
}
