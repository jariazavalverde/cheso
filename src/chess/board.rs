use crate::chess::color::Color;
use crate::chess::piece::Piece;
use crate::chess::square::Square;
use std::collections::HashMap;

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

    // Get the possible piece (and its color) in a square of the current position.
    pub fn get_square(&self, square: &Square) -> Option<(Piece, Color)> {
        match self.white_pieces.get(square) {
            None => match self.black_pieces.get(&square) {
                None => None,
                Some(piece) => Some((*piece, Color::Black)),
            },
            Some(piece) => Some((*piece, Color::White)),
        }
    }

    // Get the list of pieces to move in the current position.
    pub fn get_pieces_to_move(&self) -> &HashMap<Square, Piece> {
        match self.side_to_move {
            Color::White => &self.white_pieces,
            Color::Black => &self.black_pieces,
        }
    }
}
