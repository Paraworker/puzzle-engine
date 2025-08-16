use crate::{
    RulesError,
    expr::{Context, integer::IntExpr},
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

    /// Query board state
    ///
    /// - ColorAtPos: The color of the piece at the given position.
    ColorAtPos(IntExpr, IntExpr),

    /// Movement only
    ///
    /// - MovingColor: The color of the piece being moved.
    MovingColor,

    /// Placement only
    ///
    /// - ToPlaceColor: The color of the piece being placed.
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
}
