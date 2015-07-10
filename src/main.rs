#![feature(append)]

extern crate rand;
extern crate piston_window;
extern crate piston;

mod board;
mod game;


use board::{ChessPieceType, ChessPiece, ChessPieceColor, ChessBoard, BoardCoordinates};
use game::ChessGame;

fn main() {
	let piece = ChessPiece { type_name: ChessPieceType::Rook, color: ChessPieceColor::Black};
	let my_board = ChessBoard::new(8);
	let game = ChessGame::new(my_board);
	println!("{:#?}", game.get_legal_moves_for_piece(piece, &BoardCoordinates {row: 0, col:0}));
}
