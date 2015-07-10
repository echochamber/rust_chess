

use board::ChessPiece;
use board::ChessPieceType;
use board::ChessBoard;
use board::BoardCoordinates;

#[allow(dead_code)]
pub struct ChessGame {
	board: ChessBoard<ChessPiece>
}

#[allow(dead_code)]
impl ChessGame {

	pub fn new(board: ChessBoard<ChessPiece>) -> ChessGame {
		ChessGame {
			board: board
		}
	}

	pub fn get_legal_moves_for_piece(&self, piece: ChessPiece, current_pos: &BoardCoordinates) -> Vec<&BoardCoordinates> {
		let mut result: Vec<&BoardCoordinates> = Vec::new();
		match piece.type_name {
			ChessPieceType::Rook => {
				result.append(&mut self.get_coordinates_for_row(current_pos));
				result.append(&mut self.get_coordinates_for_col(current_pos));
			},
			ChessPieceType::Knight => {
				result.append(&mut self.get_knight_moves_for_coordinate(current_pos));
			},
			ChessPieceType::Bishop => {
				result.append(&mut self.get_coordinates_for_diagonals(current_pos));
			},
			ChessPieceType::Queen => {
				result.append(&mut self.get_coordinates_for_diagonals(current_pos));
				result.append(&mut self.get_coordinates_for_row(current_pos));
				result.append(&mut self.get_coordinates_for_col(current_pos));
			},
			ChessPieceType::King => {
				result.append(&mut self.get_adjacent_coordinates(current_pos));	
			}
			ChessPieceType::Pawn => {}
		}

		result
	}

	fn get_coordinates_for_row(&self, start_coordinates: &BoardCoordinates) -> Vec<&BoardCoordinates> {
		let mut result: Vec<&BoardCoordinates> = Vec::new();

		let mut vertical_move: i8 = 0;
		
		loop {
			vertical_move += 1;
			match self.board.get_move_destination(start_coordinates, 0, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		vertical_move = 0;
		loop {
			vertical_move -= 1;
			match self.board.get_move_destination(start_coordinates, 0, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		result
	}

	fn get_coordinates_for_col(&self, start_coordinates: &BoardCoordinates) -> Vec<&BoardCoordinates> {
		let mut result: Vec<&BoardCoordinates> = Vec::new();

		let mut horzontal_move: i8 = 0;

		loop {
			horzontal_move += 1;
			match self.board.get_move_destination(start_coordinates, horzontal_move, 0) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		loop {
			horzontal_move -= 1;
			match self.board.get_move_destination(start_coordinates, horzontal_move, 0) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		result
	}

	fn get_coordinates_for_diagonals(&self, start_coordinates: &BoardCoordinates) -> Vec<&BoardCoordinates> {
		let mut result: Vec<&BoardCoordinates> = Vec::new();

		let mut horzontal_move: i8 = 0;
		let mut vertical_move: i8 = 0;

		loop {
			horzontal_move += 1;
			vertical_move += 1;
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		vertical_move = 0;
		loop {
			horzontal_move -= 1;
			vertical_move -= 1;
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		vertical_move = 0;
		loop {
			horzontal_move += 1;
			vertical_move -= 1;
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		horzontal_move = 0;
		vertical_move = 0;
		loop {
			horzontal_move -= 1;
			vertical_move += 1;
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => { break; }
			}
		}

		result
	}

	fn get_adjacent_coordinates(&self, start_coordinates: &BoardCoordinates) -> Vec<&BoardCoordinates> {
		let mut result: Vec<&BoardCoordinates> = Vec::new();
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
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => {}
			}
		}

		result
	}

	fn get_knight_moves_for_coordinate(&self, start_coordinates: &BoardCoordinates) -> Vec<&BoardCoordinates> {
		let mut result: Vec<&BoardCoordinates> = Vec::new();
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
			match self.board.get_move_destination(start_coordinates, horzontal_move, vertical_move) {
				Some(board_cell) => { result.push(board_cell.get_coordinates()); },
				None => {}
			}
		}

		result
	}
}