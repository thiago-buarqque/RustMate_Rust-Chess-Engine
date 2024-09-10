use crate::game_bit_board::{board_utils::get_piece_type_from_index, utils::get_piece_symbol};

use super::{
    _move::Move,
    board_utils::{
        get_color_index, get_piece_type_index, is_pawn_promotion, BISHOPS_IDX, BLACK_IDX,
        KINGS_IDX, KNIGHTS_IDX, PAWNS_IDX, PIECE_INDEXES, QUEENS_IDX, ROOKS_IDX, WHITE_IDX,
    },
    enums::{Color, PieceType},
    positions::BBPositions,
};

pub struct Board {
    bitboards: [u64; 8],
    en_passant_bb_position: u64,
    en_passant_bb_piece_square: u64,
    side_to_move: Color,
    black_king_moved: bool,
    white_king_moved: bool,
    // 0000 1111
    // *Only considered the last 4 digits*
    //
    // Bit 4: White kingside castling
    // Bit 5: White queenside castling.
    // Bit 6: Black kingside castling.
    // Bit 7: Black queenside castling.
    castling_rights: u8,

    // Stacks used to unmake moves
    castling_rights_history: Vec<u8>,
    bitboards_history: Vec<[u64; 8]>,
    en_passant_bb_position_history: Vec<u64>,
    en_passant_bb_piece_square_history: Vec<u64>,
    black_king_moved_history: Vec<bool>,
    white_king_moved_history: Vec<bool>,
    moves_history: Vec<Move>,
}

impl Board {
    const BLACK_CASTLING_RIGHTS: u8 = 0x3;
    const BLACK_KING_SIDE_CASTLE_ROOK_FINAL_POS: u64 = BBPositions::F8;
    const BLACK_KING_SIDE_CASTLE_ROOK_INITIAL_POS: u64 = BBPositions::H8;
    const BLACK_KING_SIDE_CASTLING_RIGHT: u8 = 0x2;
    const BLACK_QUEEN_SIDE_CASTLE_ROOK_FINAL_POS: u64 = BBPositions::D8;
    const BLACK_QUEEN_SIDE_CASTLE_ROOK_INITIAL_POS: u64 = BBPositions::A8;
    const BLACK_QUEEN_SIDE_CASTLING_RIGHT: u8 = 0x1;
    const WHITE_CASTLING_RIGHTS: u8 = 0xC;
    const WHITE_KING_SIDE_CASTLE_ROOK_FINAL_POS: u64 = BBPositions::F1;
    const WHITE_KING_SIDE_CASTLE_ROOK_INITIAL_POS: u64 = BBPositions::H1;
    const WHITE_KING_SIDE_CASTLING_RIGHT: u8 = 0x8;
    const WHITE_QUEEN_SIDE_CASTLE_ROOK_FINAL_POS: u64 = BBPositions::D1;
    const WHITE_QUEEN_SIDE_CASTLE_ROOK_INITIAL_POS: u64 = BBPositions::A1;
    const WHITE_QUEEN_SIDE_CASTLING_RIGHT: u8 = 0x4;

    pub fn new() -> Self {
        let mut board = Self::empty();

        board.reset();

        board
    }

