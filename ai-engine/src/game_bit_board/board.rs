use crate::game_bit_board::{board_utils::get_piece_type_from_index, utils::get_piece_symbol};

use super::{
    _move::Move,
    board_utils::{
        get_color_index, get_piece_type_index, is_pawn_promotion, BISHOPS_IDX, BLACK_IDX,
        KINGS_IDX, KNIGHTS_IDX, PAWNS_IDX, PIECE_INDEXES, QUEENS_IDX, ROOKS_IDX, WHITE_IDX,
    },
    enums::{Color, PieceType},
};

pub struct Board {
    bitboards: [u64; 8],
    en_passant_bb_position: u64,
    en_passant_bb_piece_square: u64,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            bitboards: [0; 8],
            en_passant_bb_position: 0,
            en_passant_bb_piece_square: 0,
        };
        board.reset();
        board
    }

    pub fn empty() -> Self {
        Board {
            bitboards: [0; 8],
            en_passant_bb_position: 0,
            en_passant_bb_piece_square: 0,
        }
    }

    pub fn reset(&mut self) {
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

    pub fn get_en_passant(&self) -> u64 { self.en_passant_bb_position }

    pub fn get_en_passant_square(&self) -> u64 { self.en_passant_bb_piece_square }

    pub fn get_piece_positions(&self, color: Color, piece_type: PieceType) -> u64 {
        self.bitboards[get_piece_type_index(piece_type)] & self.bitboards[get_color_index(color)]
    }

    pub fn get_player_pieces_positions(&self, color: Color) -> u64 {
        self.bitboards[get_color_index(color)]
    }

    pub fn get_occupied_squares(&self) -> u64 {
        self.bitboards[WHITE_IDX] | self.bitboards[BLACK_IDX]
    }

    pub fn get_empty_squares(&self) -> u64 { !self.get_occupied_squares() }

    pub fn place_piece(&mut self, color: Color, piece_type: PieceType, bb_position: u64) {
        self.bitboards[get_color_index(color)] |= bb_position;

        self.bitboards[get_piece_type_index(piece_type)] |= bb_position;
    }

    /// This function assumes the piece exist on the specified position.
    fn remove_piece(&mut self, color: Color, piece_type: PieceType, bb_position: u64) {
        self.bitboards[get_color_index(color)] ^= bb_position;

        self.bitboards[get_piece_type_index(piece_type)] ^= bb_position;
    }

    pub fn move_piece(&mut self, _move: Move) {
        let color = self.get_piece_color(_move.get_from());
        let mut piece_type = self.get_piece_type(_move.get_from());
        let from: u64 = 1 << _move.get_from();
        let to: u64 = 1 << _move.get_to();

        if _move.is_en_passant() {
            // Remove the en passant enemy piece
            self.remove_piece(color, piece_type, self.en_passant_bb_piece_square);
            self.en_passant_bb_position = 0;
            self.en_passant_bb_piece_square = 0;
        }

        self.remove_piece(color, piece_type, from);

        if piece_type == PieceType::Pawn {
            if _move.is_double_pawn_push() {
                // Save en passant info
                self.en_passant_bb_position = _move.get_en_passant_bb_position();
                self.en_passant_bb_piece_square = _move.get_en_passant_bb_piece_square();
            } else if is_pawn_promotion(color, from, to) {
                // TODO: get promotion defined inside the move
                piece_type = PieceType::Queen;
            }
        } else if self.en_passant_bb_position != 0 {
            // When a move is made and en passant is available, remove en passant option
            self.en_passant_bb_position = 0;
            self.en_passant_bb_piece_square = 0;
        }

        self.place_piece(color, piece_type, to);
    }

    /// This function assumes that the square is not empty
    pub fn get_piece_color(&self, square: usize) -> Color {
        if self.bitboards[WHITE_IDX] & 1 << square != 0 {
            return Color::White;
        }

        return Color::Black;
    }

    /// This function assumes that the square is not empty
    pub fn get_piece_type(&self, square: usize) -> PieceType {
        for piece_index in PIECE_INDEXES {
            if self.bitboards[piece_index] & 1 << square != 0 {
                return get_piece_type_from_index(piece_index);
            }
        }

        PieceType::Empty
    }

    pub fn display(&self) {
        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                let piece_char = get_piece_symbol(
                    self.get_piece_color(row * 8 + col),
                    self.get_piece_type(row * 8 + col),
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
        _move::Move,
        board::{Board, PAWNS_IDX, QUEENS_IDX},
        enums::{Color, PieceType},
        move_contants::{DOUBLE_PAWN_PUSH, EN_PASSANT, QUEEN_PROMOTION},
        positions::{BBPositions, Squares},
    };

    #[test]
    fn test_en_passant_move() {
        let mut board = Board::new();

        board.move_piece(Move::from_to(Squares::D2, Squares::D4));
        board.move_piece(Move::from_to(Squares::E7, Squares::E5));
        board.move_piece(Move::from_to(Squares::D4, Squares::D5));

        let mut _move = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::C7, Squares::C5);

        _move.set_en_passant_bb_position(BBPositions::C6);
        _move.set_en_passant_bb_piece_square(BBPositions::C5);

        board.move_piece(_move);

        assert_eq!(PieceType::Pawn, board.get_piece_type(Squares::C5));

        board.display();

        board.move_piece(Move::with_flags(EN_PASSANT, Squares::D5, Squares::C6));

        board.display();

        assert_eq!(PieceType::Empty, board.get_piece_type(Squares::C5));
        assert_eq!(PieceType::Pawn, board.get_piece_type(Squares::C6));
        assert_eq!(Color::White, board.get_piece_color(Squares::C6));
    }

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

        let from = BBPositions::A7;
        let to = BBPositions::A6;

        board.move_piece(Move::from_to(48, 40));

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
        let from = BBPositions::H7;
        let to = BBPositions::H8;

        board.place_piece(Color::White, PieceType::Pawn, from);

        board.move_piece(Move::with_flags(QUEEN_PROMOTION, 55, 63));

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

        board.move_piece(Move::from_to(8, 16));

        println!("After moving white pawn from a2 to a3:");

        board.display();
    }
}
