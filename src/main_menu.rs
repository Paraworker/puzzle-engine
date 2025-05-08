use bevy::prelude::*;

use crate::AppState;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(Update, update.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup);
    }
}

#[derive(Component)]
struct MainMenuCamera;

#[derive(Component)]
struct MainMenuUI;

fn update(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;

                next_state.set(AppState::InGame);
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

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn((Camera2d, MainMenuCamera));
    commands.spawn(button());
}

fn cleanup(
    mut commands: Commands,
    cameras: Query<Entity, With<MainMenuCamera>>,
    uis: Query<Entity, With<MainMenuUI>>,
) {
    for camera in cameras {
        commands.entity(camera).despawn();
    }

    for ui in uis {
        commands.entity(ui).despawn();
    }
}

fn button() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        MainMenuUI,
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(12.0)),
            BackgroundColor(NORMAL_BUTTON),
            MainMenuUI,
            children![(
                Text::new("New Game"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
                MainMenuUI,
            )]
        )],
    )
}
