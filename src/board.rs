use std::fmt::Debug;
use view::Renderable;
use opengl_graphics::GlGraphics;
use piston_window::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessBoard<T: Clone + Debug> {
	size: u8,
	columns: Vec<Vec<ChessBoardCell<T>>>
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ChessPieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	Up,
	Down
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ChessPieceColor {
	Black,
	White
}

impl ChessPieceColor {

	pub fn opposite_color(&self) -> ChessPieceColor {
		match self {
			&ChessPieceColor::Black => ChessPieceColor::White,
			&ChessPieceColor::White => ChessPieceColor::Black
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct ChessPiece {
	type_name: ChessPieceType,
	color: ChessPieceColor
}

impl ChessPiece {
	pub fn new(type_name: ChessPieceType, color: ChessPieceColor) -> ChessPiece {
		ChessPiece {
			type_name: type_name,
			color: color
		}
	}

	pub fn get_type(&self) -> ChessPieceType {
		self.type_name
	}

	pub fn get_color(&self) -> ChessPieceColor {
		self.color
	}
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct BoardCoordinates {
	pub row: u8,
	pub col: u8
}

// (x, y) coordinates
impl Into<BoardCoordinates> for (u8, u8) {
	fn into(self) -> BoardCoordinates {
		match self {
			(col, row) => BoardCoordinates {
				col: col,
				row: row
			}
		}
	}
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
pub struct ChessBoardCell<T: Clone + Debug> {
	coordinates: BoardCoordinates,
	contents: Option<T> // What piece is occupying the cell
}

#[allow(dead_code)]
impl<T: Clone + Debug> ChessBoardCell<T> {
	pub fn get_coordinates(&self) -> &BoardCoordinates {
		&self.coordinates
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

	pub fn from_coordinates(coordinates: BoardCoordinates) -> ChessBoardCell<T> {
		ChessBoardCell {
			coordinates: coordinates,
			contents: None
		}
	}

	pub fn get_contents(&self) -> &Option<T> {
		&self.contents
	}

	pub fn set_contents(&mut self, contents: Option<T>) {
		self.contents = contents;
	}

	pub fn is_empty(&self) -> bool {
		match self.contents {
			Some(..) => false,
			None => true
		}
	}
}

impl<'a> ChessBoardCell<ChessPiece> {
	pub fn contains_piece_of_color(&self, color: ChessPieceColor) -> bool {
		match self.contents {
			Some(piece) => {
				return piece.color == color;
			},
			None => {
				return false;
			}
		}
	}
}

#[allow(dead_code)]
impl<T: Clone + Debug> ChessBoard<T> {
	pub fn new (size: u8) -> ChessBoard<T> {
		
		let mut columns: Vec<Vec<ChessBoardCell<T>>> = Vec::new();
		for c in 0..size {
			let mut column: Vec<ChessBoardCell<T>> = Vec::new();
			for r in 0..size {
				column.push(
					ChessBoardCell::from_coordinates(
						BoardCoordinates {
							row: r,
							col: c
						}
					)
				)
			}
			columns.push(column);
		}

		ChessBoard {
			size: size,
			columns: columns
		}
	}

	pub fn get_size(&self) -> u8 {
		self.size
	}

	pub fn get_contents_at_coordinates(&self, coordinates: &BoardCoordinates) -> Result<&Option<T>, &'static str> {
		if self.size < coordinates.col || self.size < coordinates.row {
			return Err("Invalid coordinates");
		}

		match self.columns.get(coordinates.col as usize) {
			Some(rows) => {
				match rows.get(coordinates.row as usize) {
					Some(cell) => {
						return Ok(cell.get_contents())
					},
					None => {
						return Err("Invalid coordinates");
					}
				}
			},
			None => { return Err("Invalid coordinates"); }
		}
	}

	pub fn set_contents_at_coordinates(&mut self, coordinates: &BoardCoordinates, contents: Option<T>) {
		if self.size < coordinates.col || self.size < coordinates.row {
			panic!("Coordinates are out of bounds of the board");
		}

		self.columns
			.get_mut(coordinates.col as usize).unwrap()
			.get_mut(coordinates.row as usize).unwrap()
			.set_contents(contents);
	}

	pub fn get_move_destination(&self, start_coordinates: &BoardCoordinates, horzontal: i8, vertical: i8) -> Option<&ChessBoardCell<T>> {
		let final_col = start_coordinates.col as i8 + horzontal;
		let final_row = start_coordinates.row as i8 + vertical;

		if final_row < 0 || final_col < 0 {
			return None;
		}

		match self.columns.iter().nth(final_col as usize) {
			Some(rows) => {
				return rows.iter().nth(final_row as usize);
			}
			None => {
				return None;
			}
		}
	}

	pub fn move_contents(&mut self, start_coordinates: &BoardCoordinates, end_coordinates: &BoardCoordinates)
	{
		if start_coordinates.col > self.size || start_coordinates.row > self.size || 
			end_coordinates.col > self.size || end_coordinates.row > self.size
		{
				panic!("Moving contents from or outside of board")
		}
		// Todo, figure out how to use Cell, RC, Refcell ect... 
		// so that contents don't have to be cloned to do this
		let contents = match self.get_contents_at_coordinates(start_coordinates) {
			Ok(contents_at_coordinates) => {
				contents_at_coordinates.clone()
			},
			Err(e) => { panic!("{}", e) }
		};
		self.set_contents_at_coordinates(start_coordinates, None);
		self.set_contents_at_coordinates(end_coordinates, contents);
	}
}

impl ChessBoard<ChessPiece> {
	pub fn fresh_game(&mut self, bottom_player_color: ChessPieceColor) {
		for (i, r) in self.columns.iter_mut().enumerate() {
			for board_cell in r.iter_mut() {
				let contents = ChessBoard::get_contents_for_starting_coordinates(bottom_player_color, board_cell.get_coordinates());
				board_cell.set_contents(contents);
			}
		}
	}

	fn get_contents_for_starting_coordinates(bottom_player_color: ChessPieceColor, coordinates: &BoardCoordinates) -> Option<ChessPiece> {
		let &BoardCoordinates{row, col} = coordinates;
		match (col, row) {
			(_, 1) => {
				return Some(ChessPiece::new(ChessPieceType::Pawn, bottom_player_color));
			},
			(_, 6) => {
				return Some(ChessPiece::new(ChessPieceType::Pawn, bottom_player_color.opposite_color()));
			},
			(0, 0) | (7, 0) => {
				return Some(ChessPiece::new(ChessPieceType::Rook, bottom_player_color));	
			},
			(1, 0) | (6, 0) => {
				return Some(ChessPiece::new(ChessPieceType::Knight, bottom_player_color));	
			},
			(2, 0) | (5, 0) => {
				return Some(ChessPiece::new(ChessPieceType::Bishop, bottom_player_color));	
			},
			(3 , 0) => {
				return Some(ChessPiece::new(ChessPieceType::King, bottom_player_color));	
			},
			(4, 0) => {
				return Some(ChessPiece::new(ChessPieceType::Queen, bottom_player_color));	
			},
			(0, 7) | (7, 7) => {
				return Some(ChessPiece::new(ChessPieceType::Rook, bottom_player_color.opposite_color()));	
			},
			(1 , 7) | (6, 7) => {
				return Some(ChessPiece::new(ChessPieceType::Knight, bottom_player_color.opposite_color()));	
			},
			(2, 7) | (5, 7) => {
				return Some(ChessPiece::new(ChessPieceType::Bishop, bottom_player_color.opposite_color()));	
			},
			(4 , 7) => {
				return Some(ChessPiece::new(ChessPieceType::King, bottom_player_color.opposite_color()));	
			},
			(3, 7) => {
				return Some(ChessPiece::new(ChessPieceType::Queen, bottom_player_color.opposite_color()));	
			},
			_ => { 
				return None
			}
		}
	}
}

#[derive(Debug)]
pub struct ChessMove {
	pub start_coordinates: BoardCoordinates,
	pub end_coordinates: BoardCoordinates
}

impl ChessMove {
	pub fn new(start_coordinates: &BoardCoordinates, end_coordinates: &BoardCoordinates) -> ChessMove {
		ChessMove {
			start_coordinates: start_coordinates.clone(),
			end_coordinates: end_coordinates.clone()
		}
	}
}

impl Renderable for ChessBoard<ChessPiece> {
	fn draw(&self, window: &PistonWindow) {
		&self.columns.first().unwrap().first().unwrap().draw(window);
		return;
		for row in &self.columns {
			for board_cell in row {
				board_cell.draw(window);
				return;
			}
		};

		// Todo, also draw the boarders between cells?
	}
}

impl Renderable for ChessBoardCell<ChessPiece> {
	fn draw(&self, window: &PistonWindow) {
	    match self.contents {
	    	// Todo: Lookup ref keyword
	    	Some(ref contents) => {
	    		window.draw_2d(|c, gl| {
	    			draw_chess_piece_sprite(window, contents, (200f64, 200f64));
	    		})
	    	},
	    	None => {}
	    }
	}
}

pub fn draw_chess_piece_sprite(window: &PistonWindow, piece: &ChessPiece, position: (f64, f64))
{
	let assets = ::find_folder::Search::ParentsThenKids(3, 3)
	.for_folder("sprites").unwrap();

	let tex = ::std::rc::Rc::new(::piston_window::Texture::from_path(
        &mut *window.factory.borrow_mut(),
        assets.join("0_bishop.png"),
        ::piston_window::Flip::None,
        &::piston_window::TextureSettings::new()
    ).unwrap());

    let mut sprite = ::sprite::Sprite::from_texture(tex.clone());
    sprite.set_position(position.0, position.1);

    window.draw_2d(|c, gl| {
    	sprite.draw(c.transform, gl);
    });
}