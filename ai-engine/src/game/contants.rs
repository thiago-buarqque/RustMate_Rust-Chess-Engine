use crate::common::enums::{PieceColor, PieceType};

pub const BLACK_KING_INITIAL_POSITION: i8 = 4;
pub const BLACK_KING_ROOK_POS: i8 = 7;
pub const BLACK_KING_VALUE: i8 = PieceColor::Black as i8 | PieceType::King as i8;
pub const BLACK_PAWN_VALUE: i8 = PieceColor::Black as i8 | PieceType::Pawn as i8;
pub const BLACK_QUEEN_ROOK_POS: i8 = 0;

pub const LETTER_A_UNICODE: u8 = b'a';

pub const WHITE_KING_INITIAL_POSITION: i8 = 60;
pub const WHITE_KING_ROOK_POS: i8 = 63;
pub const WHITE_KING_VALUE: i8 = PieceColor::White as i8 | PieceType::King as i8;
pub const WHITE_PAWN_VALUE: i8 = PieceColor::White as i8 | PieceType::Pawn as i8;
pub const WHITE_QUEEN_ROOK_POS: i8 = 56;