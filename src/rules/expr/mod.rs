use crate::{piece::PieceKind, position::Pos};

pub mod arith;
pub mod boolean;

#[derive(Debug)]
pub struct ExprContext {
    pub kind: PieceKind,
    pub source: Pos,
    pub target: Pos,
}
