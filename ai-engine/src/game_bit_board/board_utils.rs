use super::{
    enums::{Color, PieceType},
    positions::{ROW_1, ROW_2, ROW_7, ROW_8},
};

pub const BLACK_IDX: usize = 0;
pub const WHITE_IDX: usize = 1;

pub const BISHOPS_IDX: usize = 2;
pub const KINGS_IDX: usize = 3;
pub const KNIGHTS_IDX: usize = 4;
pub const PAWNS_IDX: usize = 5;
pub const QUEENS_IDX: usize = 6;
pub const ROOKS_IDX: usize = 7;

pub const PIECE_INDEXES: [usize; 6] = [
    BISHOPS_IDX,
    KINGS_IDX,
    KNIGHTS_IDX,
    PAWNS_IDX,
    QUEENS_IDX,
    ROOKS_IDX,
];

pub fn is_white_pawn_promotion(color: Color, from: u64, to: u64) -> bool {
    color == Color::White && ROW_7.contains(&from) && ROW_8.contains(&to)
}

pub fn is_black_pawn_promotion(color: Color, from: u64, to: u64) -> bool {
    color == Color::Black && ROW_2.contains(&from) && ROW_1.contains(&to)
}

pub fn is_pawn_promotion(color: Color, from: u64, to: u64) -> bool {
    is_black_pawn_promotion(color, from, to) || is_white_pawn_promotion(color, from, to)
}

pub fn get_color_index(color: Color) -> usize {
    match color {
        Color::White => WHITE_IDX,
        Color::Black => BLACK_IDX,
    }
}

pub fn get_piece_type_index(piece_type: PieceType) -> usize {
    match piece_type {
        PieceType::Bishop => BISHOPS_IDX,
        PieceType::King => KINGS_IDX,
        PieceType::Knight => KNIGHTS_IDX,
        PieceType::Pawn => PAWNS_IDX,
        PieceType::Queen => QUEENS_IDX,
        PieceType::Rook => ROOKS_IDX,
        PieceType::Empty => usize::MAX
    }
}

pub fn get_piece_type_from_index(index: usize) -> PieceType {
    match index {
        BISHOPS_IDX => PieceType::Bishop,
        KINGS_IDX => PieceType::King,
        KNIGHTS_IDX => PieceType::Knight,
        PAWNS_IDX => PieceType::Pawn,
        QUEENS_IDX => PieceType::Queen,
        ROOKS_IDX => PieceType::Rook,
        _ => PieceType::Empty,
    }
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::board_utils::*;

    #[test]
    fn test_get_piece_type_from_index() {
        assert_eq!(PieceType::Bishop, get_piece_type_from_index(BISHOPS_IDX));

        assert_eq!(PieceType::King, get_piece_type_from_index(KINGS_IDX));

        assert_eq!(PieceType::Knight, get_piece_type_from_index(KNIGHTS_IDX));

        assert_eq!(PieceType::Pawn, get_piece_type_from_index(PAWNS_IDX));

        assert_eq!(PieceType::Queen, get_piece_type_from_index(QUEENS_IDX));

        assert_eq!(PieceType::Rook, get_piece_type_from_index(ROOKS_IDX));
    }

    #[test]
    fn test_get_piece_type_index() {
        assert_eq!(BISHOPS_IDX, get_piece_type_index(PieceType::Bishop));

        assert_eq!(KINGS_IDX, get_piece_type_index(PieceType::King));

        assert_eq!(KNIGHTS_IDX, get_piece_type_index(PieceType::Knight));

        assert_eq!(PAWNS_IDX, get_piece_type_index(PieceType::Pawn));

        assert_eq!(QUEENS_IDX, get_piece_type_index(PieceType::Queen));

        assert_eq!(ROOKS_IDX, get_piece_type_index(PieceType::Rook));
    }
}