    pub fn empty() -> Self {
        Board {
            bitboards: [0; 8],
            en_passant_bb_position: 0,
            en_passant_bb_piece_square: 0,
            side_to_move: Color::White,
            black_king_moved: false,
            white_king_moved: false,
            castling_rights: 0,

            castling_rights_history: Vec::new(),
            bitboards_history: Vec::new(),
            en_passant_bb_position_history: Vec::new(),
            en_passant_bb_piece_square_history: Vec::new(),
            black_king_moved_history: Vec::new(),
            white_king_moved_history: Vec::new(),
            moves_history: Vec::new(),
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

        self.castling_rights = 0xF;
    }

    pub fn get_side_to_move(&self) -> Color { self.side_to_move }

    pub fn has_king_side_castle_right(&self, color: Color) -> bool {
        if color.is_black() {
            return !self.black_king_moved && self.castling_rights & 0x2 != 0;
        }

        !self.white_king_moved && self.castling_rights & 0x8 != 0
    }

    pub fn has_queen_side_castle_right(&self, color: Color) -> bool {
        if color.is_black() {
            return !self.black_king_moved && self.castling_rights & 0x1 != 0;
        }

        !self.white_king_moved && self.castling_rights & 0x4 != 0
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

    fn save_current_state(&mut self, _move: Move) {
        self.castling_rights_history.push(self.castling_rights);
        self.bitboards_history.push(self.bitboards);
        self.en_passant_bb_position_history
            .push(self.en_passant_bb_position);
        self.en_passant_bb_piece_square_history
            .push(self.en_passant_bb_piece_square);
        self.black_king_moved_history.push(self.black_king_moved);
        self.white_king_moved_history.push(self.white_king_moved);
        self.moves_history.push(_move);
    }

    pub fn unmake_last_move(&mut self) {
        if self.castling_rights_history.len() == 0 {
            return;
        }

        self.castling_rights = *self.castling_rights_history.last().unwrap();
        self.bitboards = *self.bitboards_history.last().unwrap();
        self.en_passant_bb_position = *self.en_passant_bb_position_history.last().unwrap();
        self.en_passant_bb_piece_square = *self.en_passant_bb_piece_square_history.last().unwrap();
        self.black_king_moved = *self.black_king_moved_history.last().unwrap();
        self.white_king_moved = *self.white_king_moved_history.last().unwrap();

        self.castling_rights_history
            .remove(self.castling_rights_history.len() - 1);
        self.bitboards_history
            .remove(self.bitboards_history.len() - 1);
        self.en_passant_bb_position_history
            .remove(self.en_passant_bb_position_history.len() - 1);
        self.en_passant_bb_piece_square_history
            .remove(self.en_passant_bb_piece_square_history.len() - 1);
        self.black_king_moved_history
            .remove(self.black_king_moved_history.len() - 1);
        self.white_king_moved_history
            .remove(self.white_king_moved_history.len() - 1);
        self.moves_history.remove(self.moves_history.len() - 1);

        self.side_to_move = self.side_to_move.opponent();
    }

    pub fn move_piece(&mut self, _move: Move) {
        if self.get_piece_color(_move.get_from()) != self.side_to_move {
            panic!("Invalid player move, it's not your turn. {_move}")
        }

        self.save_current_state(_move.clone());

        let color = self.get_piece_color(_move.get_from());
        let mut piece_type = self.get_piece_type(_move.get_from());
        let from: u64 = 1 << _move.get_from();
        let to: u64 = 1 << _move.get_to();

        if _move.is_en_passant() {
            // Remove the en passant enemy piece
            self.remove_piece(
                color.opponent(),
                piece_type,
                self.en_passant_bb_piece_square,
            );
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

        let opponent_piece = self.get_piece_type(_move.get_to());

        if opponent_piece != PieceType::Empty {
            self.remove_piece(color.opponent(), opponent_piece, to);
        }

        self.place_piece(color, piece_type, to);

        // Remove castling rights when king moves
        if piece_type == PieceType::King {
            if color.is_black() {
                self.black_king_moved = true;
                self.castling_rights &= Board::WHITE_CASTLING_RIGHTS;

                if _move.is_king_castle() {
                    self.remove_piece(
                        color,
                        PieceType::Rook,
                        Board::BLACK_KING_SIDE_CASTLE_ROOK_INITIAL_POS,
                    );
                    self.place_piece(
                        color,
                        PieceType::Rook,
                        Board::BLACK_KING_SIDE_CASTLE_ROOK_FINAL_POS,
                    );
                } else if _move.is_queen_castle() {
                    self.remove_piece(
                        color,
                        PieceType::Rook,
                        Board::BLACK_QUEEN_SIDE_CASTLE_ROOK_INITIAL_POS,
                    );
                    self.place_piece(
                        color,
                        PieceType::Rook,
                        Board::BLACK_QUEEN_SIDE_CASTLE_ROOK_FINAL_POS,
                    );
                }
            } else {
                self.white_king_moved = true;
                self.castling_rights &= Board::BLACK_CASTLING_RIGHTS;

                if _move.is_king_castle() {
                    self.remove_piece(
                        color,
                        PieceType::Rook,
                        Board::WHITE_KING_SIDE_CASTLE_ROOK_INITIAL_POS,
                    );
                    self.place_piece(
                        color,
                        PieceType::Rook,
                        Board::WHITE_KING_SIDE_CASTLE_ROOK_FINAL_POS,
                    );
                } else if _move.is_queen_castle() {
                    self.remove_piece(
                        color,
                        PieceType::Rook,
                        Board::WHITE_QUEEN_SIDE_CASTLE_ROOK_INITIAL_POS,
                    );
                    self.place_piece(
                        color,
                        PieceType::Rook,
                        Board::WHITE_QUEEN_SIDE_CASTLE_ROOK_FINAL_POS,
                    );
                }
            }
        }

        self.side_to_move = self.side_to_move.opponent();

        if piece_type != PieceType::Rook {
            return;
        }

        if color.is_black() {
            if from == BBPositions::A8 {
                self.castling_rights &= !Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT;
            } else if from == BBPositions::H8 {
                self.castling_rights &= !Board::BLACK_KING_SIDE_CASTLING_RIGHT;
            }
        } else {
            if from == BBPositions::A1 {
                self.castling_rights &= !Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT;
            } else if from == BBPositions::H1 {
                self.castling_rights &= !Board::WHITE_KING_SIDE_CASTLING_RIGHT;
            }
        }
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

    pub fn display(&self) { self.display_with_attacks(Vec::new()); }

    pub fn display_with_attacks(&self, attack_squares: Vec<usize>) {
        println!("\nen_passant_bb_position: {}", self.en_passant_bb_position);
        println!(
            "en_passant_bb_piece_square: {}",
            self.en_passant_bb_piece_square
        );
        println!("side_to_move: {}", self.side_to_move);
        println!("black_king_moved: {}", self.black_king_moved);
        println!("white_king_moved: {}", self.white_king_moved);

        println!(
            "BLACK_KING_SIDE_CASTLING_RIGHT: {}",
            self.castling_rights & Board::BLACK_KING_SIDE_CASTLING_RIGHT != 0
        );
        println!(
            "BLACK_QUEEN_SIDE_CASTLING_RIGHT: {}",
            self.castling_rights & Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT != 0
        );
        println!(
            "WHITE_KING_SIDE_CASTLING_RIGHT: {}",
            self.castling_rights & Board::WHITE_KING_SIDE_CASTLING_RIGHT != 0
        );
        println!(
            "WHITE_QUEEN_SIDE_CASTLING_RIGHT: {}",
            self.castling_rights & Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT != 0
        );

        println!("");

        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                let square = row * 8 + col;

                let piece_char = if attack_squares.contains(&square) {
                    "*".to_string()
                } else {
                    get_piece_symbol(self.get_piece_color(square), self.get_piece_type(square))
                };

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
    fn test_castling() {
        // White side: king moved
        println!("\nTest - White side: king moved\n");
        let mut board = Board::new();

        assert_eq!(0xF, board.castling_rights);

        board.move_piece(Move::dummy_from_to(Squares::D2, Squares::D4));
        board.move_piece(Move::dummy_from_to(Squares::D7, Squares::D5));
        board.move_piece(Move::dummy_from_to(Squares::E1, Squares::D2));

        board.display();

        assert_eq!(0x3, board.castling_rights);

        // White side: towers moved
        println!("\nTest - White side: towers moved\n");
        let mut board = Board::new();

        board.remove_piece(Color::White, PieceType::Knight, BBPositions::B1);
        board.remove_piece(Color::White, PieceType::Bishop, BBPositions::C1);
        board.remove_piece(Color::White, PieceType::Queen, BBPositions::D1);

        board.remove_piece(Color::White, PieceType::Knight, BBPositions::G1);
        board.remove_piece(Color::White, PieceType::Bishop, BBPositions::F1);

        assert_eq!(
            0xF, board.castling_rights,
            "Default castling rights should be available"
        );

        board.move_piece(Move::dummy_from_to(Squares::A1, Squares::B1));

        board.display();

        assert_eq!(
            0,
            board.castling_rights & Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT
        );

        board.move_piece(Move::dummy_from_to(Squares::H7, Squares::H6));
        board.move_piece(Move::dummy_from_to(Squares::H1, Squares::G1));

        board.display();

        assert_eq!(
            0,
            board.castling_rights & Board::WHITE_KING_SIDE_CASTLING_RIGHT
        );

        assert_eq!(0, board.castling_rights & Board::WHITE_CASTLING_RIGHTS);

        // Black side: king moved
        println!("\nTest - Black side: king moved\n");
        let mut board = Board::new();

        assert_eq!(0xF, board.castling_rights);

        board.move_piece(Move::dummy_from_to(Squares::E2, Squares::E3));
        board.move_piece(Move::dummy_from_to(Squares::E8, Squares::D7));

        board.display();

        assert_eq!(0xC, board.castling_rights);

        // Black side: towers moved
        println!("\nTest - White side: towers moved\n");
        let mut board = Board::new();

        board.remove_piece(Color::Black, PieceType::Knight, BBPositions::B8);
        board.remove_piece(Color::Black, PieceType::Bishop, BBPositions::C8);
        board.remove_piece(Color::Black, PieceType::Queen, BBPositions::D8);

        board.remove_piece(Color::Black, PieceType::Bishop, BBPositions::F8);
        board.remove_piece(Color::Black, PieceType::Knight, BBPositions::G8);

        assert_eq!(
            0xF, board.castling_rights,
            "Default castling rights should be available"
        );

        board.move_piece(Move::dummy_from_to(Squares::A2, Squares::A3));
        board.move_piece(Move::dummy_from_to(Squares::A8, Squares::B8));

        board.display();

        assert_eq!(
            0,
            board.castling_rights & Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT
        );

        board.move_piece(Move::dummy_from_to(Squares::A3, Squares::A4));
        board.move_piece(Move::dummy_from_to(Squares::H8, Squares::G8));

        board.display();

        assert_eq!(
            0,
            board.castling_rights & Board::BLACK_KING_SIDE_CASTLING_RIGHT
        );

        assert_eq!(0, board.castling_rights & Board::BLACK_CASTLING_RIGHTS);
    }

    #[test]
    fn test_en_passant_move() {
        let mut board = Board::new();

        board.move_piece(Move::dummy_from_to(Squares::D2, Squares::D4));
        board.move_piece(Move::dummy_from_to(Squares::E7, Squares::E5));
        board.move_piece(Move::dummy_from_to(Squares::D4, Squares::D5));

        let mut _move = Move::dummy_with_flags(DOUBLE_PAWN_PUSH, Squares::C7, Squares::C5);

        _move.set_en_passant_bb_position(BBPositions::C6);
        _move.set_en_passant_bb_piece_square(BBPositions::C5);

        board.move_piece(_move);

        assert_eq!(PieceType::Pawn, board.get_piece_type(Squares::C5));

        board.display();

        board.move_piece(Move::dummy_with_flags(EN_PASSANT, Squares::D5, Squares::C6));

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

        let from = BBPositions::A2;
        let to = BBPositions::A4;

        board.move_piece(Move::dummy_from_to(Squares::A2, Squares::A4));

        assert_eq!(
            board.bitboards[PAWNS_IDX] & to,
            to,
            "Pawn should be moved to a4"
        );
        assert_eq!(
            board.bitboards[PAWNS_IDX] & from,
            0,
            "Pawn should no longer be at a2"
        );
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::empty();
        let from = BBPositions::H7;
        let to = BBPositions::H8;

        board.place_piece(Color::White, PieceType::Pawn, from);

        board.move_piece(Move::dummy_with_flags(QUEEN_PROMOTION, 55, 63));

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

        board.move_piece(Move::dummy_from_to(8, 16));

        println!("After moving white pawn from a2 to a3:");

        board.display();
    }
}
