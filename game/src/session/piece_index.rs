use bevy::prelude::*;
use rule_engine::position::Pos;
use std::collections::{HashMap, hash_map};

/// Entities associated with a piece.
#[derive(Debug, Clone)]
pub struct PieceEntities {
    base: Entity,
    highlighted: Entity,
}

impl PieceEntities {
    /// Creates a new `PieceEntities`.
    pub fn new(base: Entity, highlighted: Entity) -> Self {
        Self { base, highlighted }
    }

    /// Returns the base entity.
    pub fn base(&self) -> Entity {
        self.base
    }

    /// Returns the highlighted entity.
    pub fn highlighted(&self) -> Entity {
        self.highlighted
    }
}

#[derive(Debug)]
pub enum Entry<'a> {
    Vacant(Vacant<'a>),
    Occupied(Occupied<'a>),
}

#[derive(Debug)]
pub struct Vacant<'a>(hash_map::VacantEntry<'a, Pos, PieceEntities>);

impl<'a> Vacant<'a> {
    pub fn insert(self, entities: PieceEntities) -> &'a mut PieceEntities {
        self.0.insert(entities)
    }
}

#[derive(Debug)]
pub struct Occupied<'a>(hash_map::OccupiedEntry<'a, Pos, PieceEntities>);

impl Occupied<'_> {
    pub fn get(&self) -> &PieceEntities {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> &mut PieceEntities {
        self.0.get_mut()
    }

    pub fn remove(self) -> PieceEntities {
        self.0.remove()
    }
}

/// An index of placed piece entities by their position.
#[derive(Debug)]
pub struct PlacedPieceIndex(HashMap<Pos, PieceEntities>);

impl PlacedPieceIndex {
    /// Creates a new `PieceIndex`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the entry of a position key
    pub fn entry(&mut self, pos: Pos) -> Entry<'_> {
        match self.0.entry(pos) {
            hash_map::Entry::Occupied(o) => Entry::Occupied(Occupied(o)),
            hash_map::Entry::Vacant(v) => Entry::Vacant(Vacant(v)),
        }
    }

    /// Adds a piece entities at the given position.
    pub fn add(&mut self, pos: Pos, entities: PieceEntities) -> Option<PieceEntities> {
        self.0.insert(pos, entities)
    }

    /// Removes the piece entities at the given position.
    pub fn remove(&mut self, pos: Pos) -> Option<PieceEntities> {
        self.0.remove(&pos)
    }

    /// Returns the piece entities at the given position.
    pub fn get(&self, pos: Pos) -> Option<&PieceEntities> {
        self.0.get(&pos)
    }
}
