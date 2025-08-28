use crate::{
    RulesError,
    expr::{Context, boolean::BoolExpr, integer::IntExpr},
    piece::PieceColor,
    pos::Pos,
    utils::{from_ron_str, to_ron_str},
};
use serde::{Deserialize, Serialize};

/// Color expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum ColorExpr {
    /// Literal color value.
    Literal(PieceColor),

    /// Conditional expression
    ///
    /// (condition, then, otherwise)
    If(Box<BoolExpr>, Box<ColorExpr>, Box<ColorExpr>),

    /// Query the color of the piece at the given position.
    ColorAtPos(Box<IntExpr>, Box<IntExpr>),

    /// Query the color of the piece being moved (Movement only).
    MovingColor,

    /// Query the color of the piece being placed (Placement only).
    ToPlaceColor,
}

impl ColorExpr {
    /// Evaluates the expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<PieceColor, C::Error>
    where
        C: Context,
    {
        match self {
            ColorExpr::Literal(color) => Ok(*color),
            ColorExpr::If(cond, then, otherwise) => Self::conditional(cond, then, otherwise, ctx),
            ColorExpr::ColorAtPos(row, col) => {
                ctx.color_at_pos(Pos::new(row.evaluate(ctx)?, col.evaluate(ctx)?))
            }
            ColorExpr::MovingColor => ctx.moving_color(),
            ColorExpr::ToPlaceColor => ctx.to_place_color(),
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
        then: &ColorExpr,
        otherwise: &ColorExpr,
        ctx: &C,
    ) -> Result<PieceColor, C::Error>
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
