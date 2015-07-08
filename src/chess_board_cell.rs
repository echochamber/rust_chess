
use chess_piece::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessBoardCell<T> {
	col: u8,
	row: u8,
	contents: Option<T>
}

#[allow(dead_code)]
impl ChessBoardCell<ChessPiece> {
	pub fn from_string<T>(cell_name: String) -> ChessBoardCell<T> {
		ChessBoardCell {
			col: ChessBoardCell::char_to_column(
				match cell_name.chars().nth(0) {
					Some(v) => v,
					None => panic!("String must be at least 2 characters long")
				}
			),
			row: ChessBoardCell::char_to_row(
				match cell_name.chars().nth(1) {
					Some(v) => v,
					None => panic!("String must be at least 2 characters long")
				}
			),
			contents: None
		}
	}

	pub fn from_string_with_contents<T>(cell_name: String, contents: Option<T>) -> ChessBoardCell<T> {
		if cell_name.len() > 2 {
			panic!("String must be exactly 2 characters long.");
		}
		
		ChessBoardCell {
			col: ChessBoardCell::char_to_column(
				match cell_name.chars().nth(0) {
					Some(v) => v,
					None => panic!("String must be at least 2 characters long")
				}
			),
			row: ChessBoardCell::char_to_row(
				match cell_name.chars().nth(1) {
					Some(v) => v,
					None => panic!("String must be at least 2 characters long")
				}
			),
			contents: contents
		}
	}

	pub fn coordinates_to_string(&self) -> String {
		match String::from_utf8(vec![self.col + 'a' as u8, self.row + '1' as u8]) {
			Ok(v) => v,
			Err(_) => panic!("Coordinates were not valid utf8: {}, {}", self.col, self.row)
		}
	}

	fn char_to_column(col: char) -> u8 {

		match col {
			input @ 'a' ... 'h' => input as u8 - ('a' as u8),
    		_  => panic!("Invalid column name: {}.", col)
		}
	}

	fn char_to_row(row: char) -> u8 {
		match row {
			input @ '1' ... '8' => input as u8 - ('1' as u8),
    		_  => panic!("Invalid row number: {}.", row)
		}
	}
}

#[allow(dead_code)]
impl<T> ToString for ChessBoardCell<T> {
	fn to_string(&self) -> String {
		"fefrf".to_string()
	}
}