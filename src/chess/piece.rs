use crate::chess::color::Color;

// Enumeration of piece-type codes.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
