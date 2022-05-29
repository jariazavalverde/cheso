use std::vec::Vec;

/// A square of the board.
///
/// This enumeration is mainly used to store pieces in the board and to represent movements.
/// The square is supposed to be between rank (row) 1 and 8, and between file (column) 1 and 8.
/// Using the functions of this module makes it impossible to get an invalid square (out of range).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square {
    pub rank: isize,
    pub file: isize,
}

impl Square {
    /// Create a new square.
    pub fn new(rank: isize, file: isize) -> Square {
        Square {
            rank: std::cmp::max(1, std::cmp::min(8, rank)),
            file: std::cmp::max(1, std::cmp::min(8, file)),
        }
    }

    /// Check if square is on board.
    pub fn on_board(&self) -> bool {
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
