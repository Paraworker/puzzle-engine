use crate::states::{AppState, no_pending_transition};
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), on_enter)
            .add_systems(
                Update,
                update.run_if(in_state(AppState::Loading).and(no_pending_transition::<AppState>)),
            )
            .add_systems(OnExit(AppState::Loading), on_exit);
    }
}

fn on_enter() {
    // no-op
}

fn update(mut next_state: ResMut<NextState<AppState>>) {
    // Nothing to do now

    // Switch to the `Playing` state.
    next_state.set(AppState::Playing);
}

fn on_exit() {
    // no-op
}
