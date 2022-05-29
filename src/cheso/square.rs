use crate::cheso::color::Color;

/// A square of the board.
///
/// This enumeration is used to store pieces in the board and to represent movements.
/// An square must be between rank (row) 1 and 8, and between file (column) 1 and 8.
/// It is impossible to generate an invalid (out of range) square.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square {
    rank: isize,
    file: isize,
}

impl Square {
    /// Create a new square.
    pub fn new(rank: isize, file: isize) -> Square {
        Square {
            rank: std::cmp::max(1, std::cmp::min(8, rank)),
            file: std::cmp::max(1, std::cmp::min(8, file)),
        }
    }

    /// Get the rank of the square.
    pub fn rank(&self) -> isize {
        self.rank
    }

    /// Get the file of the square.
    pub fn file(&self) -> isize {
        self.file
    }

    /// Check if square is on board.
    fn on_board(&self) -> bool {
        self.rank >= 1 && self.rank <= 8 && self.file >= 1 && self.file <= 8
    }

    /// Translate the square in a safe way.
    pub fn translate(&self, rank_offset: isize, file_offset: isize) -> Option<Square> {
        Some(Square {
            rank: self.rank + rank_offset,
            file: self.file + file_offset,
        })
        .filter(Self::on_board)
    }

    /// Check if is the last rank, based on the side to move.
    pub fn is_last_rank(&self, color: Color) -> bool {
        match color {
            Color::White => self.rank == 8,
            Color::Black => self.rank == 1,
        }
    }

    /// Check if is the initial rank for a pawn, based on the side to move.
    pub fn is_pawn_rank(&self, color: Color) -> bool {
        match color {
            Color::White => self.rank == 2,
            Color::Black => self.rank == 7,
        }
    }

    /// Get one forward square, based on the side to move.
    pub fn forward(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.north(),
            Color::Black => self.south(),
        }
    }

    /// Get two forward squares, based on the side to move.
    pub fn forward2(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.translate(2, 0),
            Color::Black => self.translate(-2, 0),
        }
    }

    /// Get the left square capture of a pawn, based on the side to move.
    pub fn pawn_left_capture(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.northwest(),
            Color::Black => self.southwest(),
        }
    }

    /// Get the right square capture of a pawn, based on the side to move.
    pub fn pawn_right_capture(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.northeast(),
            Color::Black => self.southeast(),
        }
    }

    /// Get south square.
    pub fn south(&self) -> Option<Square> {
        self.translate(-1, 0)
    }

    /// Get north square.
    pub fn north(&self) -> Option<Square> {
        self.translate(1, 0)
    }

    /// Get west square.
    pub fn west(&self) -> Option<Square> {
        self.translate(0, -1)
    }

    /// Get east square.
    pub fn east(&self) -> Option<Square> {
        self.translate(0, 1)
    }

    /// Get northeast square.
    pub fn northeast(&self) -> Option<Square> {
        self.translate(1, 1)
    }

    /// Get northwest square.
    pub fn northwest(&self) -> Option<Square> {
        self.translate(1, -1)
    }

    /// Get southeast square.
    pub fn southeast(&self) -> Option<Square> {
        self.translate(-1, 1)
    }

    /// Get southwest square.
    pub fn southwest(&self) -> Option<Square> {
        self.translate(-1, -1)
    }
}
