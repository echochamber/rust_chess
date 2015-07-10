#![feature(append)]

extern crate rand;
extern crate piston_window;
extern crate piston;

mod board;
mod game;


use board::{ChessBoardCell, ChessPieceType, ChessPiece, ChessPieceColor, ChessBoard, BoardCoordinates};
use game::ChessGame;

fn main() {
	let piece = ChessPiece { type_name: ChessPieceType::Rook, color: ChessPieceColor::Black};
	let some_board = ChessBoardCell::from_string_with_contents("g5".to_string(), Some(ChessPieceType::Rook));
	let my_board = ChessBoard::new(8);
	let game = ChessGame::new(my_board);
	println!("{:?}", ChessGame::get_legal_moves_for_piece(piece, &BoardCoordinates {row: 0, col:0}));
    println!("Hello, world! {:#?}", some_board.coordinates_to_string());
}
