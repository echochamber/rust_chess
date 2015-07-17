#![feature(append)]

extern crate rand;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate sprite;
extern crate gfx_device_gl;
extern crate piston;

mod board;
mod game;
mod view;


use board::{ChessPieceType, ChessPiece, ChessPieceColor, BoardCoordinates, ChessMove};
use game::ChessGame;
use piston_window::*;
use view::Renderable;


fn main() {
	let piece = ChessPiece::new(ChessPieceType::Rook, ChessPieceColor::Black);
	let mut game = ChessGame::new(ChessPieceColor::Black);
	let start_coord: BoardCoordinates =  (0,0).into();
	let end_coord: BoardCoordinates = (0,1).into();
	game.set_contents(Some(piece), &start_coord);
	//println!("Moves for piece at space {:?} before moving piece", start_coord);
	//println!("{:#?}", game.get_legal_moves(&start_coord));

	game.move_piece(&ChessMove::new(&start_coord, &end_coord));

	//println!("Moves for piece at space {:?} before moving piece", end_coord);
	//println!("{:#?}", game.get_legal_moves(&end_coord));

	game.initialize_pieces();
	//println!("{:#?}", game.get_legal_moves(&end_coord));

	//println!("{:#?}", game.get_legal_moves(&piece, &BoardCoordinates {row: 0, col:0}));
	
	let (width, height) = (1280, 720);
    let opengl = OpenGL::V3_2;
    let window: PistonWindow =
        WindowSettings::new("piston: sprite", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

	for e in window {
		e.draw_2d(|c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
        });

        board::draw_chess_piece_sprite(&e, &piece, (300f64, 300f64));
    	//game.draw(&e);
	}
}
