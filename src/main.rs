#![feature(append)]

extern crate rand;
extern crate piston_window;
extern crate piston;

mod board;
mod game;


use board::{ChessPieceType, ChessPiece, ChessPieceColor, BoardCoordinates, ChessMove};
use game::ChessGame;

fn main() {
	let piece = ChessPiece::new(ChessPieceType::Rook, ChessPieceColor::Black);
	let mut game = ChessGame::new(ChessPieceColor::Black);
	let start_coord: BoardCoordinates =  (0,0).into();
	let end_coord: BoardCoordinates = (0,1).into();
	game.set_contents(Some(piece), &start_coord);
	println!("Moves for piece at space {:?} before moving piece", start_coord);
	println!("{:#?}", game.get_legal_moves(&start_coord));

	game.move_piece(&ChessMove::new(&start_coord, &end_coord));

	println!("Moves for piece at space {:?} before moving piece", end_coord);
	println!("{:#?}", game.get_legal_moves(&end_coord));

	//println!("{:#?}", game.get_legal_moves(&piece, &BoardCoordinates {row: 0, col:0}));
}
