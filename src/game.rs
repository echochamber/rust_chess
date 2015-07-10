

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

	pub fn get_legal_moves_for_piece(piece: ChessPiece, current_pos: &BoardCoordinates) -> Vec<BoardCoordinates> {
		let mut result: Vec<BoardCoordinates> = Vec::new();
		match piece.type_name {
			ChessPieceType::Rook => {
				result.append(&mut ChessGame::get_coordinates_for_row(current_pos.row));
				result.append(&mut ChessGame::get_coordinates_for_col(current_pos.col));
			},
			ChessPieceType::Knight => {
				result.append(&mut ChessGame::get_knight_moves_for_coordinate(current_pos));
			},
			ChessPieceType::Bishop => {
				result.append(&mut ChessGame::get_coordinates_for_diagonals(current_pos));
			},
			ChessPieceType::Queen => {
				result.append(&mut ChessGame::get_coordinates_for_diagonals(current_pos));
				result.append(&mut ChessGame::get_coordinates_for_row(current_pos.row));
				result.append(&mut ChessGame::get_coordinates_for_col(current_pos.col));
			},
			ChessPieceType::King => {
				result.append(&mut ChessGame::get_adjacent_coordinates(current_pos));	
			}
			ChessPieceType::Pawn => {}

		}

		result
	}

	fn get_coordinates_for_row(row: u8) -> Vec<BoardCoordinates> {
		let mut result: Vec<BoardCoordinates> = Vec::new();
		for i in 0..7 {
			result.push(BoardCoordinates {
				row: row,
				col: i
			})
		}

		result
	}

	fn get_coordinates_for_col(col: u8) -> Vec<BoardCoordinates> {
		let mut result: Vec<BoardCoordinates> = Vec::new();
		for i in 0..7 {
			result.push(BoardCoordinates {
				row: i,
				col: col
			})
		}

		result
	}

	fn get_coordinates_for_diagonals(coordinate: &BoardCoordinates) -> Vec<BoardCoordinates> {
		let mut result: Vec<BoardCoordinates> = Vec::new();

		let mut current_col = coordinate.col;
		let mut current_row = coordinate.row;

		while current_col < 7 && current_row < 7 {
			current_row += 1;
			current_col += 1;
			result.push(BoardCoordinates {
				row: current_row,
				col: current_col
			})
		}

		current_col = coordinate.col;
		current_row = coordinate.row;
		while current_col > 0 && current_row > 0 {
			current_row -= 1;
			current_col -= 1;
			result.push(BoardCoordinates {
				row: current_row,
				col: current_col
			})
		}

		current_col = coordinate.col;
		current_row = coordinate.row;
		while current_col > 0 && current_row < 7 {
			current_row += 1;
			current_col -= 1;
			result.push(BoardCoordinates {
				row: current_row,
				col: current_col
			})
		}

		current_col = coordinate.col;
		current_row = coordinate.row;
		while current_col < 7 && current_row > 0 {
			current_row -= 1;
			current_col += 1;
			result.push(BoardCoordinates {
				row: current_row,
				col: current_col
			})
		}

		result
	}

	fn get_adjacent_coordinates(coordinate: &BoardCoordinates) -> Vec<BoardCoordinates> {
		let mut result: Vec<BoardCoordinates> = Vec::new();
		for &(col, row) in &[
			(coordinate.col - 1, coordinate.row - 1),
			(coordinate.col - 1, coordinate.row + 1),
			(coordinate.col - 1, coordinate.row),
			(coordinate.col + 1, coordinate.row - 1),
			(coordinate.col + 1, coordinate.row + 1),
			(coordinate.col + 1, coordinate.row),
			(coordinate.col, coordinate.row - 1),
			(coordinate.col, coordinate.row + 1)
		] {
			if (col <= 7 && col >= 0 && row <= 7 && row >= 0) {
				result.push(BoardCoordinates {
					row: row,
					col: col
				})
			}
		}

		result
	}

	fn get_knight_moves_for_coordinate(coordinate: &BoardCoordinates) -> Vec<BoardCoordinates> {
		let mut result: Vec<BoardCoordinates> = Vec::new();
		for &(col, row) in &[
			(coordinate.col - 2, coordinate.row - 1),
			(coordinate.col - 2, coordinate.row + 1),
			(coordinate.col + 2, coordinate.row - 1),
			(coordinate.col + 2, coordinate.row + 1),
			(coordinate.col - 1, coordinate.row - 2),
			(coordinate.col - 1, coordinate.row + 2),
			(coordinate.col + 1, coordinate.row - 2),
			(coordinate.col + 1, coordinate.row + 2)
		] {
			if (col <= 7 && col >= 0 && row <= 7 && row >= 0) {
				result.push(BoardCoordinates {
					row: row,
					col: col
				})
			}
		}

		result
	}
}