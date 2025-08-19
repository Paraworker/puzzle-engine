use crate::states::playing::{
    PiecePressed, TopPanelText,
    camera::PlayingCamera,
    phases::GamePhase,
    piece::{MovingPiece, PiecePos},
    session::GameSession,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::EguiContexts;
use std::collections::hash_map::Entry;

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
    mut piece_pressed: Option<ResMut<Events<PiecePressed>>>,
    mut drag: Option<ResMut<Events<Pointer<Drag>>>>,
    mut wheel: Option<ResMut<Events<Pointer<MouseWheel>>>>,
    session: Res<GameSession>,
    mut top_panel_text: ResMut<TopPanelText>,
) {
    // Clear events
    // In case the old events are still in the queue
    if let Some(piece_pressed) = &mut piece_pressed {
        piece_pressed.clear();
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
    piece_query: Query<&PiecePos>,
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

    let session = session.as_mut();

    let child = child_query.get(event.0).unwrap();
    let pos = piece_query.get(child.parent()).unwrap();

    let Entry::Occupied(entry) = session.placed_pieces.entry(pos.0) else {
        panic!("No placed piece at position: {:?}", pos.0);
    };

    // If the piece color does not match the current player's color, do nothing
    if session
        .players
        .get_by_index(session.turn.current_player())
        .0
        != entry.get().color()
    {
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
