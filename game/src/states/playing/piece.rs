use crate::{
    GameError,
    assets::GameAssets,
    expr_contexts::{movement::MovementContext, placement::PlacementContext},
    states::playing::{
        PiecePress,
        board::pos_translation,
        session::{GameSession, PlacedPieceIndex, player::Players},
        tile::Tile,
    },
};
use bevy::prelude::*;
use bevy_tweening::{
    AnimTarget, Sequence, Tween, TweenAnim,
    lens::{TransformPositionLens, TransformScaleLens},
};
use rulery::{
    CheckedGameRules,
    piece::{PieceColor, PieceModel, PieceRules},
    pos::Pos,
};
use std::{
    collections::{HashSet, hash_map::Entry},
    time::Duration,
};

/// Entities associated with a piece.
#[derive(Debug, Clone)]
pub struct PieceEntities {
    root: Entity,
    base_mesh: Entity,
    highlight: Entity,
}

impl PieceEntities {
    /// Creates a new `PieceEntities`.
    pub fn new(root: Entity, base_mesh: Entity, highlight: Entity) -> Self {
        Self {
            root,
            base_mesh,
            highlight,
        }
    }

    /// Returns the piece root entity.
    pub fn root(&self) -> Entity {
        self.root
    }

    /// Returns the base mesh entity.
    pub fn base_mesh(&self) -> Entity {
        self.base_mesh
    }

    /// Returns the highlight entity.
    pub fn highlight(&self) -> Entity {
        self.highlight
    }
}

#[derive(Debug, Clone)]
pub struct PlacedPiece {
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
    entities: PieceEntities,
}

impl PlacedPiece {
    /// Creates a new placed piece.
    pub fn new(model: PieceModel, color: PieceColor, pos: Pos, entities: PieceEntities) -> Self {
        Self {
            model,
            color,
            pos,
            entities,
        }
    }

    /// Returns the piece model.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the piece color.
    pub fn color(&self) -> PieceColor {
        self.color
    }

    /// Returns the position of the placed piece.
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Returns the piece entities.
    pub fn entities(&self) -> &PieceEntities {
        &self.entities
    }
}

#[derive(Debug, Resource)]
pub struct MovingPiece {
    model: PieceModel,
    color: PieceColor,
    source: Pos,
    entities: PieceEntities,
    movable: HashSet<Pos>,
}

impl MovingPiece {
    /// Creates a new moving piece.
    pub fn new(model: PieceModel, color: PieceColor, source: Pos, entities: PieceEntities) -> Self {
        Self {
            model,
            color,
            source,
            entities,
            movable: HashSet::new(),
        }
    }

    /// Collects movable positions based on the given movement expression.
    pub fn collect_movable(
        &mut self,
        session: &GameSession,
        tile_query: Query<&Tile>,
        rules: &PieceRules,
    ) -> Result<(), GameError> {
        for tile in tile_query {
            // Skip source tile
            if self.source == tile.pos() {
                continue;
            }

            let ctx = MovementContext {
                session,
                moving_model: self.model,
                moving_color: self.color,
                source_pos: self.source,
                target_pos: tile.pos(),
            };

            if rules.can_move(&ctx)? {
                self.movable.insert(tile.pos());
            }
        }

        Ok(())
    }

    /// Returns the set of movable positions.
    pub fn movable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.movable.iter().cloned()
    }

    /// Checks if the piece can move to the given position.
    pub fn can_move_to(&self, pos: Pos) -> bool {
        self.movable.contains(&pos)
    }

    /// Returns the piece model.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the piece color.
    pub fn color(&self) -> PieceColor {
        self.color
    }

    /// Returns the source position.
    pub fn source_pos(&self) -> Pos {
        self.source
    }

    /// Returns the piece entities.
    pub fn entities(&self) -> &PieceEntities {
        &self.entities
    }
}

#[derive(Debug, Resource)]
pub struct PlacingPiece {
    model: PieceModel,
    color: PieceColor,
    placeable: HashSet<Pos>,
}

impl PlacingPiece {
    /// Creates a new placing piece.
    pub fn new(model: PieceModel, color: PieceColor) -> Self {
        Self {
            model,
            color,
            placeable: HashSet::new(),
        }
    }

    /// Collects placeable positions based on the given placement expression.
    pub fn collect_placeable(
        &mut self,
        session: &GameSession,
        tile_query: Query<&Tile>,
        rules: &PieceRules,
    ) -> Result<(), GameError> {
        for tile in tile_query {
            let ctx = PlacementContext {
                session,
                to_place_model: self.model,
                to_place_color: self.color,
                to_place_pos: tile.pos(),
            };

            if rules.can_place(&ctx)? {
                self.placeable.insert(tile.pos());
            }
        }

        Ok(())
    }

