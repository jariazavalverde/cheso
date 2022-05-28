use crate::chess::color::Color;
use crate::chess::piece::Piece;
use crate::chess::square::Square;
use std::vec::Vec;

#[macro_export]
macro_rules! moves_until_collision {
    ( $board:expr, $square:expr, $candidates:expr, $moves:expr ) => {
        for square_to in $candidates {
            let piece: Piece = $board.get_square(square_to);
            match piece.get_color() {
                None => $moves.push(($square, square_to, Piece::Nil)),
                Some(color) => {
                    if color != $board.side_to_move {
                        $moves.push(($square, square_to, Piece::Nil))
                    }
                    break;
                }
            }
        }
    };
}

// Movement representation (from, to, promotion).
type Movement = (Square, Square, Piece);

// Knight moves.
static KNIGHT_MOVES: &'static [(isize, isize)] =
    &[(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1)];

// Board representation.
pub struct Board {
    pub squares: [Piece; 64],
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
        let squares: [Piece; 64] = [
            Piece::Rook(Color::White),
            Piece::Knight(Color::White),
            Piece::Bishop(Color::White),
            Piece::Queen(Color::White),
            Piece::King(Color::White),
            Piece::Bishop(Color::White),
            Piece::Knight(Color::White),
            Piece::Rook(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Pawn(Color::White),
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Nil,
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Pawn(Color::Black),
            Piece::Rook(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Queen(Color::Black),
            Piece::King(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Rook(Color::Black),
        ];
        Board {
            squares: squares,
            side_to_move: Color::White,
            white_kingside_castling: true,
            white_queenside_castling: true,
            black_kingside_castling: true,
            black_queenside_castling: true,
            en_passant: 0,
            reversible_moves: 0,
        }
    }

    // Get square content by indices $(x, y) \in [1,8]^2$.
    pub fn get_square(&self, square: Square) -> Piece {
        let index: isize = (square.y - 1) * 8 + square.x - 1;
        self.squares[index as usize]
    }

    // Generate the list of movements.
    pub fn moves(&self) -> Vec<Movement> {
        let mut moves: Vec<Movement> = Vec::new();
        let side_to_move: Color = self.side_to_move;
        for x in 1..=8 {
            for y in 1..=8 {
                let square: Square = Square { x, y };
                match self.get_square(square) {
                    Piece::Pawn(color) if color == side_to_move => {
                        self.pawn_moves(square, &mut moves)
                    }
                    Piece::Knight(color) if color == side_to_move => {
                        self.knight_moves(square, &mut moves)
                    }
                    Piece::Bishop(color) if color == side_to_move => {
                        self.bishop_moves(square, &mut moves)
                    }
                    Piece::Rook(color) if color == side_to_move => {
                        self.rook_moves(square, &mut moves)
                    }
                    Piece::Queen(color) if color == side_to_move => {
                        self.queen_moves(square, &mut moves)
                    }
                    Piece::King(color) if color == side_to_move => {
                        self.king_moves(square, &mut moves)
                    }
                    _ => (),
                }
            }
        }
        moves
    }

    // Populate the list of pawn movements.
    fn pawn_moves(&self, square: Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    // Populate the list of knight movements.
    fn knight_moves(&self, square: Square, moves: &mut Vec<Movement>) -> () {
        for i in 0..KNIGHT_MOVES.len() {
            let square_to: Square = Square {
                x: square.x + KNIGHT_MOVES[i].0,
                y: square.y + KNIGHT_MOVES[i].1,
            };
            if square_to.on_board() {
                let piece: Piece = self.get_square(square);
                if piece == Piece::Nil || piece.get_color() != Some(self.side_to_move) {
                    moves.push((square, square_to, Piece::Nil));
                }
            }
        }
    }

    // Populate the list of bishop movements.
    fn bishop_moves(&self, square: Square, moves: &mut Vec<Movement>) -> () {
        // south-west
        // north-east
    }

    // Populate the list of rook movements.
    fn rook_moves(&self, square: Square, moves: &mut Vec<Movement>) -> () {
        // south
        let south = (1..square.y)
            .rev()
            .map(|y: isize| Square { x: square.x, y: y });
        moves_until_collision!(self, square, south, moves);
        // north
        let north = (square.y + 1..=8).map(|y: isize| Square { x: square.x, y: y });
        moves_until_collision!(self, square, north, moves);
        // east
        let east = (1..square.x)
            .rev()
            .map(|x: isize| Square { x: x, y: square.y });
        moves_until_collision!(self, square, east, moves);
        // west
        let west = (square.x + 1..=8).map(|x: isize| Square { x: x, y: square.y });
        moves_until_collision!(self, square, west, moves);
    }

    // Populate the list of queen movements.
    fn queen_moves(&self, square: Square, moves: &mut Vec<Movement>) -> () {
        self.bishop_moves(square, moves);
        self.rook_moves(square, moves);
    }

    // Populate the list of king movements.
    fn king_moves(&self, square: Square, moves: &mut Vec<Movement>) -> () {
        ()
    }
}
