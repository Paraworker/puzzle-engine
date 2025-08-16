use crate::{
    RulesError,
    expr::Context,
    piece::{PieceColor, PieceModel},
    pos::Pos,
    rect::Rect,
    utils::{from_ron_str, to_ron_str},
};
use serde::{Deserialize, Serialize};

/// Integer expression.
#[derive(Debug, Serialize, Deserialize)]
pub enum IntExpr {
    /// A constant integer value.
    Const(i64),

    /// Addition
    Add(Box<IntExpr>, Box<IntExpr>),
    /// Subtraction
    Sub(Box<IntExpr>, Box<IntExpr>),
    /// Multiplication
    Mul(Box<IntExpr>, Box<IntExpr>),
    /// Division
    Div(Box<IntExpr>, Box<IntExpr>),
    /// Absolute value
    Abs(Box<IntExpr>),

    /// Query turn information
    ///
    /// - TurnNumber: The current turn number.
    /// - RoundNumber: The current round number.
    TurnNumber,
    RoundNumber,

    /// Query last action information
    ///
    /// - LastActionRow: The row of the last action.
    /// - LastActionCol: The column of the last action.
    LastActionRow,
    LastActionCol,

    /// Query piece count
    ///
    /// - CountInRect: The number of pieces in the given rectangle.
    /// - CountPieceInRect: The number of pieces with the given model and color in the given rectangle.
    ///
    CountInRect((Box<IntExpr>, Box<IntExpr>), (Box<IntExpr>, Box<IntExpr>)),
    CountPieceInRect(
        (PieceModel, PieceColor),
        (Box<IntExpr>, Box<IntExpr>),
        (Box<IntExpr>, Box<IntExpr>),
    ),

    /// Movement only variables
    ///
    /// - SourceRow: The source tile row.
    /// - SourceCol: The source tile column.
    /// - TargetRow: The destination tile row.
    /// - TargetCol: The destination tile column.
    SourceRow,
    SourceCol,
    TargetRow,
    TargetCol,

    /// Placement only variables
    ///
    /// - ToPlaceRow: The row where the piece is being placed.
    /// - ToPlaceCol: The column where the piece is being placed.
    ToPlaceRow,
    ToPlaceCol,
}

impl IntExpr {
    /// Evaluates the arithmetic expression.
    pub fn evaluate<C>(&self, ctx: &C) -> Result<i64, C::Error>
    where
        C: Context,
    {
        match self {
            IntExpr::Const(n) => Ok(*n),
            IntExpr::Add(lhs, rhs) => Ok(lhs.evaluate(ctx)? + rhs.evaluate(ctx)?),
            IntExpr::Sub(lhs, rhs) => Ok(lhs.evaluate(ctx)? - rhs.evaluate(ctx)?),
            IntExpr::Mul(lhs, rhs) => Ok(lhs.evaluate(ctx)? * rhs.evaluate(ctx)?),
            IntExpr::Div(lhs, rhs) => Self::div(lhs, rhs, ctx),
            IntExpr::Abs(expr) => Ok(expr.evaluate(ctx)?.abs()),
            IntExpr::TurnNumber => ctx.turn_number(),
            IntExpr::RoundNumber => ctx.round_number(),
            IntExpr::LastActionRow => ctx.last_action_row(),
            IntExpr::LastActionCol => ctx.last_action_col(),
            IntExpr::CountInRect(pos1, pos2) => Self::count_in_rect(pos1, pos2, ctx),
            IntExpr::CountPieceInRect(piece, pos1, pos2) => {
                Self::count_piece_in_rect(*piece, pos1, pos2, ctx)
            }
            IntExpr::SourceRow => ctx.source_row(),
            IntExpr::SourceCol => ctx.source_col(),
            IntExpr::TargetRow => ctx.target_row(),
            IntExpr::TargetCol => ctx.target_col(),
            IntExpr::ToPlaceRow => ctx.to_place_row(),
            IntExpr::ToPlaceCol => ctx.to_place_col(),
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

    /// Evaluates the division operation.
    fn div<C>(lhs: &IntExpr, rhs: &IntExpr, ctx: &C) -> Result<i64, C::Error>
    where
        C: Context,
    {
        let num = lhs.evaluate(ctx)?;
        let denom = rhs.evaluate(ctx)?;

        if denom == 0 {
            Err(RulesError::DivisionByZero.into())
        } else {
            Ok(num / denom)
        }
    }

    fn count_in_rect<C>(
        pos1: &(Box<IntExpr>, Box<IntExpr>),
        pos2: &(Box<IntExpr>, Box<IntExpr>),
        ctx: &C,
    ) -> Result<i64, C::Error>
    where
        C: Context,
    {
        ctx.count_in_rect(Rect::new(
            Pos::new(pos1.0.evaluate(ctx)?, pos1.1.evaluate(ctx)?),
            Pos::new(pos2.0.evaluate(ctx)?, pos2.1.evaluate(ctx)?),
        ))
    }

    fn count_piece_in_rect<C>(
        piece: (PieceModel, PieceColor),
        pos1: &(Box<IntExpr>, Box<IntExpr>),
        pos2: &(Box<IntExpr>, Box<IntExpr>),
        ctx: &C,
    ) -> Result<i64, C::Error>
    where
        C: Context,
    {
        ctx.count_piece_in_rect(
            piece,
            Rect::new(
                Pos::new(pos1.0.evaluate(ctx)?, pos1.1.evaluate(ctx)?),
                Pos::new(pos2.0.evaluate(ctx)?, pos2.1.evaluate(ctx)?),
            ),
        )
    }
}
