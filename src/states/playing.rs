use crate::states::{
    ActiveScene, GameState, TileHoverMaterial, TileNormalMaterial, TilePressedMaterial,
};
use bevy::{
    color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*,
    scene::SceneInstanceReady,
};

#[derive(Component)]
struct PlayingMarker;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                draw_mesh_intersections.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup(mut commands: Commands, scene: Res<ActiveScene>) {
    // Scene
    commands
        .spawn((SceneRoot(scene.0.clone()), PlayingMarker))
        .observe(on_scene_spawned);

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
        PlayingMarker,
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        PlayingMarker,
    ));
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<PlayingMarker>>) {
    // Delete entities
    for entity in entities {
        commands.entity(entity).despawn();
    }

    // Delete related resources
    commands.remove_resource::<ActiveScene>();
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) + 'static {
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

/// An observer called when a scene is spawned.
fn on_scene_spawned(
    trigger: Trigger<SceneInstanceReady>,
    meshes: Query<&Mesh3d>,
    children: Query<&Children>,
    normal_mat: Res<TileNormalMaterial>,
    hover_mat: Res<TileHoverMaterial>,
    pressed_mat: Res<TilePressedMaterial>,
    mut commands: Commands,
) {
    // Root
    commands.entity(trigger.target()).observe(rotate_on_drag);

    // Mesh
    for child in children.iter_descendants(trigger.target()) {
        if meshes.contains(child) {
            commands
                .entity(child)
                .observe(update_material_on::<Pointer<Over>>(hover_mat.0.clone()))
                .observe(update_material_on::<Pointer<Out>>(normal_mat.0.clone()))
                .observe(update_material_on::<Pointer<Pressed>>(
                    pressed_mat.0.clone(),
                ))
                .observe(update_material_on::<Pointer<Released>>(hover_mat.0.clone()));
        }
    }
}
