use crazy_puzzle_rules::{
    conditions::placement::{PlacementBool, PlacementInt},
    expr::{Context, QueryError},
    piece::{PieceColor, PieceModel},
    position::Pos,
};

#[derive(Debug)]
pub struct PlacementContext {
    pub model: PieceModel,
    pub color: PieceColor,
    pub turn_number: i64,
    pub round_number: i64,
    pub to_place: Pos,
}

impl Context for PlacementContext {
    type BoolVar = PlacementBool;
    type IntVar = PlacementInt;

    fn query_bool(&self, var: &Self::BoolVar) -> Result<bool, QueryError> {
        match var {
            PlacementBool::ToPlaceModelEqual(model) => Ok(self.model == *model),
            PlacementBool::ToPlaceColorEqual(color) => Ok(self.color == *color),
        }
    }

    fn query_int(&self, var: &Self::IntVar) -> Result<i64, QueryError> {
        match var {
            PlacementInt::TurnNumber => Ok(self.turn_number),
            PlacementInt::RoundNumber => Ok(self.round_number),
            PlacementInt::ToPlaceCol => Ok(self.to_place.col()),
            PlacementInt::ToPlaceRow => Ok(self.to_place.row()),
        }
    }
}
