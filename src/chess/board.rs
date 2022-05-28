use crate::chess::color::Color;
use crate::chess::piece::Piece;
use std::vec::Vec;

// Square representation.
type Square = (isize, isize);

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

    // Check if square is on board.
    pub fn on_board(square: Square) -> bool {
        let (x, y) = square;
        return x >= 1 && x <= 8 && y >= 1 && y <= 8;
    }

    // Get square content by indices x, y in [1,8].
    pub fn get_square(&self, square: Square) -> Piece {
        let (x, y) = square;
        let index: isize = (y - 1) * 8 + x - 1;
        self.squares[index as usize]
    }

    // Generate the list of movements.
    pub fn moves(&self) -> Vec<Movement> {
        let mut moves: Vec<Movement> = Vec::new();
        let side_to_move: Color = self.side_to_move;
        for x in 1..=8 {
            for y in 1..=8 {
                match self.get_square((x, y)) {
                    Piece::Pawn(color) if color == side_to_move => {
                        self.pawn_moves((x, y), &mut moves)
                    }
                    Piece::Knight(color) if color == side_to_move => {
                        self.knight_moves((x, y), &mut moves)
                    }
                    Piece::Bishop(color) if color == side_to_move => {
                        self.bishop_moves((x, y), &mut moves)
                    }
                    Piece::Rook(color) if color == side_to_move => {
                        self.rook_moves((x, y), &mut moves)
                    }
                    Piece::Queen(color) if color == side_to_move => {
                        self.queen_moves((x, y), &mut moves)
                    }
                    Piece::King(color) if color == side_to_move => {
                        self.king_moves((x, y), &mut moves)
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
            let x: isize = square.0 + KNIGHT_MOVES[i].0;
            let y: isize = square.1 + KNIGHT_MOVES[i].1;
            let square_to: Square = (x, y);
            if Board::on_board(square_to) {
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
        let (x, y) = square;
        // vertical
        for j in 1..=8 {
            if y != j {
                let square_to: Square = (x, j);
                let piece: Piece = self.get_square(square_to);
                match piece.get_color() {
                    None => moves.push((square, square_to, Piece::Nil)),
                    Some(color) => {
                        if color != self.side_to_move {
                            moves.push((square, square_to, Piece::Nil))
                        }
                        break;
                    }
                }
            }
        }
        // horizontal
        for i in 1..=8 {
            if x != i {
                let square_to: Square = (i, y);
                let piece: Piece = self.get_square(square_to);
                match piece.get_color() {
                    None => moves.push((square, square_to, Piece::Nil)),
                    Some(color) => {
                        if color != self.side_to_move {
                            moves.push((square, square_to, Piece::Nil))
                        }
                        break;
                    }
                }
            }
        }
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
