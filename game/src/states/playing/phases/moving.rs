use crate::states::{
    game_setup::LoadedRules,
    playing::{
        TileEnter, despawn_placed_piece,
        phases::GamePhase,
        piece::{MovingPiece, PlacedPiece},
        pos_translation,
        session::{GameSession, piece_index::PieceEntities},
        tile::Tile,
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

#[derive(Resource)]
pub struct MovingData(pub PieceEntities);

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

fn on_enter() {
    // no-op
}

fn on_exit(mut commands: Commands) {
    commands.remove_resource::<MovingData>();
}

/// A system that triggered when the primary button is released.
fn on_button_released(
    mut released: EventReader<Pointer<Released>>,
    mut egui: EguiContexts,
    mut commands: Commands,
    moving_piece_query: Query<&MovingPiece>,
    mut visibility_query: Query<&mut Visibility>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    data: Res<MovingData>,
) {
    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    let session = session.as_mut();

    for event in released.read() {
        if event.button == PointerButton::Primary {
            if let Ok(piece) = moving_piece_query.get(data.0.root()) {
                // Unhighlight the moving piece
                if let Ok(mut visibility) = visibility_query.get_mut(data.0.highlight()) {
                    *visibility = Visibility::Hidden;
                }

                // Unhighlight the move initial tile
                if let Ok(mut visibility) = visibility_query.get_mut(
                    session
                        .tiles
                        .get(piece.initial_pos())
                        .unwrap()
                        .source_or_target(),
                ) {
                    *visibility = Visibility::Hidden;
                }

                // Unhighlight placeable tiles
                for pos in piece.placeable_tiles() {
                    if let Ok(mut visibility) =
                        visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
                    {
                        *visibility = Visibility::Hidden;
                    }
                }

                // If the target position is already occupied, remove the existing piece (i.e. capture it)
                despawn_placed_piece(
                    &mut commands,
                    &mut session.placed_pieces,
                    piece.current_pos(),
                );

                // Update component
                commands
                    .entity(data.0.root())
                    .insert(PlacedPiece::new(
                        piece.model(),
                        piece.color(),
                        piece.current_pos(),
                    ))
                    .remove::<MovingPiece>();

                // Add piece entities to the placed piece index at the current position
                session
                    .placed_pieces
                    .add(piece.current_pos(), data.0.clone());

                if piece.moved() {
                    // Update last action position
                    session.last_action = Some(piece.current_pos());

                    // Finish this turn
                    next_phase.set(GamePhase::TurnEnd);
                } else {
                    // Movement cancelled.
                    next_phase.set(GamePhase::Selecting);
                }
            }

            // We only handle the first release event
            break;
        }
    }
}

fn on_tile_enter(
    mut enter: EventReader<TileEnter>,
    child_query: Query<&ChildOf>,
    mut moving_piece_query: Query<(&mut Transform, &mut MovingPiece)>,
    tile_query: Query<&Tile>,
    rules: Res<LoadedRules>,
    data: Res<MovingData>,
) {
    let Some(event) = enter.read().last() else {
        return;
    };

    let Ok(child) = child_query.get(event.0) else {
        return;
    };

    let Ok(tile) = tile_query.get(child.parent()) else {
        return;
    };

    let Ok((mut transform, mut moving)) = moving_piece_query.get_mut(data.0.root()) else {
        return;
    };

    // Attempt to move the piece
    if !moving.try_move_to(tile.pos()) {
        return;
    }

    // Update transform
    *transform = pos_translation(tile.pos(), &rules.board);
}
