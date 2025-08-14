use crate::{
    assets::GameAssets,
    states::{
        game_setup::LoadedRules,
        playing::{
            PlayingEvent, despawn_placed_piece, phases::GamePhase, piece::PlacingPiece,
            session::GameSession, spawn_placed_piece, tile::Tile,
        },
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

#[derive(Resource)]
pub struct PlacingData(pub PlacingPiece);

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Placing), on_enter)
            .add_systems(
                Update,
                (on_button_pressed, on_playing_event).run_if(in_state(GamePhase::Placing)),
            )
            .add_systems(OnExit(GamePhase::Placing), on_exit);
    }
}

fn on_enter() {
    // no-op
}

fn on_exit(mut commands: Commands) {
    commands.remove_resource::<PlacingData>();
}

/// A system that triggered when the primary button is pressed.
fn on_button_pressed(
    mut pressed: EventReader<Pointer<Pressed>>,
    mut egui: EguiContexts,
    mut commands: Commands,
    mut visibility_query: Query<&mut Visibility>,
    assets: Res<GameAssets>,
    rules: Res<LoadedRules>,
    data: Res<PlacingData>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    for event in pressed.read() {
        if event.button == PointerButton::Primary {
            // Unhighlight placeable tiles
            for pos in data.0.placeable_tiles() {
                if let Ok(mut visibility) =
                    visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                {
                    *visibility = Visibility::Hidden;
                }
            }

            if let Some(to_place) = data.0.to_place_pos() {
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
                    data.0.model(),
                    data.0.color(),
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
                // Placement cancelled.
                next_phase.set(GamePhase::Selecting);
            }

            // We only handle the first release event
            break;
        }
    }
}

fn on_playing_event(
    mut events: EventReader<PlayingEvent>,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    mut visibility_query: Query<&mut Visibility>,
    mut data: ResMut<PlacingData>,
    session: Res<GameSession>,
) {
    for event in events.read() {
        match event {
            PlayingEvent::TileHovered(entity) => on_tile_hovered(
                *entity,
                child_query,
                tile_query,
                &mut visibility_query,
                &mut data,
                &session,
            ),
            PlayingEvent::TileOut(_) => on_tile_out(&mut visibility_query, &mut data, &session),
            _ => {}
        }
    }
}

fn on_tile_hovered(
    entity: Entity,
    child_query: Query<&ChildOf>,
    tile_query: Query<&Tile>,
    visibility_query: &mut Query<&mut Visibility>,
    data: &mut PlacingData,
    session: &GameSession,
) {
    let Ok(child) = child_query.get(entity) else {
        return;
    };

    let Ok(tile) = tile_query.get(child.parent()) else {
        return;
    };

    // Attempt to place the piece
    if !data.0.try_place_at(tile.pos()) {
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

fn on_tile_out(
    visibility_query: &mut Query<&mut Visibility>,
    data: &mut PlacingData,
    session: &GameSession,
) {
    if let Some(to_place) = data.0.to_place_pos() {
        let entities = session.tiles.get(to_place).unwrap();

        // Highlight placable
        if let Ok(mut visibility) = visibility_query.get_mut(entities.placeable()) {
            *visibility = Visibility::Visible;
        }

        // Unhighlight to place
        if let Ok(mut visibility) = visibility_query.get_mut(entities.source_or_target()) {
            *visibility = Visibility::Hidden;
        }

        data.0.clear_to_place();
    }
}