    /// Returns the set of placeable positions.
    pub fn placeable_tiles(&self) -> impl Iterator<Item = Pos> {
        self.placeable.iter().cloned()
    }

    /// Checks if the piece can be placed at the given position.
    pub fn can_place_at(&self, pos: Pos) -> bool {
        self.placeable.contains(&pos)
    }

    /// Returns the piece model.
    pub fn model(&self) -> PieceModel {
        self.model
    }

    /// Returns the piece color.
    pub fn color(&self) -> PieceColor {
        self.color
    }
}

#[derive(Debug, Component)]
pub struct PiecePos(pub Pos);

/// Spawns a piece on the board at the specified position with the given model and color.
pub fn place_new_piece(
    commands: &mut Commands,
    assets: &GameAssets,
    rules: &CheckedGameRules,
    board_entity: Entity,
    players: &mut Players,
    placed_pieces: &mut PlacedPieceIndex,
    model: PieceModel,
    color: PieceColor,
    pos: Pos,
) -> Result<(), GameError> {
    fn on_piece_pressed(on_press: On<Pointer<Press>>, mut msg: MessageWriter<PiecePress>) {
        msg.write(PiecePress(on_press.event_target(), on_press.button));
    }

    let Entry::Vacant(entry) = placed_pieces.entry(pos) else {
        // Try to spawn duplicate piece at one position.
        return Err(GameError::DuplicatePiece(pos));
    };

    // Decrease the piece stock
    players
        .get_by_color_mut(color)
        .piece_mut(model)
        .try_take_stock()?;

    let (mesh, local_transform) = assets.meshes.piece.get(model);

    // Animation
    let (transform, movement_sequence, scale_sequence) = {
        let end = pos_translation(pos, rules);
        let mut start = end;

        let movement_sequence = {
            start.translation.y += CheckedGameRules::tile_height() * 10.0;
            start.scale = Vec3::splat(0.8);

            let impact = CheckedGameRules::tile_height() * 0.06;

            let drop_in = Tween::new(
                EaseFunction::CubicIn,
                Duration::from_millis(140),
                TransformPositionLens {
                    start: start.translation,
                    end: end.translation - Vec3::Y * impact,
                },
            );

            let rebound = Tween::new(
                EaseFunction::BounceOut,
                Duration::from_millis(80),
                TransformPositionLens {
                    start: end.translation - Vec3::Y * impact,
                    end: end.translation,
                },
            );

            Sequence::new([drop_in, rebound])
        };

        let scale_sequence = {
            let squash = Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(140),
                TransformScaleLens {
                    start: start.scale,
                    end: Vec3::new(1.08, 0.90, 1.08),
                },
            );

            let settle = Tween::new(
                EaseFunction::BackOut,
                Duration::from_millis(110),
                TransformScaleLens {
                    start: Vec3::new(1.08, 0.90, 1.08),
                    end: Vec3::ONE,
                },
            );

            Sequence::new([squash, settle])
        };

        (start, movement_sequence, scale_sequence)
    };

    let piece_root = commands
        .spawn((transform, GlobalTransform::default(), PiecePos(pos)))
        .id();

    commands.spawn((
        TweenAnim::new(movement_sequence),
        AnimTarget::component::<Transform>(piece_root),
    ));
    commands.spawn((
        TweenAnim::new(scale_sequence),
        AnimTarget::component::<Transform>(piece_root),
    ));

    let base_mesh = commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(assets.materials.piece.get(color).clone()),
            local_transform.clone(),
            GlobalTransform::default(),
        ))
        .observe(on_piece_pressed)
        .id();

    let highlight = commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(assets.materials.common.highlight_source_or_target.clone()),
            local_transform.clone(),
            GlobalTransform::default(),
            Pickable::IGNORE,
            Visibility::Hidden,
        ))
        .id();

    commands.entity(board_entity).add_child(piece_root);
    commands
        .entity(piece_root)
        .add_children(&[base_mesh, highlight]);

    // Add to placed piece index
    entry.insert(PlacedPiece::new(
        model,
        color,
        pos,
        PieceEntities::new(piece_root, base_mesh, highlight),
    ));

    Ok(())
}

/// Despawns a piece at the specified position.
pub fn capture_piece(
    commands: &mut Commands,
    placed_piece_index: &mut PlacedPieceIndex,
    players: &mut Players,
    pos: Pos,
) {
    if let Some(placed) = placed_piece_index.remove(&pos) {
        players
            .get_by_color_mut(placed.color())
            .piece_mut(placed.model())
            .record_capture();

        commands.entity(placed.entities().root()).despawn();
    }
}
