use crate::{
    assets::GameAssets,
    states::{
        game_setup::LoadedRules,
        playing::{
            TileEnter, TileOut, despawn_placed_piece,
            phases::GamePhase,
            piece::{PlacedPiece, PlacingPiece},
            session::{GameSession, tile_index::TileIndex},
            spawn_placed_piece,
            tile::Tile,
        },
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use rule_engine::pos::Pos;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Placing), on_enter)
            .add_systems(
                Update,
                (on_button_pressed, on_tile_enter, on_tile_out)
                    .run_if(in_state(GamePhase::Placing)),
            )
            .add_systems(OnExit(GamePhase::Placing), on_exit);
    }
}

fn on_enter(
    mut visibility_query: Query<&mut Visibility>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    placed_piece_query: Query<&PlacedPiece>,
    session: Res<GameSession>,
    mut data: ResMut<PlacingPiece>,
) {
    let placement = rules.pieces.get_by_model(data.model()).placement();

    // Collect placeable tiles
    data.collect_placeable(&session, placed_piece_query, tile_query, placement)
        .unwrap();

    // Highlight placeable tiles
    for pos in data.placeable_tiles() {
        if let Ok(mut visibility) =
            visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
        {
            *visibility = Visibility::Visible;
        }
    }
}

fn on_exit(
    mut commands: Commands,
    mut visibility_query: Query<&mut Visibility>,
    session: Res<GameSession>,
    data: Res<PlacingPiece>,
) {
    // Unhighlight placeable tiles
    for pos in data.placeable_tiles() {
        if let Ok(mut visibility) =
            visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
        {
            *visibility = Visibility::Hidden;
        }
    }

    commands.remove_resource::<PlacingPiece>();
}

/// A system that triggered when the primary button is pressed.
fn on_button_pressed(
    mut pressed: EventReader<Pointer<Pressed>>,
    mut egui: EguiContexts,
    mut commands: Commands,
    mut visibility_query: Query<&mut Visibility>,
    assets: Res<GameAssets>,
    rules: Res<LoadedRules>,
    data: Res<PlacingPiece>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    for event in pressed.read() {
        if event.button == PointerButton::Primary {
            if let Some(to_place) = data.to_place_pos() {
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
                    data.model(),
                    data.color(),
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
                next_phase.set(GamePhase::TurnEnd);
            } else {
                // Cancelled.
                next_phase.set(GamePhase::Selecting);
            }

            // We only handle the first release event
            break;
        }
    }
}

fn on_tile_enter(
    mut enter: EventReader<TileEnter>,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    mut visibility_query: Query<&mut Visibility>,
    mut data: ResMut<PlacingPiece>,
    session: Res<GameSession>,
) {
    let Some(event) = enter.read().last() else {
        return;
    };

    let child = child_query.get(event.0).unwrap();
    let tile = tile_query.get(child.parent()).unwrap();

    apply_to_place(
        &mut visibility_query,
        &session.tiles,
        &mut data,
        Some(tile.pos()),
    );
}

fn on_tile_out(
    mut out: EventReader<TileOut>,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    mut visibility_query: Query<&mut Visibility>,
    mut data: ResMut<PlacingPiece>,
    session: Res<GameSession>,
) {
    let Some(to_place) = data.to_place_pos() else {
        return;
    };

    for event in out.read() {
        let child = child_query.get(event.0).unwrap();
        let tile = tile_query.get(child.parent()).unwrap();

        if tile.pos() == to_place {
            apply_to_place(&mut visibility_query, &session.tiles, &mut data, None);
            break;
        }
    }
}

fn apply_to_place(
    visibility_query: &mut Query<&mut Visibility>,
    tiles: &TileIndex,
    data: &mut PlacingPiece,
    new_to_place: Option<Pos>,
) {
    // Clear the previous to place position if any
    if let Some(old) = data.clear_to_place_pos() {
        let entities = tiles.get(old).unwrap();

        // Unhighlight to place
        if let Ok(mut visibility) = visibility_query.get_mut(entities.source_or_target()) {
            *visibility = Visibility::Hidden;
        }

        // Highlight placable
        if let Ok(mut visibility) = visibility_query.get_mut(entities.placeable()) {
            *visibility = Visibility::Visible;
        }
    }

    // Set the new to place position if any
    if let Some(pos) = new_to_place {
        if data.set_to_place_pos(pos) {
            let entities = tiles.get(pos).unwrap();

            // Unhighlight placable
            if let Ok(mut visibility) = visibility_query.get_mut(entities.placeable()) {
                *visibility = Visibility::Hidden;
            }

            // Highlight to place
            if let Ok(mut visibility) = visibility_query.get_mut(entities.source_or_target()) {
                *visibility = Visibility::Visible;
            }
        }
    }
}
