use rule_engine::{
    conditions::game_over::{GameOverBool, GameOverInt},
    expr::{Context, QueryError},
};

#[derive(Debug)]
pub struct GameOverContext {
    pub turn_number: i64,
    pub round_number: i64,
}

impl Context for GameOverContext {
    type BoolVar = GameOverBool;
    type IntVar = GameOverInt;

    fn query_bool(&self, var: &Self::BoolVar) -> Result<bool, QueryError> {
        todo!()
    }

    fn query_int(&self, var: &Self::IntVar) -> Result<i64, QueryError> {
        match var {
            GameOverInt::TurnNumber => Ok(self.turn_number),
            GameOverInt::RoundNumber => Ok(self.round_number),
        }
    }
}
