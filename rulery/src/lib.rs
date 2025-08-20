use crate::{
    board::BoardRuleSet,
    count::Count,
    expr::{Context, boolean::BoolExpr},
    initial_layout::{InitialLayout, InitialPiece},
    piece::{PieceColor, PieceModel, PieceRuleSet, PieceRules},
    player::{PlayerRuleSet, PlayerRules},
    pos::Pos,
    utils::{from_ron_file, to_ron_file},
};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
use thiserror::Error;

pub mod board;
pub mod count;
pub mod expr;
pub mod initial_layout;
pub mod piece;
pub mod player;
pub mod pos;
pub mod rect;

mod utils;

#[derive(Debug, Error)]
pub enum RulesError {
    #[error("invalid board size")]
    InvalidBoardSize,
    #[error("duplicate piece color")]
    DuplicateColor,
    #[error("duplicate piece model")]
    DuplicateModel,
    #[error("no such piece color: {0}")]
    NoSuchColor(PieceColor),
    #[error("no such piece model: {0}")]
    NoSuchModel(PieceModel),
    #[error("no rule name")]
    NoName,
    #[error("no added piece")]
    NoAddedPiece,
    #[error("no added player")]
    NoAddedPlayer,
    #[error("division by zero")]
    DivisionByZero,
    #[error("initial piece position out of board: {0}")]
    InitialPosOutOfBoard(Pos),
    #[error("duplicate initial piece position: {0}")]
    DuplicateInitialPos(Pos),
    #[error("logical AND invalid arity")]
    AndInvalidArity,
    #[error("logical OR invalid arity")]
    OrInvalidArity,
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

/// Unchecked game rules.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct UncheckedGameRules(GameRulesInner);

impl UncheckedGameRules {
    /// Loads game rules from a file.
    pub fn load<P>(path: P) -> Result<Self, RulesError>
    where
        P: AsRef<Path>,
    {
        from_ron_file(path)
    }

    /// Sets the name of the game rules.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.0.name = name.into();
    }

    /// Sets the number of board rows.
    pub const fn set_board_rows(&mut self, num: i64) {
        self.0.board.set_rows(num);
    }

    /// Sets the number of board columns.
    pub const fn set_board_cols(&mut self, num: i64) {
        self.0.board.set_cols(num);
    }

    /// Adds a new piece model with its rules.
    pub fn add_piece(&mut self, model: PieceModel, rules: PieceRules) -> Result<(), RulesError> {
        self.0.pieces.add(model, rules)
    }

    /// Parses pieces from a ron string.
    pub fn set_pieces_from_ron_str(&mut self, str: &str) -> Result<(), RulesError> {
        let set = PieceRuleSet::from_ron_str(str)?;
        self.0.pieces = set;
        Ok(())
    }

    /// Adds a new player with its rules.
    pub fn add_player(&mut self, color: PieceColor, rules: PlayerRules) -> Result<(), RulesError> {
        self.0.players.add(color, rules)
    }

    /// Parses players from a ron string.
    pub fn set_players_from_ron_str(&mut self, str: &str) -> Result<(), RulesError> {
        let set = PlayerRuleSet::from_ron_str(str)?;
        self.0.players = set;
        Ok(())
    }

    /// Adds a new initial piece to the layout.
    pub fn add_initial_piece(&mut self, piece: InitialPiece) {
        self.0.initial_layout.add(piece);
    }

    /// Parses initial layout from a ron string.
    pub fn set_initial_layout_from_ron_str(&mut self, str: &str) -> Result<(), RulesError> {
        let layout = InitialLayout::from_ron_str(str)?;
        self.0.initial_layout = layout;
        Ok(())
    }

    /// Sets the game over condition from a boolean expression.
    pub fn set_game_over_condition(&mut self, cond: BoolExpr) {
        self.0.game_over_condition = cond;
    }

    /// Parses the game over condition from a ron string.
    pub fn set_game_over_condition_from_ron_str(&mut self, str: &str) -> Result<(), RulesError> {
        let cond = BoolExpr::from_ron_str(str)?;
        self.0.game_over_condition = cond;
        Ok(())
    }

    /// Checks the rules and returns a checked version.
    pub fn check(self) -> Result<CheckedGameRules, RulesError> {
        // Check name
        if self.0.name.is_empty() {
            return Err(RulesError::NoName);
        }

        // Check board size.
        if self.0.board.rows() <= 0 || self.0.board.cols() <= 0 {
            return Err(RulesError::InvalidBoardSize);
        }

        // Check pieces
        if self.0.pieces.is_empty() {
            return Err(RulesError::NoAddedPiece);
        }

        // Check players
        if self.0.players.is_empty() {
            return Err(RulesError::NoAddedPlayer);
        }

        // Check initial layout:
        // - Colors must exist in `players`
        // - Models must exist in `pieces`
        // - For each (model, color), the number of initial pieces must not exceed the model's count
        // - Each position must be inside the board
        // - Positions must not be duplicated
        {
            let rows = self.0.board.rows();
            let cols = self.0.board.cols();

            // Tally per (model, color)
            let mut per_color_model: HashMap<(PieceModel, PieceColor), usize> = HashMap::new();
            // Track occupied coordinates to catch duplicates
            let mut occupied: HashSet<Pos> = HashSet::new();

            for piece in self.0.initial_layout.pieces() {
                let model = piece.model();
                let color = piece.color();
                let pos = piece.pos();

                // 1) board bounds check: 0 <= r < rows, 0 <= c < cols
                if pos.row() < 0 || pos.row() >= rows || pos.col() < 0 || pos.col() >= cols {
                    return Err(RulesError::InitialPosOutOfBoard(pos));
                }

                // 2) duplicate position check
                if !occupied.insert(pos) {
                    return Err(RulesError::DuplicateInitialPos(pos));
                }

                // 3) declared color check
                let _ = self.0.players.get_by_color(color)?;

                // 4) declared model check
                let pr = self.0.pieces.get_by_model(model)?;

                // 5) per-(model,color) quota check
                let cnt = per_color_model.entry((model, color)).or_insert(0);
                *cnt += 1;

                if let Count::Finite(limit) = pr.count() {
                    if *cnt > limit {
                        // Initial layout exceeds the allowed count for this (model, color)
                        return Err(RulesError::CountDepleted);
                    }
                }
            }
        }

        Ok(CheckedGameRules(self.0))
    }
}

