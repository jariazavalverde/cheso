// Square representation.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square {
    pub rank: isize,
    pub file: isize,
}

impl Square {
    // Check if square is on board.
    pub fn on_board(&self) -> bool {
        return self.rank >= 1 && self.rank <= 8 && self.file >= 1 && self.file <= 8;
    }
}
