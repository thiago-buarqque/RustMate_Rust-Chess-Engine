use super::{
    contants::{WHITE_LOWER_BOUND, WHITE_UPPER_BOUND, BISHOP_WORTH, KING_WORTH, KNIGHT_WORTH, PAWN_WORTH, QUEEN_WORTH, ROOK_WORTH},
    enums::{PieceColor, PieceType},
};

pub fn pieces_to_fen(pieces: &[i8]) -> Vec<char> {
    pieces
        .iter()
        .map(|&piece| piece_fen_from_value(piece))
        .collect()
}

pub fn is_piece_of_type(piece: i8, piece_type: PieceType) -> bool {
    get_piece_type(piece) == piece_type
}

pub fn piece_value_from_fen(piece_fen: &char) -> i8 {
    let color = if piece_fen.is_uppercase() {
        PieceColor::White
    } else {
        PieceColor::Black
    };

    let piece_type = match piece_fen.to_lowercase().next().unwrap() {
        'b' => PieceType::Bishop,
        'k' => PieceType::King,
        'n' => PieceType::Knight,
        'p' => PieceType::Pawn,
        'q' => PieceType::Queen,
        'r' => PieceType::Rook,
        _ => PieceType::Empty,
    };

    (color.value()) | (piece_type.value())
}

pub fn piece_fen_from_value(piece_value: i8) -> char {
    match piece_value {
        17 => 'B',
        18 => 'K',
        19 => 'N',
        20 => 'P',
        21 => 'Q',
        22 => 'R',
        9 => 'b',
        10 => 'k',
        11 => 'n',
        12 => 'p',
        13 => 'q',
        14 => 'r',
        _ => '-',
    }
}

pub fn get_promotion_options(white: bool) -> Vec<i8> {
    if !white {
        return vec![9, 11, 13, 14];
    }

    vec![17, 19, 21, 22]
}

pub fn get_piece_type(piece_value: i8) -> PieceType {
    match piece_value {
        0 => PieceType::Empty,
        17 | 9 => PieceType::Bishop,
        18 | 10 => PieceType::King,
        19 | 11 => PieceType::Knight,
        20 | 12 => PieceType::Pawn,
        21 | 13 => PieceType::Queen,
        22 | 14 => PieceType::Rook,
        _ => PieceType::Empty,
    }
}

pub fn get_piece_worth(piece_value: i8) -> i32 {
    match piece_value {
        17 | 9 => BISHOP_WORTH as i32,    // Bishop
        18 | 10 => KING_WORTH as i32, // King
        19 | 11 => KNIGHT_WORTH as i32,   // Knight
        20 | 12 => PAWN_WORTH as i32,   // Pawn
        21 | 13 => QUEEN_WORTH as i32,   // Queen
        22 | 14 => ROOK_WORTH as i32,   // Rook
        _ => 0,
    }
}

#[inline]
pub fn is_white_piece(piece_value: i8) -> bool {
    (WHITE_LOWER_BOUND..=WHITE_UPPER_BOUND).contains(&piece_value)
}

#[inline]
pub fn is_same_color(piece1: i8, piece2: i8) -> bool {
    is_white_piece(piece1) == is_white_piece(piece2)
}