impl Default for UncheckedGameRules {
    fn default() -> Self {
        Self(GameRulesInner {
            name: String::new(),
            board: BoardRuleSet::new(),
            pieces: PieceRuleSet::new(),
            players: PlayerRuleSet::new(),
            initial_layout: InitialLayout::new(),
            game_over_condition: BoolExpr::False,
        })
    }
}

/// Checked game rules.
///
/// Read only after checking.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct CheckedGameRules(GameRulesInner);

impl CheckedGameRules {
    /// Saves the game rules to a file.
    pub fn save<P>(&self, path: P) -> Result<(), RulesError>
    where
        P: AsRef<Path>,
    {
        to_ron_file(&self.0, path)
    }

    /// Returns the name of the game rules.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Returns the board row count.
    pub const fn board_rows(&self) -> i64 {
        self.0.board.rows()
    }

    /// Returns the board column count.
    pub const fn board_cols(&self) -> i64 {
        self.0.board.cols()
    }

    /// Returns the size of each tile.
    pub const fn tile_size() -> f32 {
        BoardRuleSet::tile_size()
    }

    /// Returns the height of each tile.
    pub const fn tile_height() -> f32 {
        BoardRuleSet::tile_height()
    }

    /// Returns the piece rules for the specified model.
    pub fn get_piece(&self, model: PieceModel) -> Result<&PieceRules, RulesError> {
        self.0.pieces.get_by_model(model)
    }

    /// Returns all piece rules.
    pub fn pieces(&self) -> impl Iterator<Item = (PieceModel, &PieceRules)> {
        self.0.pieces.iter()
    }

    /// Converts pieces into a ron string.
    pub fn pieces_to_ron_str(&self) -> Result<String, RulesError> {
        self.0.pieces.to_ron_str()
    }

    /// Returns the player rules with the specified color.
    pub fn get_player(&self, color: PieceColor) -> Result<&PlayerRules, RulesError> {
        self.0.players.get_by_color(color)
    }

    /// Returns all player rules.
    pub fn players(&self) -> impl Iterator<Item = (PieceColor, &PlayerRules)> {
        self.0.players.iter()
    }

    /// Converts players into a ron string.
    pub fn players_to_ron_str(&self) -> Result<String, RulesError> {
        self.0.players.to_ron_str()
    }

    /// Returns all initial pieces.
    pub fn initial_pieces(&self) -> impl Iterator<Item = &InitialPiece> {
        self.0.initial_layout.pieces()
    }

    /// Converts initial layout into a ron string.
    pub fn initial_layout_to_ron_str(&self) -> Result<String, RulesError> {
        self.0.initial_layout.to_ron_str()
    }

    /// Evaluates game over condition.
    pub fn evaluate_game_over_condition<C>(&self, ctx: &C) -> Result<bool, C::Error>
    where
        C: Context,
    {
        self.0.game_over_condition.evaluate(ctx)
    }

    /// Converts game over condition into a ron string.
    pub fn game_over_condition_to_ron_str(&self) -> Result<String, RulesError> {
        self.0.game_over_condition.to_ron_str()
    }
}

impl Default for CheckedGameRules {
    fn default() -> Self {
        let mut pieces = PieceRuleSet::new();
        let mut players = PlayerRuleSet::new();
        let mut initial_layout = InitialLayout::new();

        // At least one type of piece, `Cube` as default.
        pieces
            .add(
                PieceModel::Cube,
                PieceRules::new(Count::Finite(10), BoolExpr::True, BoolExpr::True),
            )
            .unwrap();

        // At least one player, `White` as default.
        players
            .add(
                PieceColor::White,
                PlayerRules::new(BoolExpr::False, BoolExpr::False),
            )
            .unwrap();

        // Add some initial pieces.
        initial_layout.add(InitialPiece::new(
            PieceModel::Cube,
            PieceColor::White,
            Pos::new(0, 0),
        ));

        initial_layout.add(InitialPiece::new(
            PieceModel::Cube,
            PieceColor::White,
            Pos::new(1, 1),
        ));

        initial_layout.add(InitialPiece::new(
            PieceModel::Cube,
            PieceColor::White,
            Pos::new(2, 2),
        ));

        Self(GameRulesInner {
            name: "Default Rules".into(),
            board: BoardRuleSet::new(),
            pieces,
            players,
            initial_layout,
            game_over_condition: BoolExpr::False,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GameRulesInner {
    name: String,
    board: BoardRuleSet,
    pieces: PieceRuleSet,
    players: PlayerRuleSet,
    initial_layout: InitialLayout,
    game_over_condition: BoolExpr,
}
