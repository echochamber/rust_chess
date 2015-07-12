use board::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ChessGame {
	board: ChessBoard<ChessPiece>,
	colors_directions: HashMap<ChessPieceColor, Direction>
}

#[allow(dead_code)]
impl ChessGame {

	pub fn new(up_color: ChessPieceColor) -> ChessGame {
		let mut game = ChessGame {
			board: ChessBoard::new(8),
			colors_directions: HashMap::new()
		};

		match up_color {
			ChessPieceColor::Black => {
				game.colors_directions.insert(ChessPieceColor::Black, Direction::Up);
				game.colors_directions.insert(ChessPieceColor::White, Direction::Down);
			},
			ChessPieceColor::White => {
				game.colors_directions.insert(ChessPieceColor::White, Direction::Up);
				game.colors_directions.insert(ChessPieceColor::Black, Direction::Down);
			}
		}

		game
	}

	// Temporarily public for testing stuff
	pub fn set_contents(&mut self, piece: Option<ChessPiece>, coordinates: &BoardCoordinates) {
		self.board.set_contents_at_coordinates(coordinates, piece);
	}

	pub fn move_piece(&mut self, chess_move: &ChessMove) {
		self.board.move_contents(&chess_move.start_coordinates, &chess_move.end_coordinates);
	}

	pub fn get_legal_moves(&self, current_pos: &BoardCoordinates) -> Vec<ChessMove> {
		let mut result: Vec<ChessMove> = Vec::new();
		match self.board.get_contents_at_coordinates(current_pos) {
			Ok(Some(piece)) if piece.get_type() == ChessPieceType::Rook => {
				result.append(&mut self.get_vertical_moves(&piece, current_pos));
				result.append(&mut self.get_horzontal_moves(&piece, current_pos));
			},
			Ok(Some(piece)) if piece.get_type() == ChessPieceType::Knight => {
				result.append(&mut self.get_knight_moves(&piece, current_pos));
			},
			Ok(Some(piece)) if piece.get_type() == ChessPieceType::Bishop => {
				result.append(&mut self.get_diagonal_moves(&piece, current_pos));
			},
			Ok(Some(piece)) if piece.get_type() == ChessPieceType::Queen => {
				result.append(&mut self.get_diagonal_moves(&piece, current_pos));
				result.append(&mut self.get_vertical_moves(&piece, current_pos));
				result.append(&mut self.get_horzontal_moves(&piece, current_pos));
			},
			Ok(Some(piece)) if piece.get_type() == ChessPieceType::King => {
				result.append(&mut self.get_adjacent_moves(&piece, current_pos));	
			},
			Ok(Some(piece)) if piece.get_type() == ChessPieceType::Pawn => {
				result.append(&mut self.get_pawn_moves(&piece, current_pos));
			},
			Ok(Some(..)) => { /* Some other piece type I guess */ },
			Ok(None) => { /* Space was empty */ },
			Err(e) => panic!("{}", e)
		}

		result
	}

