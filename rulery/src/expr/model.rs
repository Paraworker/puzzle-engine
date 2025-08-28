use crate::{
    RulesError,
    expr::{Context, boolean::BoolExpr, integer::IntExpr},
    piece::PieceModel,
    pos::Pos,
    utils::{from_ron_str, to_ron_str},
};
use serde::{Deserialize, Serialize};

/// Model expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum ModelExpr {
    /// Literal model value.
    Literal(PieceModel),

    /// Conditional expression
    ///
    /// (condition, then, otherwise)
    If(Box<BoolExpr>, Box<ModelExpr>, Box<ModelExpr>),

    /// Query the model of the piece at the given position.
    ModelAtPos(Box<IntExpr>, Box<IntExpr>),

    /// Query the model of the piece being moved (Movement only).
    MovingModel,

    /// Query the model of the piece being placed (Placement only).
    ToPlaceModel,
}

impl ModelExpr {
    /// Evaluates the expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<PieceModel, C::Error>
    where
        C: Context,
    {
        match self {
            ModelExpr::Literal(model) => Ok(*model),
            ModelExpr::If(cond, then, otherwise) => Self::conditional(cond, then, otherwise, ctx),
            ModelExpr::ModelAtPos(row, col) => {
                ctx.model_at_pos(Pos::new(row.evaluate(ctx)?, col.evaluate(ctx)?))
            }
            ModelExpr::MovingModel => ctx.moving_model(),
            ModelExpr::ToPlaceModel => ctx.to_place_model(),
        }
    }

    /// Parses from a ron string.
    pub fn from_ron_str(str: &str) -> Result<Self, RulesError> {
        from_ron_str(str)
    }

    /// Converts into a ron string.
    pub fn to_ron_str(&self) -> Result<String, RulesError> {
        to_ron_str(self)
    }

    fn conditional<C>(
        condition: &BoolExpr,
        then: &ModelExpr,
        otherwise: &ModelExpr,
        ctx: &C,
    ) -> Result<PieceModel, C::Error>
    where
        C: Context,
    {
        if condition.evaluate(ctx)? {
            then.evaluate(ctx)
        } else {
            otherwise.evaluate(ctx)
        }
    }
}
