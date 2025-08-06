use crate::{
    board::BoardRuleSet,
    expr::boolean::BoolExpr,
    initial_layout::InitialLayout,
    piece::{PieceColor, PieceModel, PieceRuleSet, PieceRules},
    player::{PlayerRuleSet, PlayerRules},
    utils::{ron_from_file, ron_to_file},
};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod board;
pub mod count;
pub mod expr;
pub mod initial_layout;
pub mod piece;
pub mod player;
pub mod position;
pub mod utils;

#[derive(Debug, Error)]
pub enum RulesError {
    #[error("invalid board size")]
    InvalidBoardSize,
    #[error("duplicate piece color")]
    DuplicateColor,
    #[error("duplicate piece model")]
    DuplicateModel,
    #[error("division by zero")]
    DivisionByZero,
    #[error("unsupported variable")]
    UnsupportedVariable,
    #[error("piece count is depleted")]
    CountDepleted,
    #[error("ron serialize or deserialize error: {0}")]
    RonSpanned(#[from] SpannedError),
    #[error("ron error: {0}")]
    Ron(#[from] ron::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Rules for a game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub name: String,
    pub board: BoardRuleSet,
    pub pieces: PieceRuleSet,
    pub players: PlayerRuleSet,
    pub initial_layout: InitialLayout,
    pub game_over_condition: BoolExpr,
}

impl GameRules {
    pub fn load<P>(path: P) -> Result<Self, RulesError>
    where
        P: AsRef<Path>,
    {
        ron_from_file(path)
    }

    pub fn save<P>(&self, path: P) -> Result<(), RulesError>
    where
        P: AsRef<Path>,
    {
        ron_to_file(self, path)
    }
}

impl Default for GameRules {
    fn default() -> Self {
        let mut pieces = PieceRuleSet::default();
        let mut players = PlayerRuleSet::default();

        // At least one type of piece, `Cube` as default.
        pieces.add(PieceModel::Cube, PieceRules::default()).unwrap();

        // At least one player, `White` as default.
        players
            .add(PieceColor::White, PlayerRules::default())
            .unwrap();

        Self {
            name: "Default Rules".into(),
            board: Default::default(),
            pieces,
            players,
            initial_layout: Default::default(),
            game_over_condition: BoolExpr::False,
        }
    }
}
