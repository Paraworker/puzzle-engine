use crate::{
    config::{BoardMeta, BoardName, Config},
    states::{GameState, loading::load_board},
};
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameSetup), on_enter)
            .add_systems(Update, update.run_if(in_state(GameState::GameSetup)))
            .add_systems(OnExit(GameState::GameSetup), on_exit);
    }
}

#[derive(Component)]
struct GameSetupMarker;

fn on_enter(mut commands: Commands, config: Res<Config>) {
    // camera
    commands.spawn((Camera2d, GameSetupMarker));

    // UI
    // FIXME: Only spawn one button for now
    if let Some((name, meta)) = config.boards().next() {
        commands.spawn((board_button(name.clone(), meta), GameSetupMarker));
    }
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<GameSetupMarker>>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}

fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &BoardName,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, board_name) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;

                load_board(
                    &mut commands,
                    &asset_server,
                    &mut next_state,
                    board_name.clone(),
                );
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn board_button(name: BoardName, meta: &BoardMeta) -> impl Bundle + 'static {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(360.0),
                height: Val::Px(48.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(12.0)),
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new(meta.display_name()),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )],
            name.clone(),
        )],
    )
}
