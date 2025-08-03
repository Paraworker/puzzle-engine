use crate::{piece::PlacingPiece, session::piece_index::PieceEntities};

#[derive(Debug)]
pub enum SessionState {
    /// The player is selecting a piece to move on the board.
    Selecting,

    /// The player is moving a piece on the board.
    /// Stores the associated entities of the moving piece.
    Moving(PieceEntities),

    /// The player is placing a new piece on the board.
    /// Stores the placing piece data.
    Placing(PlacingPiece),

    /// The player is reviewing the board in a read-only state.
    Reviewing,
}
