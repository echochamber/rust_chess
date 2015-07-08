use chess_piece::*;
use chess_board_cell::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessBoard {
	rows: [[ChessBoardCell<ChessPiece>; 8]; 8]
}