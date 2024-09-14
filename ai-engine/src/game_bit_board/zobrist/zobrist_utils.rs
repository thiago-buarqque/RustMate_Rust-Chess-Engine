// use crate::common::{
//     enums::PieceType,
//     piece_utils::{get_piece_type, is_white_piece},
// };

use crate::game_bit_board::enums::{Color, PieceType};

pub const WHITE_BISHOP_INDEX: usize = 0;
pub const WHITE_KING_INDEX: usize = 1;
pub const WHITE_KNIGHT_INDEX: usize = 2;
pub const WHITE_PAWN_INDEX: usize = 3;
pub const WHITE_QUEEN_INDEX: usize = 4;
pub const WHITE_ROOK_INDEX: usize = 5;

pub const BLACK_BISHOP_INDEX: usize = 6;
pub const BLACK_KING_INDEX: usize = 7;
pub const BLACK_KNIGHT_INDEX: usize = 8;
pub const BLACK_PAWN_INDEX: usize = 9;
pub const BLACK_QUEEN_INDEX: usize = 10;
pub const BLACK_ROOK_INDEX: usize = 11;

pub fn get_piece_index(color: Color, piece_type: PieceType) -> usize {
    if color.is_white() {
        match piece_type {
            PieceType::Bishop => WHITE_BISHOP_INDEX,
            PieceType::King => WHITE_KING_INDEX,
            PieceType::Knight => WHITE_KNIGHT_INDEX,
            PieceType::Pawn => WHITE_PAWN_INDEX,
            PieceType::Queen => WHITE_QUEEN_INDEX,
            PieceType::Rook => WHITE_ROOK_INDEX,
            _ => 100,
        }
    } else {
        match piece_type {
            PieceType::Bishop => BLACK_BISHOP_INDEX,
            PieceType::King => BLACK_KING_INDEX,
            PieceType::Knight => BLACK_KNIGHT_INDEX,
            PieceType::Pawn => BLACK_PAWN_INDEX,
            PieceType::Queen => BLACK_QUEEN_INDEX,
            PieceType::Rook => BLACK_ROOK_INDEX,
            _ => 100,
        }
    }
}
