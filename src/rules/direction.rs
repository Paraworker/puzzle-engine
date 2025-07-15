use crate::rules::distance::Distance;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

/// A set of rules of a direction.
#[derive(Debug)]
pub struct DirectionInfo {
    max_distance: Distance,
}

impl DirectionInfo {
    /// Creates a new `DirectionInfo`.
    pub const fn new(max_distance: Distance) -> Self {
        Self { max_distance }
    }

    /// Returns the max move distance of this direction.
    pub const fn max_distance(&self) -> Distance {
        self.max_distance
    }
}

#[derive(Debug)]
pub struct Directions {
    map: HashMap<Direction, DirectionInfo>,
}

impl Directions {
    /// Creates a new `Directions`.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Adds a direction info.
    pub fn add(&mut self, direction: Direction, info: DirectionInfo) {
        self.map.insert(direction, info);
    }

    /// Returns the info of the given direction.
    pub fn get(&self, direction: Direction) -> Option<&DirectionInfo> {
        self.map.get(&direction)
    }
}
