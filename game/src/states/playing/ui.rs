use crate::states::{
    game_setup::LoadedRules,
    playing::{
        PlayingMarker,
        phases::{GamePhase, placing::start_place_piece},
        session::GameSession,
    },
};
use bevy::prelude::*;
use rulery::piece::PieceModel;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PANEL_BG: Color = Color::srgb(0.118, 0.118, 0.118);
const TOP_BG: Color = Color::srgba(0.118, 0.118, 0.118, 0.75);

const BTN_NORMAL: Color = Color::srgba(0.15, 0.15, 0.15, 0.9);
const BTN_HOVERED: Color = Color::srgba(0.45, 0.45, 0.45, 0.9);
const BTN_PRESSED: Color = Color::srgb(0.85, 0.85, 0.85);
const BTN_DISABLED: Color = Color::srgba(0.1, 0.1, 0.1, 0.6);

const BORDER_NORMAL: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);
const BORDER_HOVERED: Color = Color::WHITE;

/// The text shown in the top status bar (set by the phase systems).
#[derive(Resource)]
pub struct TopPanelText(pub String);

/// Marker for the top status bar's text entity.
#[derive(Component)]
pub struct TopPanelLabel;

/// Which bottom-panel row a count label/button belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Row {
    Stock,
    Captured,
}

/// A clickable "In Stock" button for a given piece model.
#[derive(Component)]
pub struct StockButton {
    model: PieceModel,
}

/// A count label whose text is refreshed for the current player each frame.
#[derive(Component)]
pub struct PieceCount {
    row: Row,
    model: PieceModel,
}

/// Spawns the in-game HUD: a top status bar and a bottom panel with one
/// "In Stock" / "Captured" entry per piece model.
pub fn setup_ui(mut commands: Commands, rules: Res<LoadedRules>) {
    let models: Vec<PieceModel> = rules.pieces().map(|(model, _)| model).collect();

    // Top status bar.
    commands.spawn((
        PlayingMarker,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(0.0), Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(TOP_BG),
        children![(
            TopPanelLabel,
            Text::new(""),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    ));

    // Bottom panel.
    commands
        .spawn((
            PlayingMarker,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
        ))
        .with_children(|panel| {
            spawn_row(panel, "In Stock", Row::Stock, &models);
            spawn_row(panel, "Captured", Row::Captured, &models);
        });
}

/// Spawns one labelled row of per-model count entries.
fn spawn_row(
    panel: &mut ChildSpawnerCommands,
    header: &str,
    row: Row,
    models: &[PieceModel],
) {
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|line| {
            line.spawn((
                Text::new(header),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            for &model in models {
                let entry = Node {
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                };

                let count = (
                    PieceCount { row, model },
                    Text::new(""),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                );

                match row {
                    // "In Stock" entries are clickable buttons.
                    Row::Stock => {
                        line.spawn((
                            Button,
                            StockButton { model },
                            entry,
                            BorderColor::all(BORDER_NORMAL),
                            BackgroundColor(BTN_NORMAL),
                            children![count],
                        ));
                    }
                    // "Captured" entries are read-only.
                    Row::Captured => {
                        line.spawn((
                            entry,
                            BorderColor::all(BORDER_NORMAL),
                            BackgroundColor(BTN_DISABLED),
                            children![count],
                        ));
                    }
                }
            }
        });
}

/// Mirrors [`TopPanelText`] into the top status bar.
pub fn update_top_panel(text: Res<TopPanelText>, mut query: Query<&mut Text, With<TopPanelLabel>>) {
    if let Ok(mut label) = query.single_mut() {
        if label.0 != text.0 {
            label.0 = text.0.clone();
        }
    }
}

/// Refreshes every count label and stock-button appearance for the player whose
/// turn it currently is.
pub fn update_buttons(
    session: Res<GameSession>,
    phase: Res<State<GamePhase>>,
    mut labels: Query<(&PieceCount, &mut Text)>,
    mut buttons: Query<(&StockButton, &Interaction, &mut BackgroundColor, &mut BorderColor)>,
) {
    let (_, player) = session.players.get_by_index(session.turn.current_player());

    for (count, mut text) in &mut labels {
        let state = player.piece(count.model);
        let value = match count.row {
            Row::Stock => state.stock().to_string(),
            Row::Captured => state.captured().to_string(),
        };
        let text_value = format!("{}: {}", count.model, value);
        if text.0 != text_value {
            text.0 = text_value;
        }
    }

    let selecting = matches!(phase.get(), GamePhase::Selecting);

    for (button, interaction, mut bg, mut border) in &mut buttons {
        let enabled = selecting && !player.piece(button.model).stock().is_depleted();

        let (fill, edge) = if !enabled {
            (BTN_DISABLED, BORDER_NORMAL)
        } else {
            match interaction {
                Interaction::Pressed => (BTN_PRESSED, BORDER_HOVERED),
                Interaction::Hovered => (BTN_HOVERED, BORDER_HOVERED),
                Interaction::None => (BTN_NORMAL, BORDER_NORMAL),
            }
        };

        *bg = fill.into();
        border.set_all(edge);
    }
}

/// Starts placing a piece when its in-stock button is clicked.
pub fn handle_stock_button(
    mut commands: Commands,
    session: Res<GameSession>,
    phase: Res<State<GamePhase>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    buttons: Query<(&StockButton, &Interaction), Changed<Interaction>>,
) {
    // Pieces can only be placed from the selecting phase.
    if !matches!(phase.get(), GamePhase::Selecting) {
        return;
    }

    // Avoid a duplicate transition.
    if let NextState::Pending(_) = *next_phase {
        return;
    }

    let (color, player) = session.players.get_by_index(session.turn.current_player());

    for (button, interaction) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if player.piece(button.model).stock().is_depleted() {
            continue;
        }

        start_place_piece(&mut commands, &mut next_phase, button.model, color);
        break;
    }
}
