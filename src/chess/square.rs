// Square representation.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square {
    pub x: isize,
    pub y: isize,
}

impl Square {
    // Check if square is on board.
    pub fn on_board(&self) -> bool {
        return self.x >= 1 && self.x <= 8 && self.y >= 1 && self.y <= 8;
    }
}
