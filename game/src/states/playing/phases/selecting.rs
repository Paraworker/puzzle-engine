use crate::states::playing::{
    PiecePressed, TopPanelText,
    camera::PlayingCamera,
    phases::{GamePhase, moving::MovingEntities},
    piece::{MovingPiece, PlacedPiece},
    session::GameSession,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::EguiContexts;

pub struct SelectingPlugin;

impl Plugin for SelectingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Selecting), on_enter)
            .add_systems(
                Update,
                (on_mouse_wheel, on_pointer_drag, on_piece_pressed)
                    .run_if(in_state(GamePhase::Selecting)),
            )
            .add_systems(OnExit(GamePhase::Selecting), on_exit);
    }
}

fn on_enter(
    mut pressed: Option<ResMut<Events<Pointer<Pressed>>>>,
    mut drag: Option<ResMut<Events<Pointer<Drag>>>>,
    mut wheel: Option<ResMut<Events<Pointer<MouseWheel>>>>,
    session: Res<GameSession>,
    mut top_panel_text: ResMut<TopPanelText>,
) {
    // Clear events
    // In case the old events are still in the queue
    if let Some(pressed) = &mut pressed {
        pressed.clear();
    }

    if let Some(drag) = &mut drag {
        drag.clear();
    }

    if let Some(wheel) = &mut wheel {
        wheel.clear();
    }

    top_panel_text.0 = session.turn.turn_message(&session.players);
}

fn on_exit() {
    // no-op
}

/// A system that triggered on the mouse wheel event.
fn on_mouse_wheel(
    mut scroll_evr: EventReader<MouseWheel>,
    mut egui: EguiContexts,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    for ev in scroll_evr.read() {
        for (mut transform, mut camera) in &mut query {
            camera.zoom(ev.y);

            // Update transform
            *transform = camera.transform();
        }
    }
}

/// A system that triggered when the pointer is dragged.
fn on_pointer_drag(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut egui: EguiContexts,
    mut camera_query: Query<(&mut Transform, &mut PlayingCamera)>,
    next_phase: Res<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    if egui.ctx_mut().unwrap().wants_pointer_input() {
        return;
    }

    for drag in drag_events.read() {
        for (mut transform, mut cam) in camera_query.iter_mut() {
            cam.drag(drag.delta.x, drag.delta.y);

            // Update transform
            *transform = cam.transform();
        }
    }
}

fn on_piece_pressed(
    mut pressed: EventReader<PiecePressed>,
    mut commands: Commands,
    child_query: Query<&ChildOf>,
    placed_piece_query: Query<&PlacedPiece>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let Some(event) = pressed.read().last() else {
        return;
    };

    // Skip if the pointer event is not primary click
    if event.1 != PointerButton::Primary {
        return;
    }

    // Try to fetch the child component of the pressed entity
    let Ok(child) = child_query.get(event.0) else {
        return;
    };

    // Try to fetch the selected placed piece
    let Ok(placed) = placed_piece_query.get(child.parent()) else {
        return;
    };

    // If the piece color does not match the current player's color, do nothing
    if session
        .players
        .get_by_index(session.turn.current_player())
        .0
        != placed.color()
    {
        return;
    }

    // Take the piece entities from the placed piece index
    let Some(entities) = session.placed_pieces.remove(&placed.pos()) else {
        return;
    };

    // Apply component state change
    commands
        .entity(entities.root())
        .insert(MovingPiece::new(
            placed.model(),
            placed.color(),
            placed.pos(),
        ))
        .remove::<PlacedPiece>();

    // Enter moving state
    commands.insert_resource(MovingEntities(entities));
    next_phase.set(GamePhase::Moving);
}
