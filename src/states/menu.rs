use crate::states::GameState;
use bevy::prelude::*;

const BUTTON_NORMAL: Color = Color::srgba(0.15, 0.15, 0.15, 0.6);
const BUTTON_HOVERED: Color = Color::srgba(0.55, 0.55, 0.55, 0.6);
const BUTTON_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);

const BUTTON_BORDER_NORMAL: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);
const BUTTON_BORDER_HOVERED: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BUTTON_BORDER_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);

const BG_PATH: &str = "textures/menu_bg.png";

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), on_enter)
            .add_systems(Update, update.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), on_exit);
    }
}

#[derive(Component)]
struct MenuMarker;

fn update(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED.into();
                border_color.0 = BUTTON_BORDER_PRESSED;

                next_state.set(GameState::GameSetup);
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
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuMarker,
        ))
        .with_children(|parent| {
            // button
            parent.spawn(button("New Game"));

            // background image
            parent.spawn(background(asset_server.load(BG_PATH)));
        });
}

fn on_exit(mut commands: Commands, entities: Query<Entity, With<MenuMarker>>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}

fn button(text: impl Into<String>) -> impl Bundle + 'static {
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
            )]
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
