use crate::states::{
    game_setup::LoadedRules,
    playing::{
        PlayingEvent, TopPanelText,
        camera::PlayingCamera,
        phases::{GamePhase, moving::MovingData},
        piece::{MovingPiece, PlacedPiece},
        session::GameSession,
        tile::Tile,
    },
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::EguiContexts;

pub struct SelectingPlugin;

impl Plugin for SelectingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Selecting), on_enter)
            .add_systems(
                Update,
                (on_mouse_wheel, on_pointer_drag, on_playing_event)
                    .run_if(in_state(GamePhase::Selecting)),
            )
            .add_systems(OnExit(GamePhase::Selecting), on_exit);
    }
}

fn on_enter(session: Res<GameSession>, mut top_panel_text: ResMut<TopPanelText>) {
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

fn on_playing_event(
    mut events: EventReader<PlayingEvent>,
    mut commands: Commands,
    child_query: Query<&ChildOf>,
    placed_piece_query: Query<&PlacedPiece>,
    tile_query: Query<&Tile>,
    mut visibility_query: Query<&mut Visibility>,
    mut session: ResMut<GameSession>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    rules: Res<LoadedRules>,
) {
    for event in events.read() {
        match event {
            PlayingEvent::PiecePressed(entity, button) => on_piece_pressed(
                *entity,
                *button,
                &mut commands,
                child_query,
                placed_piece_query,
                tile_query,
                &mut visibility_query,
                &mut session,
                &mut next_phase,
                &rules,
            ),
            _ => {}
        }
    }
}

fn on_piece_pressed(
    entity: Entity,
    button: PointerButton,
    commands: &mut Commands,
    child_query: Query<&ChildOf>,
    placed_piece_query: Query<&PlacedPiece>,
    tile_query: Query<&Tile>,
    visibility_query: &mut Query<&mut Visibility>,
    session: &mut GameSession,
    next_phase: &mut NextState<GamePhase>,
    rules: &LoadedRules,
) {
    // Skip if the pointer event is not primary click
    if button != PointerButton::Primary {
        return;
    }

    // Try to fetch the child component of the pressed entity
    let Ok(child) = child_query.get(entity) else {
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

    // Try to create a move context from the selected piece
    let Ok(moving) = MovingPiece::new(
        placed.model(),
        placed.color(),
        placed.pos(),
        session,
        placed_piece_query,
        rules.pieces.get_by_model(placed.model()).movement(),
        tile_query,
    ) else {
        return;
    };

    // Take the piece entities from the placed piece index
    let Some(entities) = session.placed_pieces.remove(placed.pos()) else {
        return;
    };

    // Highlight visual elements (non-fatal)
    {
        // Highlight the moving piece
        if let Ok(mut visibility) = visibility_query.get_mut(entities.highlight()) {
            *visibility = Visibility::Visible;
        }

        // Highlight move initial tile
        if let Ok(mut visibility) =
            visibility_query.get_mut(session.tiles.get(placed.pos()).unwrap().source_or_target())
        {
            *visibility = Visibility::Visible;
        }

        // Highlight placeable tiles
        for pos in moving.placeable_tiles() {
            if let Ok(mut visibility) =
                visibility_query.get_mut(session.tiles.get(pos).unwrap().placeable())
            {
                *visibility = Visibility::Visible;
            }
        }
    }

    // Apply component state change
    commands
        .entity(entities.root())
        .insert(moving)
        .remove::<PlacedPiece>();

    // Enter moving state
    commands.insert_resource(MovingData(entities));
    next_phase.set(GamePhase::Moving);
}
