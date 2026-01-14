use crate::states::playing::{
    TopPanelText, camera::PlayingCamera, phases::GamePhase, session::GameSession,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::EguiContexts;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::GameOver), on_enter)
            .add_systems(
                Update,
                (on_mouse_wheel, on_pointer_drag).run_if(in_state(GamePhase::GameOver)),
            )
            .add_systems(OnExit(GamePhase::GameOver), on_exit);
    }
}

fn on_enter(
    mut drag: Option<ResMut<Messages<Pointer<Drag>>>>,
    mut wheel: Option<ResMut<Messages<Pointer<MouseWheel>>>>,
    session: Res<GameSession>,
    mut top_panel_text: ResMut<TopPanelText>,
) {
    // Clear messages
    // In case the old messages are still in the queue
    if let Some(drag) = &mut drag {
        drag.clear();
    }

    if let Some(wheel) = &mut wheel {
        wheel.clear();
    }

    top_panel_text.0 = format!("Game Over: {}", session.players.player_states_message());
}

fn on_exit() {
    // no-op
}

/// A system that triggered on the mouse wheel event.
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
