use crate::rules::piece::PieceColor;

#[derive(Debug, Clone)]
pub struct TopPanelText(String);

impl TopPanelText {
    pub fn turn(color: PieceColor) -> Self {
        Self(Self::format_turn(color))
    }

    pub fn set_turn(&mut self, color: PieceColor) {
        self.0 = Self::format_turn(color);
    }

    fn format_turn(color: PieceColor) -> String {
        format!("{}'s Turn", color)
    }
}

impl AsRef<str> for TopPanelText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
