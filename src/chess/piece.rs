use crate::chess::color::Color;

// Enumeration of piece-type codes.
pub enum Piece {
    Nil,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
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
