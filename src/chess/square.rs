use std::vec::Vec;

// Knight moves.
static KNIGHT_MOVES: &'static [(isize, isize)] =
    &[(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1)];

// King moves.
static KING_MOVES: &'static [(isize, isize)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

// Square representation.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square {
    pub rank: isize,
    pub file: isize,
}

impl Square {
    // Check if square is on board.
    pub fn on_board(&self) -> bool {
        self.rank >= 1 && self.rank <= 8 && self.file >= 1 && self.file <= 8
    }

    // Symmetric rank square.
    pub fn sym_rank(&self) -> Square {
        Square {
            rank: 9 - self.rank,
            file: self.file,
        }
    }

    // Symmetric file square.
    pub fn sym_file(&self) -> Square {
        Square {
            rank: self.rank,
            file: 9 - self.file,
        }
    }

    // Minimum between rank and file.
    pub fn min_rank_file(&self) -> isize {
        std::cmp::min(self.rank, self.file)
    }

    // Maximum between rank and file.
    pub fn max_rank_file(&self) -> isize {
        std::cmp::max(self.rank, self.file)
    }

    // Get L-shape squares.
    pub fn get_l_squares(&self) -> Vec<Square> {
        (0..KNIGHT_MOVES.len())
            .map(|i: usize| Square {
                rank: self.rank + KNIGHT_MOVES[i].0,
                file: self.file + KNIGHT_MOVES[i].1,
            })
            .filter(|square: &Square| square.on_board())
            .collect()
    }

    // Get squares around.
    pub fn get_squares_around(&self) -> Vec<Square> {
        (0..KING_MOVES.len())
            .map(|i: usize| Square {
                rank: self.rank + KNIGHT_MOVES[i].0,
                file: self.file + KNIGHT_MOVES[i].1,
            })
            .filter(|square: &Square| square.on_board())
            .collect()
    }

    // Get south squares.
    pub fn get_south_squares(&self) -> Vec<Square> {
        (1..self.rank)
            .rev()
            .map(|rank: isize| Square {
                rank,
                file: self.file,
            })
            .collect()
    }

    // Get north squares.
    pub fn get_north_squares(&self) -> Vec<Square> {
        (self.rank + 1..=8)
            .map(|rank: isize| Square {
                rank,
                file: self.file,
            })
            .collect()
    }

    // Get west squares.
    pub fn get_west_squares(&self) -> Vec<Square> {
        (1..self.file)
            .rev()
            .map(|file: isize| Square {
                rank: self.rank,
                file,
            })
            .collect()
    }

    // Get east squares.
    pub fn get_east_squares(&self) -> Vec<Square> {
        (self.file + 1..=8)
            .map(|file: isize| Square {
                rank: self.rank,
                file,
            })
            .collect()
    }

    // Get northeast squares.
    pub fn get_northeast_squares(&self) -> Vec<Square> {
        (1..=8 - self.max_rank_file())
            .map(|incr: isize| Square {
                rank: self.rank + incr,
                file: self.file + incr,
            })
            .collect()
    }

    // Get northwest squares.
    pub fn get_northwest_squares(&self) -> Vec<Square> {
        (1..=8 - self.sym_file().max_rank_file())
            .map(|incr: isize| Square {
                rank: self.rank + incr,
                file: self.file - incr,
            })
            .collect()
    }

    // Get southeast squares.
    pub fn get_southeast_squares(&self) -> Vec<Square> {
        (1..self.sym_file().min_rank_file())
            .map(|incr: isize| Square {
                rank: self.rank - incr,
                file: self.file + incr,
            })
            .collect()
    }

    // Get southwest squares.
    pub fn get_southwest_squares(&self) -> Vec<Square> {
        (1..self.min_rank_file())
            .map(|incr: isize| Square {
                rank: self.rank - incr,
                file: self.file - incr,
            })
            .collect()
    }
}
