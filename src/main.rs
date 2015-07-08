extern crate rand;
extern crate piston_window;
extern crate piston;

mod board;
mod chess_piece;
mod chess_board_cell;

use chess_piece::*;
use chess_board_cell::*;

fn main() {
	let some_board = ChessBoardCell::from_string_with_contents("g5".to_string(), Some(ChessPiece::Rook));
    println!("Hello, world! {:?}", some_board.coordinates_to_string());
}
