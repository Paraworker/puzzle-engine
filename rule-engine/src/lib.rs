use crate::{
    board::BoardRuleSet,
    expr::boolean::BoolExpr,
    initial_layout::{InitialLayout, InitialPiece},
    piece::{PieceColor, PieceModel, PieceRuleSet, PieceRules},
    player::{PlayerRuleSet, PlayerRules},
    position::Pos,
    utils::{from_ron_file, to_ron_file},
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
    #[error("format error: {0}")]
    Format(#[from] SpannedError),
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
        from_ron_file(path)
    }

    pub fn save<P>(&self, path: P) -> Result<(), RulesError>
    where
        P: AsRef<Path>,
    {
        to_ron_file(self, path)
    }
}

impl Default for GameRules {
    fn default() -> Self {
        let mut pieces = PieceRuleSet::default();
        let mut players = PlayerRuleSet::default();
        let mut initial_layout = InitialLayout::default();

        // At least one type of piece, `Cube` as default.
        pieces.add(PieceModel::Cube, PieceRules::default()).unwrap();

        // At least one player, `White` as default.
        players
            .add(PieceColor::White, PlayerRules::default())
            .unwrap();

        // Add some initial pieces.
        initial_layout.add(InitialPiece {
            model: PieceModel::Cube,
            color: PieceColor::White,
            pos: Pos::new(0, 0),
        });

        initial_layout.add(InitialPiece {
            model: PieceModel::Cube,
            color: PieceColor::White,
            pos: Pos::new(1, 1),
        });

        initial_layout.add(InitialPiece {
            model: PieceModel::Cube,
            color: PieceColor::White,
            pos: Pos::new(2, 2),
        });

        Self {
            name: "Default Rules".into(),
            board: Default::default(),
            pieces,
            players,
            initial_layout,
            game_over_condition: BoolExpr::False,
        }
    }
}
