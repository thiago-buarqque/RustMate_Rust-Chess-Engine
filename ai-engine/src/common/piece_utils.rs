use super::{
    contants::{
        BISHOP_WORTH, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, EMPTY_PIECE, KING_WORTH, KNIGHT_WORTH, PAWN_WORTH, QUEEN_WORTH, ROOK_WORTH, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_LOWER_BOUND, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK, WHITE_UPPER_BOUND
    },
    enums::PieceType,
};

pub fn is_piece_of_type(piece: u8, piece_type: PieceType) -> bool {
    get_piece_type(piece) == piece_type
}

pub fn get_promotion_options(white: bool) -> Vec<u8> {
    if !white {
        return vec![BLACK_BISHOP, BLACK_KNIGHT, BLACK_QUEEN, BLACK_ROOK];
    }

    vec![WHITE_BISHOP, WHITE_KNIGHT, WHITE_QUEEN, WHITE_ROOK]
}

pub fn get_piece_type(piece_value: u8) -> PieceType {
    match piece_value {
        EMPTY_PIECE => PieceType::Empty,
        WHITE_BISHOP | BLACK_BISHOP => PieceType::Bishop,
        WHITE_KING | BLACK_KING => PieceType::King,
        WHITE_KNIGHT | BLACK_KNIGHT => PieceType::Knight,
        WHITE_PAWN | BLACK_PAWN => PieceType::Pawn,
        WHITE_QUEEN | BLACK_QUEEN => PieceType::Queen,
        WHITE_ROOK | BLACK_ROOK => PieceType::Rook,
        _ => PieceType::Empty,
    }
}

pub fn get_piece_worth(piece_value: u8) -> i32 {
    match piece_value {
        WHITE_BISHOP | BLACK_BISHOP => BISHOP_WORTH as i32,
        WHITE_KING | BLACK_KING => KING_WORTH as i32,
        WHITE_KNIGHT | BLACK_KNIGHT => KNIGHT_WORTH as i32,
        WHITE_PAWN | BLACK_PAWN => PAWN_WORTH as i32,
        WHITE_QUEEN | BLACK_QUEEN => QUEEN_WORTH as i32,
        WHITE_ROOK | BLACK_ROOK => ROOK_WORTH as i32,
        _ => 0,
    }
}

#[inline]
pub fn is_white_piece(piece_value: u8) -> bool {
    (WHITE_LOWER_BOUND..=WHITE_UPPER_BOUND).contains(&piece_value)
}

#[inline]
pub fn is_same_color(piece1: u8, piece2: u8) -> bool {
    is_white_piece(piece1) == is_white_piece(piece2)
}
