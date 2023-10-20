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
        match piece_type.value() {
            x if x == PieceType::Bishop.value() => WHITE_BISHOP,
            x if x == PieceType::King.value() => WHITE_KING,
            x if x == PieceType::Knight.value() => WHITE_KNIGHT,
            x if x == PieceType::Pawn.value() => WHITE_PAWN,
            x if x == PieceType::Queen.value() => WHITE_QUEEN,
            x if x == PieceType::Rook.value() => WHITE_ROOK,
            _ => 100,
        }
    } else {
        match piece_type.value() {
            x if x == PieceType::Bishop.value() => BLACK_BISHOP,
            x if x == PieceType::King.value() => BLACK_KING,
            x if x == PieceType::Knight.value() => BLACK_KNIGHT,
            x if x == PieceType::Pawn.value() => BLACK_PAWN,
            x if x == PieceType::Queen.value() => BLACK_QUEEN,
            x if x == PieceType::Rook.value() => BLACK_ROOK,
            _ => 100,
        }
    }
}