	fn get_vertical_moves(&self, piece: & ChessPiece, start_coordinates: &BoardCoordinates) -> Vec<ChessMove> {
		let mut result: Vec<ChessMove> = Vec::new();

		let mut vertical_move: i8 = 0;
		
		loop {
			vertical_move += 1;
			match self.get_valid_move(piece, start_coordinates, 0, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		vertical_move = 0;
		loop {
			vertical_move -= 1;
			match self.get_valid_move(piece, start_coordinates, 0, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		result
	}

	fn get_horzontal_moves(&self, piece: & ChessPiece, start_coordinates: &BoardCoordinates) -> Vec<ChessMove> {
		let mut result: Vec<ChessMove> = Vec::new();

		let mut horzontal_move: i8 = 0;

		loop {
			horzontal_move += 1;
			match self.get_valid_move(piece, start_coordinates, horzontal_move, 0) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		loop {
			horzontal_move -= 1;
			match self.get_valid_move(piece, start_coordinates, horzontal_move, 0) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		result
	}

	fn get_diagonal_moves(&self, piece: & ChessPiece, start_coordinates: &BoardCoordinates) -> Vec<ChessMove> {
		let mut result: Vec<ChessMove> = Vec::new();

		let mut horzontal_move: i8 = 0;
		let mut vertical_move: i8 = 0;

		loop {
			horzontal_move += 1;
			vertical_move += 1;
			match self.get_valid_move(piece, start_coordinates, horzontal_move, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		vertical_move = 0;
		loop {
			horzontal_move -= 1;
			vertical_move -= 1;
			match self.get_valid_move(piece, start_coordinates, horzontal_move, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		vertical_move = 0;
		loop {
			horzontal_move += 1;
			vertical_move -= 1;
			match self.get_valid_move(piece, start_coordinates, horzontal_move, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		vertical_move = 0;
		loop {
			horzontal_move -= 1;
			vertical_move += 1;
			match self.get_valid_move(piece, start_coordinates, horzontal_move, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		result
	}

	fn get_adjacent_moves(&self, piece: & ChessPiece, start_coordinates: &BoardCoordinates) -> Vec<ChessMove> {
		let mut result: Vec<ChessMove> = Vec::new();
		for &(horzontal_move, vertical_move) in &[
			(-1i8, -1i8),
			(-1i8, 1i8),
			(-1i8, 0i8),
			(-1i8, -1i8),
			(1i8, 1i8),
			(1i8, 0i8),
			(0i8, -1i8),
			(0i8, 1i8)
		] {
			match self.get_valid_move(piece, start_coordinates, horzontal_move, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => { break; }
			}
		}

		result
	}

	fn get_knight_moves(&self, piece: & ChessPiece, start_coordinates: &BoardCoordinates) -> Vec<ChessMove> {
		let mut result: Vec<ChessMove> = Vec::new();
		for &(horzontal_move, vertical_move) in &[
			(-2i8, -1i8),
			(-2i8, 1i8),
			(2i8, -1i8),
			(2i8, 1i8),
			(-1i8, -2i8),
			(-1i8, 2i8),
			(1i8, -2i8),
			(1i8, 2i8)
		] {
			match self.get_valid_move(piece, start_coordinates, horzontal_move, vertical_move) {
				Some(chess_move) => { result.push(chess_move); },
				None => {}
			}
		}

		result
	}

	fn get_pawn_moves(&self, piece: & ChessPiece, start_coordinates: &BoardCoordinates) -> Vec<ChessMove> {

		let mut result: Vec<ChessMove> = Vec::new();

		let vertical_move: i8 = match self.colors_directions.get(&piece.get_color()) {
			Some(&Direction::Up) => { -1i8 },
			Some(&Direction::Down) => { 1i8 },
			_ => { panic!("Colors to directions hasmap not initialized.") }
		};

		// Check forward move
		match self.board.get_move_destination(start_coordinates, 0, vertical_move) {
			Some(board_cell) if board_cell.get_contents().is_none()  => {
				result.push(
					ChessMove::new(start_coordinates, board_cell.get_coordinates())
				);
			},
			_ => {}
		}

		// Result.len() == 1 means there is no piece directly in front of the pawn
		let pawn_is_in_starting_position = true;
		if pawn_is_in_starting_position && result.len() == 1 {
		   match self.board.get_move_destination(start_coordinates, 0, vertical_move * 2) {
				Some(board_cell) if board_cell.is_empty() => {
					result.push(
						ChessMove::new(start_coordinates, board_cell.get_coordinates())
					)
				},
				_ => {}
			}
		}

		for &horzontal_move in &[-1i8, 1i8] {
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) if board_cell.contains_piece_of_color(piece.get_color()) => {
					result.push(
						ChessMove::new(start_coordinates, board_cell.get_coordinates())
					);
				},
				_ => {}
			}
		}

		result
	}

	fn get_valid_move(
		&self,
		piece: & ChessPiece,
		start_coordinates: &BoardCoordinates,
		horzontal_move: i8,
		vertical_move: i8
	) -> Option<ChessMove> {
		match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
			Some(board_cell) if !board_cell.contains_piece_of_color(piece.get_color()) => {
				Some(ChessMove::new(start_coordinates, board_cell.get_coordinates()))
			},
			_ => { None }
		}
	}
}