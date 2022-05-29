use crate::cheso::color::Color;
use crate::cheso::movement::{Movement, KING_TRANSLATIONS, KNIGHT_TRANSLATIONS};
use crate::cheso::piece::Piece;
use crate::cheso::square::Square;
use std::collections::HashMap;

/// Board representation.
/// Pieces are stored as maps from squares to pieces (each color in its own hashmap).
pub struct Board {
    pub white_pieces: HashMap<Square, Piece>,
    pub black_pieces: HashMap<Square, Piece>,
    pub side_to_move: Color,
    pub white_kingside_castling: bool,
    pub white_queenside_castling: bool,
    pub black_kingside_castling: bool,
    pub black_queenside_castling: bool,
    pub en_passant: u8,
    pub reversible_moves: u8,
}

impl Board {
    /// Make an initial board.
    pub fn new() -> Board {
        let mut white_pieces: HashMap<Square, Piece> = HashMap::new();
        let mut black_pieces: HashMap<Square, Piece> = HashMap::new();
        white_pieces.insert(Square { file: 1, rank: 1 }, Piece::Rook);
        white_pieces.insert(Square { file: 2, rank: 1 }, Piece::Knight);
        white_pieces.insert(Square { file: 3, rank: 1 }, Piece::Bishop);
        white_pieces.insert(Square { file: 4, rank: 1 }, Piece::Queen);
        white_pieces.insert(Square { file: 5, rank: 1 }, Piece::King);
        white_pieces.insert(Square { file: 6, rank: 1 }, Piece::Bishop);
        white_pieces.insert(Square { file: 7, rank: 1 }, Piece::Knight);
        white_pieces.insert(Square { file: 8, rank: 1 }, Piece::Rook);
        white_pieces.insert(Square { file: 1, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 2, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 3, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 4, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 5, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 6, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 7, rank: 2 }, Piece::Pawn);
        white_pieces.insert(Square { file: 8, rank: 2 }, Piece::Pawn);
        black_pieces.insert(Square { file: 1, rank: 8 }, Piece::Rook);
        black_pieces.insert(Square { file: 2, rank: 8 }, Piece::Knight);
        black_pieces.insert(Square { file: 3, rank: 8 }, Piece::Bishop);
        black_pieces.insert(Square { file: 4, rank: 8 }, Piece::Queen);
        black_pieces.insert(Square { file: 5, rank: 8 }, Piece::King);
        black_pieces.insert(Square { file: 6, rank: 8 }, Piece::Bishop);
        black_pieces.insert(Square { file: 7, rank: 8 }, Piece::Knight);
        black_pieces.insert(Square { file: 8, rank: 8 }, Piece::Rook);
        black_pieces.insert(Square { file: 1, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 2, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 3, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 4, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 5, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 6, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 7, rank: 7 }, Piece::Pawn);
        black_pieces.insert(Square { file: 8, rank: 7 }, Piece::Pawn);
        Board {
            white_pieces: white_pieces,
            black_pieces: black_pieces,
            side_to_move: Color::White,
            white_kingside_castling: true,
            white_queenside_castling: true,
            black_kingside_castling: true,
            black_queenside_castling: true,
            en_passant: 0,
            reversible_moves: 0,
        }
    }

    /// Get the possible piece (and its color) in a square of the current position.
    pub fn get_square(&self, square: &Square) -> Option<(Piece, Color)> {
        match self.white_pieces.get(square) {
            None => match self.black_pieces.get(&square) {
                None => None,
                Some(piece) => Some((*piece, Color::Black)),
            },
            Some(piece) => Some((*piece, Color::White)),
        }
    }

    /// Get the list of pieces to move in the current position.
    pub fn get_pieces_to_move(&self) -> &HashMap<Square, Piece> {
        match self.side_to_move {
            Color::White => &self.white_pieces,
            Color::Black => &self.black_pieces,
        }
    }

    /// Make a movement in the current position.
    pub fn make_move(&self, movement: &Movement) -> () {}

    // Unmake a movement in the current position.
    pub fn unmake_move(&self, movement: &Movement) -> () {}

