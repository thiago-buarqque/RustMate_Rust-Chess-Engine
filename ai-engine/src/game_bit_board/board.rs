use crate::game_bit_board::{board_utils::get_piece_type_from_index, utils::get_piece_letter};

use super::{
    board_utils::{
        get_color_index, get_piece_type_index, is_pawn_promotion, BISHOPS_IDX, BLACK_IDX,
        KINGS_IDX, KNIGHTS_IDX, PAWNS_IDX, PIECE_INDEXES, QUEENS_IDX, ROOKS_IDX, WHITE_IDX,
    },
    enums::{Color, PieceType},
};

pub struct Board {
    bitboards: [u64; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board { bitboards: [0; 8] };
        board.reset();
        board
    }

    pub fn empty() -> Self {
        let mut board = Board { bitboards: [0; 8] };

        board
    }

    fn reset(&mut self) {
        // Placement of pawns
        self.bitboards[PAWNS_IDX] = 0x00FF00000000FF00;
        self.bitboards[WHITE_IDX] = 0x000000000000FFFF;
        self.bitboards[BLACK_IDX] = 0xFFFF000000000000;

        // Placement of rooks
        self.bitboards[ROOKS_IDX] = 0x8100000000000081;

        // Placement of knights
        self.bitboards[KNIGHTS_IDX] = 0x4200000000000042;

        // Placement of bishops
        self.bitboards[BISHOPS_IDX] = 0x2400000000000024;

        // Placement of queens
        self.bitboards[QUEENS_IDX] = 0x0800000000000008;

        // Placement of kings
        self.bitboards[KINGS_IDX] = 0x1000000000000010;
    }

    pub fn get_piece_positions(&self, color: Color, piece_type: PieceType) -> u64 {
        self.bitboards[get_piece_type_index(piece_type)] & self.bitboards[get_color_index(color)]
    }

    pub fn get_player_pieces_positions(&self, color: Color) -> u64 {
        self.bitboards[get_color_index(color)]
    }

    fn get_occupied_squares(&self) -> u64 { self.bitboards[WHITE_IDX] | self.bitboards[BLACK_IDX] }

    fn get_empty_squares(&self) -> u64 { !self.get_occupied_squares() }

    fn place_piece(&mut self, color: Color, piece_type: PieceType, position: u64) {
        self.bitboards[get_color_index(color)] |= position;

        self.bitboards[get_piece_type_index(piece_type)] |= position;
    }

    /// This function assumes the piece exist on the specified position.
    fn remove_piece(&mut self, color: Color, piece_type: PieceType, position: u64) {
        self.bitboards[get_color_index(color)] ^= position;

        self.bitboards[get_piece_type_index(piece_type)] ^= position;
    }

    pub fn move_piece(&mut self, color: Color, piece_type: PieceType, from: u64, to: u64) {
        self.remove_piece(color, piece_type, from);

        if piece_type == PieceType::Pawn && is_pawn_promotion(color, from, to) {
            self.place_piece(color, PieceType::Queen, to);
        } else {
            self.place_piece(color, piece_type, to);
        }
    }

    /// This function assumes that the position is not empty
    fn get_piece_color_from_position(&self, position: u64) -> Color {
        if self.bitboards[WHITE_IDX] & position != 0 {
            return Color::White;
        }

        return Color::Black;
    }

    /// This function assumes that the position is not empty
    fn get_piece_type_from_position(&self, position: u64) -> PieceType {
        for piece_index in PIECE_INDEXES {
            if self.bitboards[piece_index] & position != 0 {
                return get_piece_type_from_index(piece_index);
            }
        }

        PieceType::Empty
    }

    pub fn display(&self) {
        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                let position = 1 << (row * 8 + col);

                let piece_char = get_piece_letter(
                    self.get_piece_color_from_position(position),
                    self.get_piece_type_from_position(position),
                );

                print!("{} ", piece_char);
            }
            println!();
        }
        println!("  a b c d e f g h");
    }
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        board::{Board, PAWNS_IDX, QUEENS_IDX},
        enums::{Color, PieceType},
        positions::*,
    };

    #[test]
    fn test_board_initialization() {
        let board = Board::new();

        let white_pawns = 0xFF00; // Rank 2
        let black_pawns = 0x00FF000000000000; // Rank 7

        assert_eq!(
            board.bitboards[PAWNS_IDX] & white_pawns,
            white_pawns,
            "White pawns should be correctly initialized on rank 2"
        );
        assert_eq!(
            board.bitboards[PAWNS_IDX] & black_pawns,
            black_pawns,
            "Black pawns should be correctly initialized on rank 7"
        );
    }

    #[test]
    fn test_get_occupied_squares() {
        let board = Board::new();

        assert_eq!(0xFFFF00000000FFFF, board.get_occupied_squares())
    }

    #[test]
    fn test_get_empty_squares() {
        let board = Board::new();

        assert_eq!(0x0000FFFFFFFF0000, board.get_empty_squares())
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();

        let from = A7;
        let to = A6;

        board.move_piece(Color::Black, PieceType::Pawn, from, to);

        assert_eq!(
            board.bitboards[PAWNS_IDX] & to,
            to,
            "Pawn should be moved to a6"
        );
        assert_eq!(
            board.bitboards[PAWNS_IDX] & from,
            0,
            "Pawn should no longer be at a7"
        );
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::empty();
        let from = H7;
        let to = H8;

        board.place_piece(Color::White, PieceType::Pawn, from);

        board.move_piece(Color::White, PieceType::Pawn, from, to);

        assert_eq!(
            board.bitboards[PAWNS_IDX] & to,
            0,
            "Pawn should be promoted and not present on h1"
        );
        assert_eq!(
            board.bitboards[QUEENS_IDX] & to,
            to,
            "Queen should be placed on h1 after promotion"
        );
    }

    #[test]
    fn display() {
        let mut board = Board::new();

        board.display();

        board.move_piece(Color::White, PieceType::Pawn, A2, A3);

        println!("After moving white pawn from a2 to a3:");

        board.display();
    }
}
