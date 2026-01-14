use crate::{
    assets::GameAssets,
    states::playing::{
        PlayingMarker, TileEnter, TileOut, TileRelease,
        session::TileIndex,
        tile::{Tile, TileEntities},
    },
};
use bevy::prelude::*;
use rulery::{CheckedGameRules, pos::Pos};

/// Spawns the board.
pub fn spawn_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    rules: &CheckedGameRules,
) -> (Entity, TileIndex) {
    fn on_tile_enter(on_over: On<Pointer<Over>>, mut msg: MessageWriter<TileEnter>) {
        msg.write(TileEnter(on_over.event_target()));
    }

    fn on_tile_out(on_out: On<Pointer<Out>>, mut msg: MessageWriter<TileOut>) {
        msg.write(TileOut(on_out.event_target()));
    }

    fn on_tile_release(on_release: On<Pointer<Release>>, mut msg: MessageWriter<TileRelease>) {
        msg.write(TileRelease(on_release.event_target(), on_release.button));
    }

    let mut tiles = TileIndex::new();

    // Spawn board root
    let board_root = commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            PlayingMarker,
        ))
        .id();

    // Spawn tiles transform
    let tiles_transform = commands
        .spawn((
            Transform::from_translation(Vec3::new(
                0.0,
                -CheckedGameRules::tile_height() / 2.0,
                0.0,
            )),
            GlobalTransform::default(),
        ))
        .id();

    commands.entity(board_root).add_child(tiles_transform);

    let tile_mesh = meshes.add(Cuboid::new(
        CheckedGameRules::tile_size(),
        CheckedGameRules::tile_height(),
        CheckedGameRules::tile_size(),
    ));

    // Spawn tiles
    for col in 0..rules.board_cols() {
        for row in 0..rules.board_rows() {
            // Tile position
            let pos = Pos::new(row, col);

            // Choose color based on position
            let base_color = if (col + row) % 2 == 0 {
                assets.materials.common.tile_white.clone()
            } else {
                assets.materials.common.tile_black.clone()
            };

            let tile_root = commands
                .spawn((
                    pos_translation(pos, rules),
                    GlobalTransform::default(),
                    Tile::new(pos),
                ))
                .id();

            let base_mesh = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(base_color),
                    Transform::default(),
                    GlobalTransform::default(),
                ))
                .observe(on_tile_enter)
                .observe(on_tile_out)
                .observe(on_tile_release)
                .id();

            let source_or_target = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Pickable::IGNORE,
                    Visibility::Hidden,
                ))
                .id();

            let placeable = commands
                .spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(assets.materials.common.highlight_placeable.clone()),
                    Transform::default(),
                    GlobalTransform::default(),
                    Pickable::IGNORE,
                    Visibility::Hidden,
                ))
                .id();

            commands.entity(tiles_transform).add_child(tile_root);

            commands
                .entity(tile_root)
                .add_children(&[base_mesh, source_or_target, placeable]);

            // Add to tile index
            tiles.insert(
                pos,
                TileEntities::new(tile_root, base_mesh, source_or_target, placeable),
            );
        }
    }

    (board_root, tiles)
}

/// Converts a logical board position to board space translation.
///
/// (0, 0) is the bottom-left tile on the board.
pub fn pos_translation(pos: Pos, rules: &CheckedGameRules) -> Transform {
    const fn half_len(cols_or_rows: i64) -> f32 {
        (cols_or_rows as f32 - 1.0) * CheckedGameRules::tile_size() / 2.0
    }

    Transform::from_translation(Vec3::new(
        pos.col() as f32 * CheckedGameRules::tile_size() - half_len(rules.board_cols()),
        0.0,
        half_len(rules.board_rows()) - pos.row() as f32 * CheckedGameRules::tile_size(),
    ))
}