    /// Generate the list of pseudo-legal movements.
    /// Pieces obey normal rules of movement, but they are not checked to see if they will leave the king in check.
    pub fn gen_moves(&self) -> Vec<Movement> {
        let mut moves: Vec<Movement> = Vec::new();
        for (square, piece) in self.get_pieces_to_move() {
            match piece {
                Piece::Pawn => self.gen_pawn_moves(square, &mut moves),
                Piece::Knight => self.gen_knight_moves(square, &mut moves),
                Piece::Bishop => self.gen_bishop_moves(square, &mut moves),
                Piece::Rook => self.gen_rook_moves(square, &mut moves),
                Piece::Queen => self.gen_queen_moves(square, &mut moves),
                Piece::King => self.gen_king_moves(square, &mut moves),
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
    fn gen_pawn_moves(&self, from: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    /// Populate the list of movements from a square as a knight.
    ///
    /// Knights move two squares in a horizontal or vertical direction, then move one square horizontally or vertically.
    /// Knights are the only piece able to jump over other pieces.
    fn gen_knight_moves(&self, from: &Square, moves: &mut Vec<Movement>) -> () {
        self.gen_moves_from_array(from, KNIGHT_TRANSLATIONS, moves);
    }

    /// Populate the list of movements from a square as a bishop.
    ///
    /// Bishops move diagonally any number of squares.
    /// Bishops are unable to jump over pieces.
    fn gen_bishop_moves(&self, from: &Square, moves: &mut Vec<Movement>) -> () {
        self.gen_moves_until_collision(from, Square::northeast, moves);
        self.gen_moves_until_collision(from, Square::northwest, moves);
        self.gen_moves_until_collision(from, Square::southeast, moves);
        self.gen_moves_until_collision(from, Square::southwest, moves);
    }

    /// Populate the list of movements from a square as a rook.
    ///
    /// Rooks move horizontally or vertically any number of squares.
    /// Rooks are unable to jump over pieces.
    fn gen_rook_moves(&self, from: &Square, moves: &mut Vec<Movement>) -> () {
        self.gen_moves_until_collision(from, Square::north, moves);
        self.gen_moves_until_collision(from, Square::south, moves);
        self.gen_moves_until_collision(from, Square::east, moves);
        self.gen_moves_until_collision(from, Square::west, moves);
    }

    /// Populate the list of movements from a square as a queen.
    ///
    /// Queens move diagonally, horizontally, or vertically any number of squares.
    /// Queens are unable to jump over pieces.
    fn gen_queen_moves(&self, from: &Square, moves: &mut Vec<Movement>) -> () {
        self.gen_bishop_moves(from, moves);
        self.gen_rook_moves(from, moves);
    }

    /// Populate the list of movements from a square as a king.
    ///
    /// Kings move one square in any direction.
    /// Kings are unable to jump over pieces.
    fn gen_king_moves(&self, from: &Square, moves: &mut Vec<Movement>) -> () {
        self.gen_moves_from_array(from, KING_TRANSLATIONS, moves);
    }

    /// Populate the list of movements from an array of translations.
    ///
    /// For each translation, a movement from the original square to its translation is generated.
    /// If the generated movement is pseudo-legal, it is added to the list of pseudo-legal moves.
    /// This function generates all pseudo-legal moves for knights and kings (except castling).
    fn gen_moves_from_array(
        &self,
        from: &Square,
        translations: &[(isize, isize)],
        moves: &mut Vec<Movement>,
    ) -> () {
        for i in 0..translations.len() {
            let (rank, file) = translations[i];
            let to: Option<Square> = from.translate(rank, file);
            match to {
                Some(square) => match self.get_square(&square) {
                    None => moves.push(Movement {
                        from: *from,
                        to: square,
                        capture: None,
                        promotion: None,
                    }),
                    Some((piece, color)) if color != self.side_to_move => moves.push(Movement {
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
    fn gen_moves_until_collision(
        &self,
        from: &Square,
        update: fn(&Square) -> Option<Square>,
        moves: &mut Vec<Movement>,
    ) -> () {
        let mut to: Option<Square> = Some(*from);
        loop {
            to = to.as_ref().and_then(update);
            match to {
                Some(square) => match self.get_square(&square) {
                    None => moves.push(Movement {
                        from: *from,
                        to: square,
                        capture: None,
                        promotion: None,
                    }),
                    Some((piece, color)) => {
                        if color != self.side_to_move {
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
