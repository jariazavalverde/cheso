use crate::chess::color::Color;
use crate::chess::piece::Piece;
use crate::chess::square::Square;
use std::collections::HashMap;
use std::vec::Vec;

#[macro_export]
macro_rules! moves_until_collision {
    ( $board:expr, $square:expr, $candidates:expr, $moves:expr ) => {
        for square_to in $candidates {
            match $board.get_square(&square_to) {
                None => $moves.push(($square, square_to, None)),
                Some((_, color)) if color != $board.side_to_move => {
                    $moves.push(($square, square_to, None));
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    };
}

// Movement representation (from, to, promotion).
type Movement = (Square, Square, Option<Piece>);

// Knight moves.
static KNIGHT_MOVES: &'static [(isize, isize)] =
    &[(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1)];

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
        white_pieces.insert(Square { x: 1, y: 1 }, Piece::Rook);
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

    // Populate the list of pawn movements.
    fn pawn_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    // Populate the list of knight movements.
    fn knight_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        for i in 0..KNIGHT_MOVES.len() {
            let square_to: Square = Square {
                x: square.x + KNIGHT_MOVES[i].0,
                y: square.y + KNIGHT_MOVES[i].1,
            };
            if square_to.on_board() {
                match self.get_square(square) {
                    None => moves.push((*square, square_to, None)),
                    Some((piece, color)) if color != self.side_to_move => {
                        moves.push((*square, square_to, None))
                    }
                    _ => (),
                }
            }
        }
    }

    // Populate the list of bishop movements.
    fn bishop_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        // south-west
        // north-east
    }

    // Populate the list of rook movements.
    fn rook_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        // south
        let south = (1..square.y)
            .rev()
            .map(|y: isize| Square { x: square.x, y: y });
        moves_until_collision!(self, *square, south, moves);
        // north
        let north = (square.y + 1..=8).map(|y: isize| Square { x: square.x, y: y });
        moves_until_collision!(self, *square, north, moves);
        // east
        let east = (1..square.x)
            .rev()
            .map(|x: isize| Square { x: x, y: square.y });
        moves_until_collision!(self, *square, east, moves);
        // west
        let west = (square.x + 1..=8).map(|x: isize| Square { x: x, y: square.y });
        moves_until_collision!(self, *square, west, moves);
    }

    // Populate the list of queen movements.
    fn queen_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        self.bishop_moves(square, moves);
        self.rook_moves(square, moves);
    }

    // Populate the list of king movements.
    fn king_moves(&self, square: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }
}
