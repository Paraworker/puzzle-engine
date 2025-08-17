use crate::states::AppState;
use bevy::prelude::*;

const BUTTON_NORMAL: Color = Color::srgba(0.15, 0.15, 0.15, 0.6);
const BUTTON_HOVERED: Color = Color::srgba(0.55, 0.55, 0.55, 0.6);
const BUTTON_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);

const BUTTON_BORDER_NORMAL: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);
const BUTTON_BORDER_HOVERED: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BUTTON_BORDER_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);

const BG_PATH: &str = "textures/menu_bg.png";
const LOGO_PATH: &str = "textures/logo.png";

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), on_enter)
            .add_systems(Update, update.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), on_exit);
    }
}

#[derive(Component)]
struct MenuMarker;

fn update(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let NextState::Pending(_) = *next_state {
        return;
    }

    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED.into();
                border_color.0 = BUTTON_BORDER_PRESSED;

                next_state.set(AppState::GameSetup);
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVERED.into();
                border_color.0 = BUTTON_BORDER_HOVERED;
            }
            Interaction::None => {
                *color = BUTTON_NORMAL.into();
                border_color.0 = BUTTON_BORDER_NORMAL;
            }
        }
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn((Camera2d, MenuMarker));

    // ui
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MenuMarker,
        ))
        .with_children(|parent| {
            // logo and button
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(spacer());
                    parent.spawn(logo(asset_server.load(LOGO_PATH)));
                    parent.spawn(spacer());
                    parent.spawn(button("New Game"));
                    parent.spawn(spacer());
                });

            // background
            parent.spawn(background(asset_server.load(BG_PATH)));
        });
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<MenuMarker>>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}

fn logo(image: Handle<Image>) -> impl Bundle + 'static {
    (
        Node {
            width: Val::Percent(40.0),
            height: Val::Auto,
            ..default()
        },
        ImageNode { image, ..default() },
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
        BorderColor(BUTTON_BORDER_NORMAL),
        BorderRadius::all(Val::Px(12.0)),
        BackgroundColor(BUTTON_NORMAL),
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

fn background(image: Handle<Image>) -> impl Bundle + 'static {
    (
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            ..default()
        },
        ImageNode { image, ..default() },
        GlobalZIndex(-1),
    )
}
