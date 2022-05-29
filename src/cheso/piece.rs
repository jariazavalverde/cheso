/// Types of pieces.
///
/// A piece can be one of six types: king, queen, rook, bishop, knight, or pawn.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
