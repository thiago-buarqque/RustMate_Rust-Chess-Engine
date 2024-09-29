use crate::game_bit_board::{
    board::Board,
    enums::{Color, PieceType},
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
        PieceType::Empty => usize::MAX,
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

#[derive(Clone, Copy)]
pub enum Side {
    KingSide,
    QueenSide,
}

pub fn get_castling_right_flag(color: Color, side: Side) -> u8 {
    let castling_right = match (color, side) {
        (Color::White, Side::KingSide) => Board::WHITE_KING_SIDE_CASTLING_RIGHT,
        (Color::White, Side::QueenSide) => Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT,
        (Color::Black, Side::KingSide) => Board::BLACK_KING_SIDE_CASTLING_RIGHT,
        (Color::Black, Side::QueenSide) => Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT,
    };
    castling_right
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{enums::PieceType, utils::board_utils::*};

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
