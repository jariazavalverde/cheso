use std::vec::Vec;

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

    // Displace the square.
    pub fn displace(&self, rank_offset: isize, file_offset: isize) -> Square {
        Square {
            rank: self.rank + rank_offset,
            file: self.file + file_offset,
        }
    }

    // Get south square.
    pub fn south(&self) -> Square {
        self.displace(-1, 0)
    }

    // Get north square.
    pub fn north(&self) -> Square {
        self.displace(1, 0)
    }

    // Get west square.
    pub fn west(&self) -> Square {
        self.displace(0, -1)
    }

    // Get east square.
    pub fn east(&self) -> Square {
        self.displace(0, 1)
    }

    // Get northeast square.
    pub fn northeast(&self) -> Square {
        self.displace(1, 1)
    }

    // Get northwest square.
    pub fn northwest(&self) -> Square {
        self.displace(1, -1)
    }

    // Get southeast square.
    pub fn southeast(&self) -> Square {
        self.displace(-1, 1)
    }

    // Get southwest square.
    pub fn southwest(&self) -> Square {
        self.displace(-1, -1)
    }
}
