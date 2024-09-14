use crate::game_bit_board::enums::PieceType;

use super::move_contants::*;

pub fn get_promotion_flag_from_symbol(symbol: char) -> (u16, u16) {
    match symbol {
        'b' | 'B' => (BISHOP_PROMOTION, BISHOP_PROMOTION_CAPTURE),
        'n' | 'N' => (KNIGHT_PROMOTION, KNIGHT_PROMOTION_CAPTURE),
        'q' | 'Q' => (QUEEN_PROMOTION, QUEEN_PROMOTION_CAPTURE),
        'r' | 'R' => (ROOK_PROMOTION, ROOK_PROMOTION_CAPTURE),
        _ => (0, 0),
    }
}

pub fn get_piece_type_from_promotion_flag(flag: u16) -> PieceType {
    match flag {
        KNIGHT_PROMOTION | KNIGHT_PROMOTION_CAPTURE => PieceType::Knight,
        BISHOP_PROMOTION | BISHOP_PROMOTION_CAPTURE => PieceType::Bishop,
        ROOK_PROMOTION | ROOK_PROMOTION_CAPTURE => PieceType::Rook,
        QUEEN_PROMOTION | QUEEN_PROMOTION_CAPTURE => PieceType::Queen,
        _ => PieceType::Empty,
    }
}
