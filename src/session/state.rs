use crate::session::pieces::PieceEntities;

#[derive(Debug)]
pub enum SessionState {
    Navigating,
    Dragging(PieceEntities),
}
