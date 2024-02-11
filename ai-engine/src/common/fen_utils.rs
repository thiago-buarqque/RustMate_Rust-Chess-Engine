use super::enums::{PieceColor, PieceType};
use super::contants::{
        BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK
    };

pub fn translate_pieces_to_fen(pieces: &[u8]) -> Vec<char> {
    pieces
        .iter()
        .map(|&piece| get_piece_fen(piece))
        .collect()
}

pub fn get_fen_piece_value(piece_fen: &char) -> u8 {
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

pub fn get_piece_fen(piece_value: u8) -> char {
    match piece_value {
        WHITE_BISHOP => 'B',
        WHITE_KING => 'K',
        WHITE_KNIGHT => 'N',
        WHITE_PAWN => 'P',
        WHITE_QUEEN => 'Q',
        WHITE_ROOK => 'R',
        BLACK_BISHOP => 'b',
        BLACK_KING => 'k',
        BLACK_KNIGHT => 'n',
        BLACK_PAWN => 'p',
        BLACK_QUEEN => 'q',
        BLACK_ROOK => 'r',
        _ => '-',
    }
}