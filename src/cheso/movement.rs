use crate::cheso::board::Board;
use crate::cheso::piece::Piece;
use crate::cheso::square::Square;

// Knight movements.
static KNIGHT_MOVES: &'static [(isize, isize)] =
    &[(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1)];

// King movements.
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

// Movement representation.
#[derive(Copy, Clone)]
struct Movement {
    from: Square,
    to: Square,
    promotion: Option<Piece>,
}

impl Movement {
    // Generate the list of movements.
    pub fn generate(board: &Board) -> Vec<Movement> {
        let mut moves: Vec<Movement> = Vec::new();
        for (square, piece) in board.get_pieces_to_move() {
            match piece {
                Piece::Pawn => Self::as_pawn(board, square, &mut moves),
                Piece::Knight => Self::as_knight(board, square, &mut moves),
                Piece::Bishop => Self::as_bishop(board, square, &mut moves),
                Piece::Rook => Self::as_rook(board, square, &mut moves),
                Piece::Queen => Self::as_queen(board, square, &mut moves),
                Piece::King => Self::as_king(board, square, &mut moves),
            }
        }
        moves
    }

    // Check if a movement has collision.
    pub fn has_collision(&self, board: &Board) -> bool {
        match board.get_square(&self.to) {
            None => false,
            Some(_) => true,
        }
    }

    // Check if a movement is valid (only for the two squares involved).
    pub fn is_valid(&self, board: &Board) -> bool {
        match board.get_square(&self.to) {
            Some((_, color)) if color == board.side_to_move => false,
            _ => true,
        }
    }

    // Populate the list of movements from a square as a pawn.
    fn as_pawn(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        ()
    }

    // Populate the list of movements from a square as a knight.
    fn as_knight(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::from_array(board, from, KNIGHT_MOVES, moves);
    }

    // Populate the list of movements from a square as a bishop.
    fn as_bishop(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        // northeast
        Self::until_collision(board, from, |square| square.northeast(), moves);
        // northwest
        Self::until_collision(board, from, |square| square.northwest(), moves);
        // southeast
        Self::until_collision(board, from, |square| square.southeast(), moves);
        // southwest
        Self::until_collision(board, from, |square| square.southwest(), moves);
    }

    // Populate the list of movements from a square as a rook.
    fn as_rook(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        // north
        Self::until_collision(board, from, |square| square.north(), moves);
        // south
        Self::until_collision(board, from, |square| square.south(), moves);
        // east
        Self::until_collision(board, from, |square| square.east(), moves);
        // west
        Self::until_collision(board, from, |square| square.west(), moves);
    }

    // Populate the list of movements from a square as a queen.
    fn as_queen(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::as_bishop(board, from, moves);
        Self::as_rook(board, from, moves);
    }

    // Populate the list of movements from a square as a king.
    fn as_king(board: &Board, from: &Square, moves: &mut Vec<Movement>) -> () {
        Self::from_array(board, from, KING_MOVES, moves);
    }

    // Populate the list of movements from an array of displacements.
    fn from_array(
        board: &Board,
        from: &Square,
        displacements: &[(isize, isize)],
        moves: &mut Vec<Movement>,
    ) -> () {
        for i in 0..displacements.len() {
            let (rank, file) = displacements[i];
            let to: Square = from.displace(rank, file);
            if to.on_board() {
                let movement: Movement = Movement {
                    from: *from,
                    to: to,
                    promotion: None,
                };
                if movement.is_valid(board) {
                    moves.push(movement);
                }
            }
        }
    }

    // Populate the list of movements until collision with another piece.
    fn until_collision(
        board: &Board,
        from: &Square,
        update: fn(&Square) -> Square,
        moves: &mut Vec<Movement>,
    ) -> () {
        let mut to: Square = *from;
        loop {
            to = update(&to);
            if !to.on_board() {
                break;
            }
            let movement: Movement = Movement {
                from: *from,
                to: to,
                promotion: None,
            };
            if movement.is_valid(board) {
                moves.push(movement);
            }
            if movement.has_collision(board) {
                break;
            }
        }
    }
}
