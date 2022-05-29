use crate::cheso::piece::Piece;
use crate::cheso::square::Square;

/// Knight translations.
///
/// Constant to generate all possible moves of a knight.
pub static KNIGHT_TRANSLATIONS: &'static [(isize, isize)] =
    &[(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1)];

/// King translations.
///
/// Constant to generate all possible moves of a king.
pub static KING_TRANSLATIONS: &'static [(isize, isize)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

/// Promotion pieces.
///
/// Constant to generate all possible promotions.
pub static PROMOTION_PIECES: &'static [Piece] =
    &[Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

/// Movement representation.
///
/// A movement is represented by the original square of the piece, the square where it goes.
/// Pawn movements include a promotion field to store the piece which replaces the pawn.
/// Castling can be detected by looking at the number of files moved by the king.
#[derive(Copy, Clone)]
pub struct Movement {
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
    pub promotion: Option<Piece>,
}

impl Movement {
    /// Check if the movement is a capture.
    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }
}
