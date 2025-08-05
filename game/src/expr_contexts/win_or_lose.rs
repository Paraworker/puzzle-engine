use rule_engine::{
    conditions::win_or_lose::{WinOrLoseBool, WinOrLoseInt},
    expr::{Context, QueryError},
    piece::PieceColor,
};

#[derive(Debug)]
pub struct WinOrLoseContext {
    pub piece_color: PieceColor,
    pub turn_number: i64,
    pub round_number: i64,
}

impl Context for WinOrLoseContext {
    type BoolVar = WinOrLoseBool;
    type IntVar = WinOrLoseInt;

    fn query_bool(&self, var: &Self::BoolVar) -> Result<bool, QueryError> {
        match var {
            WinOrLoseBool::PlayerColorEqual(color) => Ok(self.piece_color == *color),
        }
    }

    fn query_int(&self, var: &Self::IntVar) -> Result<i64, QueryError> {
        match var {
            WinOrLoseInt::TurnNumber => Ok(self.turn_number),
            WinOrLoseInt::RoundNumber => Ok(self.round_number),
        }
    }
}
