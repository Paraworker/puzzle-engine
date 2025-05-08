use bevy::{
    color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*,
    scene::InstanceId,
};

use crate::AppState;

const BOARD_PATH: &str = "models/Soccer.glb#Scene0";
const BOARD_NAME: &str = "Soccer";

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_scene)
            .add_systems(
                Update,
                (
                    draw_mesh_intersections.run_if(in_state(AppState::InGame)),
                    on_board_ready
                        .run_if(board_is_ready)
                        .run_if(in_state(AppState::InGame)),
                ),
            );
    }
}

#[derive(Component)]
struct Board;

#[derive(Resource)]
struct LoadingBoard(InstanceId, String);

#[derive(Resource)]
struct WhiteMaterial(Handle<StandardMaterial>);

#[derive(Resource)]
struct HoverMaterial(Handle<StandardMaterial>);

#[derive(Resource)]
struct PressedMaterial(Handle<StandardMaterial>);

fn setup_scene(
    mut commands: Commands,
    mut spawner: ResMut<SceneSpawner>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Board
    commands.insert_resource(LoadingBoard(
        spawner.spawn(asset_server.load(BOARD_PATH)),
        BOARD_NAME.to_string(),
    ));

    // Materials
    commands.insert_resource(WhiteMaterial(materials.add(Color::WHITE)));
    commands.insert_resource(HoverMaterial(materials.add(Color::from(CYAN_300))));
    commands.insert_resource(PressedMaterial(materials.add(Color::from(YELLOW_300))));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E: 'static>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.target()) {
            material.0 = new_material.clone();
        }
    }
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

/// An observer to rotate an entity when it is dragged.
fn rotate_on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target()) {
        transform.rotate_y(drag.delta.x * 0.02);
        transform.rotate_x(drag.delta.y * 0.02);
    }
}

fn board_is_ready(spawner: Res<SceneSpawner>, board: Option<Res<LoadingBoard>>) -> bool {
    if let Some(board) = board {
        spawner.instance_is_ready(board.0)
    } else {
        false
    }
}

fn on_board_ready(
    mut commands: Commands,
    spawner: Res<SceneSpawner>,
    board: Res<LoadingBoard>,
    name_query: Query<&Name>,
    mesh_query: Query<(), With<Mesh3d>>,
    white_mat: Res<WhiteMaterial>,
    hover_mat: Res<HoverMaterial>,
    pressed_mat: Res<PressedMaterial>,
) {
    for entity in spawner.iter_instance_entities(board.0) {
        // Root
        if let Ok(name) = name_query.get(entity) {
            if name.as_str() == board.1 {
                commands
                    .entity(entity)
                    .insert(Board)
                    .observe(rotate_on_drag);
            }
        }

        // Mesh
        if mesh_query.contains(entity) {
            commands
                .entity(entity)
                .observe(update_material_on::<Pointer<Over>>(hover_mat.0.clone()))
                .observe(update_material_on::<Pointer<Out>>(white_mat.0.clone()))
                .observe(update_material_on::<Pointer<Pressed>>(
                    pressed_mat.0.clone(),
                ))
                .observe(update_material_on::<Pointer<Released>>(hover_mat.0.clone()));
        }
    }

    commands.remove_resource::<LoadingBoard>();
}
