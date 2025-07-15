use crate::states::GameState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), on_enter)
            .add_systems(Update, update.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), on_exit);
    }
}

fn on_enter() {
    // no-op
}

fn update(mut next_state: ResMut<NextState<GameState>>) {
    // Nothing to do now

    // Switch to the `Playing` state.
    next_state.set(GameState::Playing);
}

fn on_exit() {
    // no-op
}
