use crate::GameError;
use indexmap::IndexMap;
use rule_engine::{
    CheckedGameRules,
    count::Count,
    piece::{PieceColor, PieceModel},
    player::PlayerState,
};

#[derive(Debug)]
pub struct PieceState {
    /// The number of pieces in stock for this model.
    stock: Count,

    /// The number of pieces captured for this model.
    captured: usize,
}

impl PieceState {
    /// Returns the number of in stock count.
    pub fn stock(&self) -> Count {
        self.stock
    }

    /// Returns the number of captured count.
    pub fn captured(&self) -> usize {
        self.captured
    }

    /// Tries to take a piece from stock.
    pub fn try_take_stock(&mut self) -> Result<(), GameError> {
        Ok(self.stock.decrease()?)
    }

    /// Records a capture for this piece model.
    pub fn record_capture(&mut self) {
        self.captured += 1;
    }
}

/// Represents a player in the game.
#[derive(Debug)]
pub struct Player {
    /// The current state of the player.
    state: PlayerState,

    /// The piece state for the player.
    ///
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    piece: IndexMap<PieceModel, PieceState>,
}

impl Player {
    pub fn new(rules: &CheckedGameRules) -> Self {
        Self {
            state: PlayerState::Active,
            piece: rules
                .pieces()
                .map(|(model, rules)| {
                    (
                        model,
                        PieceState {
                            stock: rules.count(),
                            captured: 0,
                        },
                    )
                })
                .collect(),
        }
    }

    /// Returns the current state of the player.
    pub fn state(&self) -> PlayerState {
        self.state
    }

    /// Sets the state of the player.
    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    /// Returns the piece state for the specified model.
    pub fn piece(&self, model: PieceModel) -> &PieceState {
        self.piece.get(&model).expect("No such piece model found")
    }

    /// Returns the mutable piece state for the specified model.
    pub fn piece_mut(&mut self, model: PieceModel) -> &mut PieceState {
        self.piece
            .get_mut(&model)
            .expect("No such piece model found")
    }

    /// Returns an iterator over the piece states.
    pub fn pieces(&self) -> impl Iterator<Item = (PieceModel, &PieceState)> {
        self.piece.iter().map(|(model, state)| (*model, state))
    }
}

#[derive(Debug)]
pub struct Players {
    /// Uses [`IndexMap`] to ensure a stable iteration order.
    map: IndexMap<PieceColor, Player>,
}

impl Players {
    pub fn new(rules: &CheckedGameRules) -> Self {
        Self {
            map: rules
                .players()
                .map(|(color, _)| (color, Player::new(rules)))
                .collect(),
        }
    }

    /// Returns the player with the specified color.
    pub fn get_by_color(&self, color: PieceColor) -> &Player {
        self.map.get(&color).expect("No such player found")
    }

    /// Returns the mutable player with the specified color.
    pub fn get_by_color_mut(&mut self, color: PieceColor) -> &mut Player {
        self.map.get_mut(&color).expect("No such player found")
    }

    /// Returns the player with the index.
    pub fn get_by_index(&self, index: usize) -> (PieceColor, &Player) {
        self.map
            .get_index(index)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Returns the mutable player with the index.
    pub fn get_by_index_mut(&mut self, index: usize) -> (PieceColor, &mut Player) {
        self.map
            .get_index_mut(index)
            .map(|(color, player)| (*color, player))
            .unwrap()
    }

    /// Returns the number of players.
    pub fn num(&self) -> usize {
        self.map.len()
    }

    /// Returns all players.
    pub fn iter(&self) -> impl Iterator<Item = (PieceColor, &Player)> {
        self.map.iter().map(|(color, player)| (*color, player))
    }

    /// Returns all mutable players.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PieceColor, &mut Player)> {
        self.map.iter_mut().map(|(color, player)| (*color, player))
    }

    /// Returns formatted string for the player states.
    pub fn player_states_message(&self) -> String {
        self.map
            .iter()
            .map(|(color, player)| format!("{}[{}]", color, player.state()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
