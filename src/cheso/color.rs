/// Color of pieces.
///
/// The player who moves first is referred to as White, and the player who moves second is referred to as Black.
/// This enumeration is mainly used to indicate the color of pieces and the side to move.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Return the opposite color.
    pub fn opposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
