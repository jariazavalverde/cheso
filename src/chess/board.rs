use crate::chess::color::Color;
use crate::chess::piece::Piece;
use crate::chess::square::Square;
use std::collections::HashMap;
use std::vec::Vec;

// Movement representation (from, to, promotion).
type Movement = (Square, Square, Option<Piece>);

// Board representation.
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
    // Make an initial board.
    pub fn init() -> Board {
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

    // Get square piece.
    pub fn get_square(&self, square: &Square) -> Option<(Piece, Color)> {
        match self.white_pieces.get(square) {
            None => match self.black_pieces.get(&square) {
                None => None,
                Some(piece) => Some((*piece, Color::Black)),
            },
            Some(piece) => Some((*piece, Color::White)),
        }
    }

    // Get pieces to move.
    pub fn get_pieces_to_move(&self) -> &HashMap<Square, Piece> {
        match self.side_to_move {
            Color::White => &self.white_pieces,
            Color::Black => &self.black_pieces,
        }
    }

    // Generate the list of movements.
    pub fn moves(&self) -> Vec<Movement> {
        let mut moves: Vec<Movement> = Vec::new();
        let pieces_to_move: &HashMap<Square, Piece> = self.get_pieces_to_move();
        for (square, piece) in pieces_to_move {
            match piece {
                Piece::Pawn => self.pawn_moves(square, &mut moves),
                Piece::Knight => self.knight_moves(square, &mut moves),
                Piece::Bishop => self.bishop_moves(square, &mut moves),
                Piece::Rook => self.rook_moves(square, &mut moves),
                Piece::Queen => self.queen_moves(square, &mut moves),
                Piece::King => self.king_moves(square, &mut moves),
            }
        }
        moves
    }

    // Populate the list with pawn movements.
    fn pawn_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    // Populate the list with knight movements.
    fn knight_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        // l-movements
        self.moves_until_collision(square, square.get_l_squares(), true, moves);
    }

    // Populate the list with bishop movements.
    fn bishop_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        // northeast
        self.moves_until_collision(square, square.get_northeast_squares(), false, moves);
        // northwest
        self.moves_until_collision(square, square.get_northwest_squares(), false, moves);
        // southeast
        self.moves_until_collision(square, square.get_southeast_squares(), false, moves);
        // southwest
        self.moves_until_collision(square, square.get_southwest_squares(), false, moves);
    }

    // Populate the list with rook movements.
    fn rook_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        // north
        self.moves_until_collision(square, square.get_north_squares(), false, moves);
        // south
        self.moves_until_collision(square, square.get_south_squares(), false, moves);
        // east
        self.moves_until_collision(square, square.get_east_squares(), false, moves);
        // west
        self.moves_until_collision(square, square.get_west_squares(), false, moves);
    }

    // Populate the list with queen movements.
    fn queen_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        self.bishop_moves(square, moves);
        self.rook_moves(square, moves);
    }

    // Populate the list with king movements.
    fn king_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    // Populate the list with movements until collision with another piece.
    fn moves_until_collision(
        &self,
        square: &Square,
        candidates: Vec<Square>,
        jump: bool,
        moves: &mut Vec<Movement>,
    ) -> () {
        for i in 0..candidates.len() {
            let square_to: Square = candidates[i];
            match self.get_square(&square_to) {
                None => moves.push((*square, square_to, None)),
                Some((_, color)) if color != self.side_to_move => {
                    moves.push((*square, square_to, None));
                    if !jump {
                        break;
                    }
                }
                _ => {
                    if !jump {
                        break;
                    }
                }
            }
        }
    }
}
