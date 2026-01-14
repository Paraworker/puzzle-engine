use crate::{
    assets::GameAssets,
    states::{
        game_setup::LoadedRules,
        playing::{
            TileEnter, TileOut, TileRelease,
            phases::GamePhase,
            piece::{PlacingPiece, capture_piece},
            place_new_piece,
            session::GameSession,
            tile::Tile,
        },
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use rulery::piece::{PieceColor, PieceModel};

pub fn start_place_piece(
    commands: &mut Commands,
    next_phase: &mut NextState<GamePhase>,
    model: PieceModel,
    color: PieceColor,
) {
    commands.insert_resource(PlacingPiece::new(model, color));
    next_phase.set(GamePhase::Placing);
}

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Placing), on_enter)
            .add_systems(
                Update,
                (
                    on_tile_release,
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
    mut tile_enter: Option<ResMut<Messages<TileEnter>>>,
    mut tile_out: Option<ResMut<Messages<TileOut>>>,
    mut tile_release: Option<ResMut<Messages<TileRelease>>>,
    mut pointer_press: Option<ResMut<Messages<Pointer<Press>>>>,
    mut vis_query: Query<&mut Visibility>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    session: Res<GameSession>,
    mut data: ResMut<PlacingPiece>,
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

fn on_tile_release(
    mut press: MessageReader<TileRelease>,
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

    let Some(msg) = press.read().last() else {
        return;
    };

    if msg.1 != PointerButton::Primary {
        return;
    }

    let session = session.as_mut();

    let child = child_query.get(msg.0).unwrap();
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
        place_new_piece(
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
    mut enter: MessageReader<TileEnter>,
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

    for msg in enter.read() {
        let child = child_query.get(msg.0).unwrap();
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
    mut out: MessageReader<TileOut>,
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

    for msg in out.read() {
        let child = child_query.get(msg.0).unwrap();
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
    mut press: MessageReader<Pointer<Press>>,
    mut egui: EguiContexts,
    mut next_phase: ResMut<NextState<GamePhase>>,
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
