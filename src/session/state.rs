use crate::session::piece_index::PieceEntities;

#[derive(Debug)]
pub enum SessionState {
    Navigating,
    Dragging(PieceEntities),
}
