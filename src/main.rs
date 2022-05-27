mod chess;

use chess::color::Color;
use chess::piece::Piece;

fn main() {
    let wk: Piece = Piece::Knight(Color::White);
    println!("{}", wk);
}
