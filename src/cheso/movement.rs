use crate::cheso::board::Board;
use crate::cheso::piece::Piece;
use crate::cheso::square::Square;

/// Knight translations.
///
/// Constant to generate all possible moves of a knight.
static KNIGHT_TRANSLATIONS: &'static [(isize, isize)] =
    &[(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1)];

/// King translations.
///
/// Constant to generate all possible moves of a king.
static KING_TRANSLATIONS: &'static [(isize, isize)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

/// Movement representation.
///
/// A movement is represented by the original square of the piece, the square where it goes.
/// Pawn movements include a promotion field to store the piece which replaces the pawn.
/// Castling can be detected by looking at the number of files moved by the king.
#[derive(Copy, Clone)]
struct Movement {
    from: Square,
    to: Square,
    capture: Option<Piece>,
    promotion: Option<Piece>,
}

impl Movement {
    /// Check if movement is a capture.
    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }

    /// Make the movement in a board position.
    pub fn make(board: &Board) -> Board {
        Board {
            white_pieces: board.white_pieces.clone(),
            black_pieces: board.black_pieces.clone(),
            side_to_move: board.side_to_move.opposite(),
            white_kingside_castling: board.white_kingside_castling,
            white_queenside_castling: board.white_queenside_castling,
            black_kingside_castling: board.black_kingside_castling,
            black_queenside_castling: board.black_queenside_castling,
            en_passant: 0,
            reversible_moves: 0,
        }
    }

    /// Generate the list of pseudo-legal movements.
    /// Pieces obey normal rules of movement, but they are not checked to see if they will leave the king in check.
    pub fn generate(board: &Board) -> Vec<Movement> {
        let mut moves: Vec<Movement> = Vec::new();
        for (square, piece) in board.get_pieces_to_move() {
            match piece {
                Piece::Pawn => Self::as_pawn(board, square, &mut moves),
                Piece::Knight => Self::as_knight(board, square, &mut moves),
                Piece::Bishop => Self::as_bishop(board, square, &mut moves),
                Piece::Rook => Self::as_rook(board, square, &mut moves),
                Piece::Queen => Self::as_queen(board, square, &mut moves),
                Piece::King => Self::as_king(board, square, &mut moves),
            }
        }
        moves
    }

    /// Populate the list of movements from a square as a pawn.
    ///
    /// Pawns move vertically forward one square.
    /// Pawns can move two squares if they have not yet moved.
    /// Pawns capture one square diagonally in a forward direction.
    /// Pawns can make a special move (capture en passant).
    fn as_pawn(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    /// Populate the list of movements from a square as a knight.
    ///
    /// Knights move two squares in a horizontal or vertical direction, then move one square horizontally or vertically.
    /// Knights are the only piece able to jump over other pieces.
    fn as_knight(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::from_array(board, from, KNIGHT_TRANSLATIONS, moves);
    }

    /// Populate the list of movements from a square as a bishop.
    ///
    /// Bishops move diagonally any number of squares.
    /// Bishops are unable to jump over pieces.
    fn as_bishop(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::until_collision(board, from, Square::northeast, moves);
        Self::until_collision(board, from, Square::northwest, moves);
        Self::until_collision(board, from, Square::southeast, moves);
        Self::until_collision(board, from, Square::southwest, moves);
    }

    /// Populate the list of movements from a square as a rook.
    ///
    /// Rooks move horizontally or vertically any number of squares.
    /// Rooks are unable to jump over pieces.
    fn as_rook(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::until_collision(board, from, Square::north, moves);
        Self::until_collision(board, from, Square::south, moves);
        Self::until_collision(board, from, Square::east, moves);
        Self::until_collision(board, from, Square::west, moves);
    }

    /// Populate the list of movements from a square as a queen.
    ///
    /// Queens move diagonally, horizontally, or vertically any number of squares.
    /// Queens are unable to jump over pieces.
    fn as_queen(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::as_bishop(board, from, moves);
        Self::as_rook(board, from, moves);
    }

    /// Populate the list of movements from a square as a king.
    ///
    /// Kings move one square in any direction.
    /// Kings are unable to jump over pieces.
    fn as_king(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::from_array(board, from, KING_TRANSLATIONS, moves);
    }

    /// Populate the list of movements from an array of translations.
    ///
    /// For each translation, a movement from the original square to its translation is generated.
    /// If the generated movement is pseudo-legal, it is added to the list of pseudo-legal moves.
    /// This function generates all pseudo-legal moves for knights and kings (except castling).
    fn from_array(
        board: &Board,
        from: &Square,
        translations: &[(isize, isize)],
        moves: &mut Vec<Movement>,
    ) -> () {
        for i in 0..translations.len() {
            let (rank, file) = translations[i];
            let to: Option<Square> = from.translate(rank, file);
            match to {
                Some(square) => match board.get_square(&square) {
                    None => moves.push(Movement {
                        from: *from,
                        to: square,
                        capture: None,
                        promotion: None,
                    }),
                    Some((piece, color)) if color != board.side_to_move => moves.push(Movement {
                        from: *from,
                        to: square,
                        capture: Some(piece),
                        promotion: None,
                    }),
                    _ => (),
                },
                _ => (),
            }
        }
    }

    /// Populate the list of movements until collision with another piece.
    ///
    /// Starting in a square, the position is updated according to a function, until leaving the board or reaching a piece.
    /// At each step, a movement from initial square to current square is added to the list of pseudo-legal moves.
    /// This function generates all pseudo-legal moves for bishops, rooks and queens.
    fn until_collision(
        board: &Board,
        from: &Square,
        update: fn(&Square) -> Option<Square>,
        moves: &mut Vec<Movement>,
    ) -> () {
        let mut to: Option<Square> = Some(*from);
        loop {
            to = to.as_ref().and_then(update);
            match to {
                Some(square) => match board.get_square(&square) {
                    None => moves.push(Movement {
                        from: *from,
                        to: square,
                        capture: None,
                        promotion: None,
                    }),
                    Some((piece, color)) => {
                        if color != board.side_to_move {
                            moves.push(Movement {
                                from: *from,
                                to: square,
                                capture: Some(piece),
                                promotion: None,
                            });
                        }
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }
    }
}
