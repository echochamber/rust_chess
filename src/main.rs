extern crate rand;
extern crate piston_window;
extern crate piston;

mod board;
mod game;


use board::{ChessBoardCell, ChessPieceType, ChessPiece, ChessPieceColor};

fn main() {
	let piece = ChessPiece { type_name: ChessPieceType::Rook, color: ChessPieceColor::Black};
	let some_board = ChessBoardCell::from_string_with_contents("g5".to_string(), Some(ChessPieceType::Rook));
    println!("Hello, world! {:?}", some_board.coordinates_to_string());
}
