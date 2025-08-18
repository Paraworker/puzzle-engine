use crate::states::{
    game_setup::LoadedRules,
    playing::{
        TileEnter, capture_piece,
        phases::GamePhase,
        piece::{MovingPiece, PieceEntities, PlacedPiece},
        pos_translation,
        session::GameSession,
        tile::Tile,
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};
use std::time::Duration;

#[derive(Resource)]
pub struct MovingEntities(pub PieceEntities);

pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Moving), on_enter)
            .add_systems(
                Update,
                (on_button_released, on_tile_enter).run_if(in_state(GamePhase::Moving)),
            )
            .add_systems(OnExit(GamePhase::Moving), on_exit);
    }
}

fn on_enter(
    mut visibility_query: Query<&mut Visibility>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    placed_piece_query: Query<&PlacedPiece>,
    mut moving_piece_query: Query<&mut MovingPiece>,
    session: Res<GameSession>,
    data: Res<MovingEntities>,
) {
    let mut moving = moving_piece_query.get_mut(data.0.root()).unwrap();
    let rules = rules.get_piece(moving.model()).unwrap();

    // Collect movable tiles
    moving
        .collect_movable(&session, placed_piece_query, tile_query, rules)
        .unwrap();

    // Highlight the moving piece
    if let Ok(mut visibility) = visibility_query.get_mut(data.0.highlight()) {
        *visibility = Visibility::Visible;
    }

    // Highlight move initial tile
    if let Ok(mut visibility) = visibility_query.get_mut(
        session
            .tiles
            .get(&moving.initial_pos())
            .unwrap()
            .source_or_target(),
    ) {
        *visibility = Visibility::Visible;
    }

    // Highlight movable tiles
    for pos in moving.movable_tiles() {
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
    placed_piece_query: Query<&PlacedPiece>,
    moving_piece_query: Query<&MovingPiece>,
    mut session: ResMut<GameSession>,
    data: Res<MovingEntities>,
) {
    let moving = moving_piece_query.get(data.0.root()).unwrap();

    // Unhighlight the moving piece
    if let Ok(mut visibility) = visibility_query.get_mut(data.0.highlight()) {
        *visibility = Visibility::Hidden;
    }

    let session = session.as_mut();

    // Unhighlight the move initial tile
    if let Ok(mut visibility) = visibility_query.get_mut(
        session
            .tiles
            .get(&moving.initial_pos())
            .unwrap()
            .source_or_target(),
    ) {
        *visibility = Visibility::Hidden;
    }

    // Unhighlight movable tiles
    for pos in moving.movable_tiles() {
        if let Ok(mut visibility) =
            visibility_query.get_mut(session.tiles.get(&pos).unwrap().placeable())
        {
            *visibility = Visibility::Hidden;
        }
    }

    // If the target position is already occupied, remove the existing piece (i.e. capture it)
    capture_piece(
        &mut commands,
        placed_piece_query,
        &mut session.placed_pieces,
        &mut session.players,
        moving.current_pos(),
    );

    // Update component
    commands
        .entity(data.0.root())
        .insert(PlacedPiece::new(
            moving.model(),
            moving.color(),
            moving.current_pos(),
        ))
        .remove::<MovingPiece>();

    // Add piece entities to the placed piece index at the current position
    session
        .placed_pieces
        .insert(moving.current_pos(), data.0.clone());

    commands.remove_resource::<MovingEntities>();
}

/// A system that triggered when the primary button is released.
fn on_button_released(
    mut released: EventReader<Pointer<Released>>,
    mut egui: EguiContexts,
    moving_piece_query: Query<&MovingPiece>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    data: Res<MovingEntities>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    for event in released.read() {
        if event.button == PointerButton::Primary {
            let moving = moving_piece_query.get(data.0.root()).unwrap();

            if moving.moved() {
                // Update last action position
                session.last_action = Some(moving.current_pos());

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
    mut commands: Commands,
    child_query: Query<&ChildOf>,
    mut moving_piece_query: Query<(&Transform, &mut MovingPiece)>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    data: Res<MovingEntities>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let Some(event) = enter.read().last() else {
        return;
    };

    let Ok(child) = child_query.get(event.0) else {
        return;
    };

    let Ok(tile) = tile_query.get(child.parent()) else {
        return;
    };

    let Ok((transform, mut moving)) = moving_piece_query.get_mut(data.0.root()) else {
        return;
    };

    // Attempt to update the current pos
    if !moving.set_current_pos(tile.pos()) {
        return;
    }

    let start = transform.translation;
    let end = pos_translation(tile.pos(), &rules).translation;

    // Build a tween to animate the piece movement
    let tween = Tween::new(
        EaseFunction::CubicInOut,
        Duration::from_millis(200),
        TransformPositionLens { start, end },
    );

    commands.entity(data.0.root()).insert(Animator::new(tween));
}
