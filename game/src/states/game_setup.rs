use crate::{
    settings::Settings,
    states::{AppState, error::CurrentError, no_pending_transition},
};
use bevy::prelude::*;
use rule_engine::GameRules;
use std::ops::Deref;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameSetup), on_enter)
            .add_systems(
                Update,
                update.run_if(in_state(AppState::GameSetup).and(no_pending_transition::<AppState>)),
            )
            .add_systems(OnExit(AppState::GameSetup), on_exit);
    }
}

#[derive(Resource)]
pub struct LoadedRules(GameRules);

impl Deref for LoadedRules {
    type Target = GameRules;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
struct GameSetupMarker;

fn on_enter(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    settings: Res<Settings>,
) {
    // Load rules from settings
    let rules = match GameRules::load(settings.rules_path.as_path()) {
        Ok(rules) => rules,
        Err(err) => {
            commands.insert_resource(CurrentError(err.into()));
            next_state.set(AppState::Error);
            return;
        }
    };

    // camera
    commands.spawn((Camera2d, GameSetupMarker));

    // ui
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GameSetupMarker,
        ))
        .with_children(|parent| {
            parent.spawn(spacer());
            parent.spawn(label(format!("Loaded Rules: {}", rules.name), 50.0));
            parent.spawn(spacer());
            parent.spawn(button("Ready!"));
            parent.spawn(spacer());
        });

    commands.insert_resource(LoadedRules(rules));
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<GameSetupMarker>>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}

fn update(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;

                next_state.set(AppState::Loading);
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

fn label(text: impl Into<String>, font_size: f32) -> impl Bundle + 'static {
    (
        Node::default(),
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    )
}

fn spacer() -> impl Bundle + 'static {
    Node {
        flex_grow: 1.0,
        ..default()
    }
}

fn button(text: impl Into<String>) -> impl Bundle + 'static {
    (
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::all(Val::Px(12.0)),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            TextFont {
                font_size: 28.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
