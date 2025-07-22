use crate::{
    assets::{GameAssets, materials::piece::PieceColor, meshes::piece::PieceModel},
    rules::{GameRules, board::BoardGeometry},
    states::playing::{
        board::Pos,
        session::{GameSession, PlayState},
    },
};
use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};

#[derive(Debug, Component)]
pub struct Piece {
    pos: Pos,
    model: PieceModel,
    color: PieceColor,
}

impl Piece {
    /// Sets the position of the piece on the board.
    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    /// Returns the position of the piece on the board.
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Returns the model of the piece.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the color of the piece.
    pub fn color(&self) -> PieceColor {
        self.color
    }
}

#[derive(Debug)]
pub struct DraggedPiece {
    entity: Entity,
}

impl DraggedPiece {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

/// Calculates the world translation for placing a piece to a specific tile position.
pub fn place_piece(to: Pos, board: &BoardGeometry) -> Vec3 {
    Vec3::new(
        to.col() as f32 * board.tile_size() - board.half_width_col(),
        0.25,
        to.row() as f32 * board.tile_size() - board.half_width_row(),
    )
}

/// Spawns a piece on the board at the specified position with the given model and color.
pub fn spawn_piece(
    commands: &mut Commands,
    assets: &GameAssets,
    board: &BoardGeometry,
    pos: Pos,
    model: PieceModel,
    color: PieceColor,
) {
    fn on_piece_hovered(
        trigger: Trigger<Pointer<Over>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
        session: Res<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if let PlayState::Navigating = session.state {
            if let Ok((mut material, _)) = query.get_mut(trigger.target()) {
                material.0 = assets.materials.common.piece_hover.clone();
            }
        }
    }

    fn on_piece_out(
        trigger: Trigger<Pointer<Out>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
        session: Res<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if let PlayState::Navigating = session.state {
            if let Ok((mut material, piece)) = query.get_mut(trigger.target()) {
                // Restore the piece's original color
                material.0 = assets.materials.piece.get(piece.color()).clone();
            }
        }
    }

    fn on_piece_pressed(
        trigger: Trigger<Pointer<Pressed>>,
        mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
        mut session: ResMut<GameSession>,
        assets: Res<GameAssets>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }

        if let PlayState::Navigating = session.state {
            if let Ok((mut material, _)) = query.get_mut(trigger.target()) {
                material.0 = assets.materials.common.piece_dragged.clone();

                // Start dragging state
                session.state = PlayState::Dragging(DraggedPiece::new(trigger.target()));
            }
        }
    }

    commands
        .spawn((
            Mesh3d(assets.meshes.piece.get(model).clone()),
            MeshMaterial3d(assets.materials.piece.get(color).clone()),
            Transform {
                translation: place_piece(pos, board),
                scale: Vec3::splat(board.tile_size() * 0.5),
                ..default()
            },
            Piece { pos, model, color },
        ))
        .observe(on_piece_hovered)
        .observe(on_piece_out)
        .observe(on_piece_pressed);
}

/// A system that finishes dragging a piece when the primary button is released.
pub fn finish_dragging(
    mut released: EventReader<Pointer<Released>>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Piece)>,
    mut session: ResMut<GameSession>,
    assets: Res<GameAssets>,
) {
    if let PlayState::Dragging(dragged) = &mut session.state {
        for event in released.read() {
            if event.button == PointerButton::Primary {
                // Reset the dragged piece's material to its original color
                if let Ok((mut material, piece)) = query.get_mut(dragged.entity()) {
                    material.0 = assets.materials.piece.get(piece.color()).clone();
                }

                // Finish dragging state
                session.state = PlayState::Navigating;

                // We only handle the first release event
                break;
            }
        }
    }
}

/// A system that spawns test pieces on the board when the 'P' key is pressed.
pub fn test_spawn_piece(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    assets: Res<GameAssets>,
    rules: Res<GameRules>,
) {
    for ev in key_evr.read() {
        if ev.key_code == KeyCode::KeyP && ev.state == ButtonState::Pressed {
            spawn_piece(
                &mut commands,
                &assets,
                rules.board_geometry(),
                Pos::new(0, 0),
                PieceModel::Cube,
                PieceColor::Black,
            );

            spawn_piece(
                &mut commands,
                &assets,
                rules.board_geometry(),
                Pos::new(1, 1),
                PieceModel::Sphere,
                PieceColor::White,
            );

            spawn_piece(
                &mut commands,
                &assets,
                rules.board_geometry(),
                Pos::new(2, 2),
                PieceModel::Cylinder,
                PieceColor::Black,
            );
        }
    }
}
