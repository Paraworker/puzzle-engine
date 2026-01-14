use crate::{GameError, states::AppState};
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct ErrorPlugin;

impl Plugin for ErrorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Error), on_enter)
            .add_systems(Update, update.run_if(in_state(AppState::Error)))
            .add_systems(OnExit(AppState::Error), on_exit);
    }
}

#[derive(Resource)]
pub struct CurrentError(pub GameError);

#[derive(Component)]
struct ErrorMarker;

fn on_enter(mut commands: Commands, error: Res<CurrentError>) {
    // camera
    commands.spawn((Camera2d, ErrorMarker));

    // ui
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ErrorMarker,
        ))
        .with_children(|parent| {
            parent.spawn(spacer());
            parent.spawn(label(error.0.to_string()));
            parent.spawn(spacer());
            parent.spawn(button());
            parent.spawn(spacer());
        });
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<ErrorMarker>>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<CurrentError>();
}

fn update(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    if let NextState::Pending(_) = *next_state {
        return;
    }

    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.set_all(Color::WHITE);

                next_state.set(AppState::Menu);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.set_all(Color::WHITE);
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(Color::BLACK);
            }
        }
    }
}

fn label(err_msg: impl Into<String>) -> impl Bundle + 'static {
    (
        Node {
            width: Val::Percent(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(err_msg),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

fn spacer() -> impl Bundle + 'static {
    Node {
        flex_grow: 1.0,
        ..default()
    }
}

fn button() -> impl Bundle + 'static {
    (
        Button,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(80.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new("Back to Menu"),
            TextFont {
                font_size: 28.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
