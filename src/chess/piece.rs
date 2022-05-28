use crate::chess::color::Color;

// Enumeration of piece-type codes.
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Nil,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn get_color(self) -> Option<Color> {
        match self {
            Self::Nil => None,
            Self::Pawn(color) => Some(color),
            Self::Knight(color) => Some(color),
            Self::Bishop(color) => Some(color),
            Self::Rook(color) => Some(color),
            Self::Queen(color) => Some(color),
            Self::King(color) => Some(color),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "Nil"),
            Self::Pawn(color) => write!(f, "Pawn({})", color),
            Self::Knight(color) => write!(f, "Knight({})", color),
            Self::Bishop(color) => write!(f, "Bishop({})", color),
            Self::Rook(color) => write!(f, "Rook({})", color),
            Self::Queen(color) => write!(f, "Queen({})", color),
            Self::King(color) => write!(f, "King({})", color),
        }
    }
}
