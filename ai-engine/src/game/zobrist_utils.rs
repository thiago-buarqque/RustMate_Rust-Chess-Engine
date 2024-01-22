use crate::common::{
    enums::PieceType,
    piece_utils::{get_piece_type, is_white_piece},
};

pub const WHITE_BISHOP: usize = 0;
pub const WHITE_KING: usize = 1;
pub const WHITE_KNIGHT: usize = 2;
pub const WHITE_PAWN: usize = 3;
pub const WHITE_QUEEN: usize = 4;
pub const WHITE_ROOK: usize = 5;

pub const BLACK_BISHOP: usize = 6;
pub const BLACK_KING: usize = 7;
pub const BLACK_KNIGHT: usize = 8;
pub const BLACK_PAWN: usize = 9;
pub const BLACK_QUEEN: usize = 10;
pub const BLACK_ROOK: usize = 11;

pub fn get_piece_index(piece_value: i8) -> usize {
    let piece_type = get_piece_type(piece_value);

    if is_white_piece(piece_value) {
        match piece_type {
            PieceType::Bishop => WHITE_BISHOP,
            PieceType::King => WHITE_KING,
            PieceType::Knight => WHITE_KNIGHT,
            PieceType::Pawn => WHITE_PAWN,
            PieceType::Queen => WHITE_QUEEN,
            PieceType::Rook => WHITE_ROOK,
            _ => 100,
        }
    } else {
        match piece_type {
            PieceType::Bishop => BLACK_BISHOP,
            PieceType::King => BLACK_KING,
            PieceType::Knight => BLACK_KNIGHT,
            PieceType::Pawn => BLACK_PAWN,
            PieceType::Queen => BLACK_QUEEN,
            PieceType::Rook => BLACK_ROOK,
            _ => 100,
        }
    }
}
