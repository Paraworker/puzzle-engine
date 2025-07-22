use crate::{
    assets::GameAssets,
    rules::{GameRules, board::BoardGeometry},
    states::playing::{
        PlayingMarker,
        piece::{Piece, place_piece},
        session::{GameSession, PlayState},
    },
};
use bevy::prelude::*;

/// Tile position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos(usize, usize);

impl Pos {
    /// Creates a new `Pos`.
    pub const fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    /// Returns the row index of the tile position.
    pub const fn row(&self) -> usize {
        self.0
    }

    /// Returns the column index of the tile position.
    pub const fn col(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone, Component)]
pub struct Tile {
    pos: Pos,
    color: Handle<StandardMaterial>,
}

impl Tile {
    pub const fn pos(&self) -> Pos {
        self.pos
    }

    pub fn color(&self) -> Handle<StandardMaterial> {
        self.color.clone()
    }
}

pub fn spawn_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    board: &BoardGeometry,
) {
    fn tile_transform(pos: Pos, board: &BoardGeometry) -> Transform {
        Transform::from_translation(Vec3::new(
            pos.col() as f32 * board.tile_size() - board.half_width_col(),
            -board.height() / 2.0,
            pos.row() as f32 * board.tile_size() - board.half_width_row(),
        ))
    }

    fn on_tile_hovered(
        trigger: Trigger<Pointer<Over>>,
        mut dragged_query: Query<(&mut Transform, &mut Piece)>,
        mut tile_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
        assets: Res<GameAssets>,
        rules: Res<GameRules>,
        session: Res<GameSession>,
    ) {
        match &session.state {
            PlayState::Navigating => {
                if let Ok((mut material, _)) = tile_query.get_mut(trigger.target()) {
                    material.0 = assets.materials.common.tile_hover.clone();
                }
            }
            PlayState::Dragging(dragged) => {
                if let Ok((_, tile)) = tile_query.get_mut(trigger.target()) {
                    if let Ok((mut transform, mut piece)) = dragged_query.get_mut(dragged.entity())
                    {
                        // Update translation
                        transform.translation = place_piece(tile.pos(), rules.board_geometry());

                        // Update position
                        piece.set_pos(tile.pos());
                    }
                }
            }
        }
    }

    fn on_tile_out(
        trigger: Trigger<Pointer<Out>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Tile)>,
        session: Res<GameSession>,
    ) {
        if let PlayState::Navigating = session.state {
            if let Ok((mut material, tile)) = query.get_mut(trigger.target()) {
                // Restore the tile's original color
                material.0 = tile.color();
            }
        }
    }

    for col in 0..board.cols() {
        for row in 0..board.rows() {
            // Tile position
            let pos = Pos::new(row, col);

            // Choose color based on position
            let color = if (col + row) % 2 == 0 {
                assets.materials.common.tile_white.clone()
            } else {
                assets.materials.common.tile_black.clone()
            };

            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        board.tile_size(),
                        board.height(),
                        board.tile_size(),
                    ))),
                    MeshMaterial3d(color.clone()),
                    tile_transform(pos, board),
                    GlobalTransform::default(),
                    Tile { pos, color },
                    PlayingMarker,
                ))
                .observe(on_tile_hovered)
                .observe(on_tile_out);
        }
    }
}
