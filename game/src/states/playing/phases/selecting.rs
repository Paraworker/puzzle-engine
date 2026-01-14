use crate::states::playing::{
    PiecePress, TopPanelText,
    camera::PlayingCamera,
    phases::{GamePhase, moving::start_move_piece},
    piece::PiecePos,
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
                (on_mouse_wheel, on_pointer_drag, on_piece_press)
                    .run_if(in_state(GamePhase::Selecting)),
            )
            .add_systems(OnExit(GamePhase::Selecting), on_exit);
    }
}

fn on_enter(
    mut piece_press: Option<ResMut<Messages<PiecePress>>>,
    mut drag: Option<ResMut<Messages<Pointer<Drag>>>>,
    mut wheel: Option<ResMut<Messages<Pointer<MouseWheel>>>>,
    session: Res<GameSession>,
    mut top_panel_text: ResMut<TopPanelText>,
) {
    // Clear messages
    // In case the old messages are still in the queue
    if let Some(piece_press) = &mut piece_press {
        piece_press.clear();
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

/// A system that triggered on the mouse wheel message.
fn on_mouse_wheel(
    mut scroll_msgs: MessageReader<MouseWheel>,
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

    for msg in scroll_msgs.read() {
        for (mut transform, mut camera) in &mut query {
            camera.zoom(msg.y);

            // Update transform
            *transform = camera.transform();
        }
    }
}

/// A system that triggered when the pointer is dragged.
fn on_pointer_drag(
    mut drag_msgs: MessageReader<Pointer<Drag>>,
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

    for drag in drag_msgs.read() {
        for (mut transform, mut cam) in camera_query.iter_mut() {
            cam.drag(drag.delta.x, drag.delta.y);

            // Update transform
            *transform = cam.transform();
        }
    }
}

fn on_piece_press(
    mut press: MessageReader<PiecePress>,
    mut commands: Commands,
    child_query: Query<&ChildOf>,
    piece_query: Query<&PiecePos>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let Some(msg) = press.read().last() else {
        return;
    };

    // Skip if the pointer message is not primary click
    if msg.1 != PointerButton::Primary {
        return;
    }

    let session = session.as_mut();

    let child = child_query.get(msg.0).unwrap();
    let pos = piece_query.get(child.parent()).unwrap();

    start_move_piece(
        &mut commands,
        &mut session.placed_pieces,
        &session.players,
        &session.turn,
        &mut next_phase,
        pos.0,
    );
}
