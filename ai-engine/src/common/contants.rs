use super::enums::{PieceColor, PieceType};

pub const EMPTY_PIECE: u8 = PieceType::Empty as u8;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_LOWER_BOUND: u8 = PieceColor::White as u8 | PieceType::Bishop as u8;
pub const WHITE_UPPER_BOUND: u8 = PieceColor::White as u8 | PieceType::Rook as u8;

pub const BISHOP_WORTH: f32 = 300.0;
pub const KING_WORTH: f32 = 20000.0;
pub const KNIGHT_WORTH: f32 = 300.0;
pub const PAWN_WORTH: f32 = 100.0;
pub const QUEEN_WORTH: f32 = 900.0;
pub const ROOK_WORTH: f32 = 500.0;

pub const INVALID_BOARD_POSITION: i8 = -1;

pub const BLACK_BISHOP: u8 = PieceColor::Black as u8 | PieceType::Bishop as u8;
pub const BLACK_KING: u8 = PieceColor::Black as u8 | PieceType::King as u8;
pub const BLACK_KNIGHT: u8 = PieceColor::Black as u8 | PieceType::Knight as u8;
pub const BLACK_PAWN: u8 = PieceColor::Black as u8 | PieceType::Pawn as u8;
pub const BLACK_QUEEN: u8 = PieceColor::Black as u8 | PieceType::Queen as u8;
pub const BLACK_ROOK: u8 = PieceColor::Black as u8 | PieceType::Rook as u8;

pub const WHITE_BISHOP: u8 = PieceColor::White as u8 | PieceType::Bishop as u8;
pub const WHITE_KING: u8 = PieceColor::White as u8 | PieceType::King as u8;
pub const WHITE_KNIGHT: u8 = PieceColor::White as u8 | PieceType::Knight as u8;
pub const WHITE_PAWN: u8 = PieceColor::White as u8 | PieceType::Pawn as u8;
pub const WHITE_QUEEN: u8 = PieceColor::White as u8 | PieceType::Queen as u8;
pub const WHITE_ROOK: u8 = PieceColor::White as u8 | PieceType::Rook as u8;