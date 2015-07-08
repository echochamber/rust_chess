#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessBoard<T> {
	columns: [[ChessBoardCell<T>; 8]; 8]
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ChessPieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ChessPieceColor {
	Black,
	White
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessPiece {
	pub type_name: ChessPieceType,
	pub color: ChessPieceColor
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct BoardCoordinates {
	pub row: u8,
	pub col: u8
}


#[allow(dead_code)]
impl ToString for BoardCoordinates {
	fn to_string(&self) -> String {
		match String::from_utf8(vec![self.col + 'a' as u8, self.row + '1' as u8]) {
			Ok(v) => v,
			Err(_) => panic!("Coordinates were not valid utf8: {}, {}", self.col, self.row)
		}
	}
}

#[allow(dead_code)]
impl BoardCoordinates {
	// Convert a character to the correct column # ('a'=0, 'b'=1, ...)
	fn char_to_column(col: char) -> u8 {
		match col {
			input @ 'a' ... 'h' => input as u8 - ('a' as u8),
			_  => panic!("Invalid column name: {}.", col)
		}
	}

	// Convert a character to the correct row # ('1'=0, '2'=1, ...)
	fn char_to_row(row: char) -> u8 {
		match row {
			input @ '1' ... '8' => input as u8 - ('1' as u8),
			_  => panic!("Invalid row number: {}.", row)
		}
	}

	pub fn new(cell_name: &String) -> BoardCoordinates {
		if cell_name.len() > 2 {
			panic!("String must be exactly 2 characters long.");
		}

		BoardCoordinates {
			col: BoardCoordinates::char_to_column(
				match cell_name.chars().nth(0) {
					Some(v) => v,
					None => panic!("String must be at least 2 characters long")
				}
			),
			row: BoardCoordinates::char_to_row(
				match cell_name.chars().nth(1) {
					Some(v) => v,
					None => panic!("String must be at least 2 characters long")
				}
			),
		}
	}
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessBoardCell<T> {
	coordinates: BoardCoordinates,
	contents: Option<T> // What piece is occupying the cell
}

#[allow(dead_code)]
impl<T> ChessBoardCell<T> {

	pub fn get_coordinates(&self) -> BoardCoordinates {
		self.coordinates
	}

	// Grig coordinates converted to a string such as 'a6'
	pub fn coordinates_to_string(&self) -> String {
		self.coordinates.to_string()
	}

	pub fn from_string(cell_name: String) -> ChessBoardCell<T> {
		ChessBoardCell {
			coordinates: BoardCoordinates::new(&cell_name),
			contents: None
		}
	}

	// Converting a string like c7 to numeric coordinates in a grid, containing a piece (or None)
	pub fn from_string_with_contents(cell_name: String, contents: T) -> ChessBoardCell<T> {
		ChessBoardCell {
			coordinates: BoardCoordinates::new(&cell_name),
			contents: Some(contents)
		}
	}

	pub fn get_contents(&self) -> &Option<T> {
		&self.contents
	}
}

#[allow(dead_code)]
impl<T> ChessBoard<T> {
	pub fn get_cell_contents(&self, col: usize, row: usize) -> &Option<T> {
		self.columns[col][row].get_contents()
	}
}