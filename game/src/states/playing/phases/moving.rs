use crate::states::{
    game_setup::LoadedRules,
    playing::{
        TileEnter, TileOut, capture_piece,
        phases::GamePhase,
        piece::{MovingPiece, PiecePos, PlacedPiece},
        pos_translation,
        session::{GameSession, TileIndex},
        tile::Tile,
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};
use rule_engine::pos::Pos;
use std::time::Duration;

pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Moving), on_enter)
            .add_systems(
                Update,
                (on_button_pressed, on_tile_enter, on_tile_out).run_if(in_state(GamePhase::Moving)),
            )
            .add_systems(OnExit(GamePhase::Moving), on_exit);
    }
}

fn on_enter(
    mut pressed: Option<ResMut<Events<Pointer<Pressed>>>>,
    mut tile_enter: Option<ResMut<Events<TileEnter>>>,
    mut tile_out: Option<ResMut<Events<TileOut>>>,
    mut visibility_query: Query<&mut Visibility>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    session: Res<GameSession>,
    mut data: ResMut<MovingPiece>,
) {
    // Clear events
    // In case the old events are still in the queue
    if let Some(pressed) = &mut pressed {
        pressed.clear();
    }

    if let Some(tile_enter) = &mut tile_enter {
        tile_enter.clear();
    }

    if let Some(tile_out) = &mut tile_out {
        tile_out.clear();
    }

    let rules = rules.get_piece(data.model()).unwrap();

    // Collect movable tiles
    data.collect_movable(&session, tile_query, rules).unwrap();

    // Highlight the moving piece
    if let Ok(mut visibility) = visibility_query.get_mut(data.entities().highlight()) {
        *visibility = Visibility::Visible;
    }

    // Highlight movable tiles
    for pos in data.movable_tiles() {
        if let Ok(mut visibility) =
            visibility_query.get_mut(session.tiles.get(&pos).unwrap().placeable())
        {
            *visibility = Visibility::Visible;
        }
    }
}

fn on_exit(
    mut commands: Commands,
    mut visibility_query: Query<&mut Visibility>,
    session: Res<GameSession>,
    data: Res<MovingPiece>,
) {
    // Unhighlight the moving piece
    if let Ok(mut visibility) = visibility_query.get_mut(data.entities().highlight()) {
        *visibility = Visibility::Hidden;
    }

    // Unhighlight movable tiles
    for pos in data.movable_tiles() {
        if let Ok(mut visibility) =
            visibility_query.get_mut(session.tiles.get(&pos).unwrap().placeable())
        {
            *visibility = Visibility::Hidden;
        }
    }

    commands.remove_resource::<MovingPiece>();
}

/// A system that triggered when the primary button is pressed.
fn on_button_pressed(
    mut pressed: EventReader<Pointer<Pressed>>,
    mut commands: Commands,
    mut egui: EguiContexts,
    mut visibility_query: Query<&mut Visibility>,
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

    for event in pressed.read() {
        if event.button == PointerButton::Primary {
            let session = session.as_mut();

            if let Some(target) = data.target_pos() {
                // Unhighlight the target tile
                if let Ok(mut visibility) =
                    visibility_query.get_mut(session.tiles.get(&target).unwrap().source_or_target())
                {
                    *visibility = Visibility::Hidden;
                }

                // If the target position is already occupied, capture it.
                capture_piece(
                    &mut commands,
                    &mut session.placed_pieces,
                    &mut session.players,
                    target,
                );

                let (transform, mut piece_pos) =
                    piece_query.get_mut(data.entities().root()).unwrap();

                // Animation
                {
                    let start = transform.translation;
                    let end = pos_translation(target, &rules).translation;

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
                piece_pos.0 = target;

                // Add record to the placed piece index at the current position
                session.placed_pieces.insert(
                    target,
                    PlacedPiece::new(data.model(), data.color(), target, data.entities().clone()),
                );

                // Update last action position
                session.last_action = Some(target);

                // Finish this turn
                next_phase.set(GamePhase::TurnEnd);
            } else {
                // Cancelled.
                session.placed_pieces.insert(
                    data.source_pos(),
                    PlacedPiece::new(
                        data.model(),
                        data.color(),
                        data.source_pos(),
                        data.entities().clone(),
                    ),
                );

                next_phase.set(GamePhase::Selecting);
            }

            // We only handle the first event
            break;
        }
    }
}

fn on_tile_enter(
    mut enter: EventReader<TileEnter>,
    child_query: Query<&ChildOf>,
    mut visibility_query: Query<&mut Visibility>,
    tile_query: Query<&Tile>,
    session: Res<GameSession>,
    mut data: ResMut<MovingPiece>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let Some(event) = enter.read().last() else {
        return;
    };

    let child = child_query.get(event.0).unwrap();
    let tile = tile_query.get(child.parent()).unwrap();

    apply_target(
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
    mut data: ResMut<MovingPiece>,
    session: Res<GameSession>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let Some(target) = data.target_pos() else {
        return;
    };

    for event in out.read() {
        let child = child_query.get(event.0).unwrap();
        let tile = tile_query.get(child.parent()).unwrap();

        if tile.pos() == target {
            apply_target(&mut visibility_query, &session.tiles, &mut data, None);
            break;
        }
    }
}

fn apply_target(
    visibility_query: &mut Query<&mut Visibility>,
    tiles: &TileIndex,
    data: &mut MovingPiece,
    new_target: Option<Pos>,
) {
    // Clear the previous to place position if any
    if let Some(old) = data.clear_target_pos() {
        let entities = tiles.get(&old).unwrap();

        // Unhighlight target
        if let Ok(mut visibility) = visibility_query.get_mut(entities.source_or_target()) {
            *visibility = Visibility::Hidden;
        }

        // Highlight placable
        if let Ok(mut visibility) = visibility_query.get_mut(entities.placeable()) {
            *visibility = Visibility::Visible;
        }
    }

    // Set the new target position if any
    if let Some(pos) = new_target {
        if data.set_target_pos(pos) {
            let entities = tiles.get(&pos).unwrap();

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
