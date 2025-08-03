use crazy_puzzle_rules::{
    conditions::movement::{MovementBool, MovementInt},
    expr::{Context, QueryError},
    piece::{PieceColor, PieceModel},
    position::Pos,
};

#[derive(Debug)]
pub struct MovementContext {
    pub model: PieceModel,
    pub color: PieceColor,
    pub turn_number: i64,
    pub round_number: i64,
    pub source: Pos,
    pub target: Pos,
}

impl Context for MovementContext {
    type BoolVar = MovementBool;
    type IntVar = MovementInt;

    fn query_bool(&self, var: &Self::BoolVar) -> Result<bool, QueryError> {
        match var {
            MovementBool::MovingModelEqual(model) => Ok(self.model == *model),
            MovementBool::MovingColorEqual(color) => Ok(self.color == *color),
        }
    }

    fn query_int(&self, var: &Self::IntVar) -> Result<i64, QueryError> {
        match var {
            MovementInt::TurnNumber => Ok(self.turn_number),
            MovementInt::RoundNumber => Ok(self.round_number),
            MovementInt::SourceCol => Ok(self.source.col()),
            MovementInt::SourceRow => Ok(self.source.row()),
            MovementInt::TargetCol => Ok(self.target.col()),
            MovementInt::TargetRow => Ok(self.target.row()),
        }
    }
}
