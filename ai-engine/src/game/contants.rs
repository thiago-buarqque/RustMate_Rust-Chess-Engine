use crate::common::enums::{PieceColor, PieceType};

pub const BLACK_KING_INITIAL_POSITION: i8 = 4;
pub const BLACK_KING_ROOK_POSITION: i8 = 7;
pub const BLACK_KING_VALUE: u8 = PieceColor::Black as u8 | PieceType::King as u8;
pub const BLACK_PAWN_VALUE: u8 = PieceColor::Black as u8 | PieceType::Pawn as u8;
pub const BLACK_QUEEN_ROOK_POSITION: i8 = 0;

pub const BLACK_QUEEN_SIDE_ROOK_POSITION: i8 = 0;
pub const BLACK_KING_SIDE_ROOK_POSITION: i8 = 7;

pub const LETTER_A_UNICODE: u8 = b'a';

pub const WHITE_KING_INITIAL_POSITION: i8 = 60;
pub const WHITE_KING_ROOK_POSITION: i8 = 63;
pub const WHITE_KING_VALUE: u8 = PieceColor::White as u8 | PieceType::King as u8;
pub const WHITE_PAWN_VALUE: u8 = PieceColor::White as u8 | PieceType::Pawn as u8;
pub const WHITE_QUEEN_ROOK_POSITION: i8 = 56;

pub const WHITE_QUEEN_SIDE_ROOK_POSITION: i8 = 56;
pub const WHITE_KING_SIDE_ROOK_POSITION: i8 = 63;
