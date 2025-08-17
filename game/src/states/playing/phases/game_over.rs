use crate::states::{
    no_pending_transition,
    playing::{TopPanelText, camera::PlayingCamera, phases::GamePhase, session::GameSession},
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::EguiContexts;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::GameOver), on_enter)
            .add_systems(
                Update,
                (on_mouse_wheel, on_pointer_drag)
                    .run_if(in_state(GamePhase::GameOver).and(no_pending_transition::<GamePhase>)),
            )
            .add_systems(OnExit(GamePhase::GameOver), on_exit);
    }
}

fn on_enter(session: Res<GameSession>, mut top_panel_text: ResMut<TopPanelText>) {
    top_panel_text.0 = format!("Game Over: {}", session.players.player_states_message());
}

fn on_exit() {
    // no-op
}

/// A system that triggered on the mouse wheel event.
fn on_mouse_wheel(
    mut scroll_evr: EventReader<MouseWheel>,
    mut egui: EguiContexts,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
) {
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
) {
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
