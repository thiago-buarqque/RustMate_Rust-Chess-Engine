use crate::game_bit_board::utils::utils::get_piece_symbol;

use super::{
    _move::{_move::Move, move_utils::get_piece_type_from_promotion_flag},
    enums::{Color, PieceType},
    positions::BBPositions,
    utils::{
        bitwise_utils::{north_one, pop_lsb, south_one, to_bitboard_position},
        board_utils::*,
        utils::{
            algebraic_to_square, get_piece_color_and_type_from_symbol, get_piece_letter,
            square_to_algebraic,
        },
    },
    zobrist::zobrist::Zobrist,
};

#[derive(Clone, Copy)]
enum Side {
    KingSide,
    QueenSide,
}

#[derive(Clone)]
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
    winner: Option<Color>,
    full_move_clock: u32,
    half_move_clock: u32,

    // Stacks used to unmake moves
    castling_rights_history: Vec<u8>,
    bitboards_history: Vec<[u64; 8]>,
    en_passant_bb_position_history: Vec<u64>,
    en_passant_bb_piece_square_history: Vec<u64>,
    black_king_moved_history: Vec<bool>,
    white_king_moved_history: Vec<bool>,
    moves_history: Vec<Move>,
    zobrist_history: Vec<Zobrist>,
    zobrist: Zobrist,
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
        let mut board = Board {
            bitboards: [0; 8],
            en_passant_bb_position: 0,
            en_passant_bb_piece_square: 0,
            side_to_move: Color::White,
            black_king_moved: false,
            white_king_moved: false,
            castling_rights: 0,
            winner: None,
            full_move_clock: 1,
            half_move_clock: 0,

            castling_rights_history: Vec::new(),
            bitboards_history: Vec::new(),
            en_passant_bb_position_history: Vec::new(),
            en_passant_bb_piece_square_history: Vec::new(),
            black_king_moved_history: Vec::new(),
            white_king_moved_history: Vec::new(),
            moves_history: Vec::new(),
            zobrist_history: Vec::new(),

            zobrist: Zobrist::new(),
        };

        let mut zobrist = board.zobrist.clone();

        zobrist.compute_hash(&board);

        board.zobrist = zobrist;

        board
    }

    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        // if parts.len() != 6 {
        //     panic!("Invalid FEN string.");
        // }

        let mut board = Board::empty();

        for (i, row) in parts[0].split('/').rev().enumerate() {
            let mut col = 0;

            for char in row.chars() {
                if char.is_digit(10) {
                    col += char.to_digit(10).unwrap() as usize;
                } else {
                    let (color, piece_type) = get_piece_color_and_type_from_symbol(char);

                    let square = 8 * i + col;

                    board.place_piece(color, piece_type, to_bitboard_position(square as u64));

                    col += 1;
                }
            }
        }

        // Setup side to move
        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid side to move in FEN string."),
        };

        // Setup castling rights
        board.castling_rights = 0;
        for char in parts[2].chars() {
            match char {
                'K' => board.castling_rights |= Board::WHITE_KING_SIDE_CASTLING_RIGHT,
                'Q' => board.castling_rights |= Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT,
                'k' => board.castling_rights |= Board::BLACK_KING_SIDE_CASTLING_RIGHT,
                'q' => board.castling_rights |= Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT,
                '-' => {}
                _ => panic!("Invalid castling rights in FEN string."),
            }
        }

        // Setup en passant target square
        match parts[3] {
            "-" => {
                board.en_passant_bb_position = 0;
            }
            pos => {
                let bb_position = to_bitboard_position(algebraic_to_square(pos) as u64);

                board.en_passant_bb_position = bb_position;

                if BBPositions::ROW_3.contains(&bb_position) {
                    board.en_passant_bb_piece_square = north_one(bb_position);
                } else if BBPositions::ROW_6.contains(&bb_position) {
                    board.en_passant_bb_piece_square = south_one(bb_position);
                }
            }
        };

        if parts.len() > 4 {
            board.half_move_clock = parts[4].parse::<u32>().unwrap();
        }

        if parts.len() > 5 {
            board.full_move_clock = parts[5].parse::<u32>().unwrap();
        }

        let mut zobrist = board.zobrist.clone();

        zobrist.compute_hash(&board);

        board.zobrist = zobrist;

        board
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

    pub fn set_winner(&mut self, winner: Option<Color>) { self.winner = winner; }

    pub fn get_winner(&self) -> Option<Color> { self.winner }

    pub fn is_game_finished(&self) -> bool { self.get_winner().is_some() }

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
        self.zobrist_history.push(self.zobrist.clone());
    }

    pub fn unmake_last_move(&mut self) {
        if self.castling_rights_history.len() == 0 {
            return;
        }

        self.castling_rights = self
            .castling_rights_history
            .remove(self.castling_rights_history.len() - 1);
        self.bitboards = self
            .bitboards_history
            .remove(self.bitboards_history.len() - 1);
        self.en_passant_bb_position = self
            .en_passant_bb_position_history
            .remove(self.en_passant_bb_position_history.len() - 1);
        self.en_passant_bb_piece_square = self
            .en_passant_bb_piece_square_history
            .remove(self.en_passant_bb_piece_square_history.len() - 1);
        self.black_king_moved = self
            .black_king_moved_history
            .remove(self.black_king_moved_history.len() - 1);
        self.white_king_moved = self
            .white_king_moved_history
            .remove(self.white_king_moved_history.len() - 1);
        self.zobrist = self.zobrist_history.remove(self.zobrist_history.len() - 1);

        if self.side_to_move.is_white() {
            self.full_move_clock -= 1;
        }

        self.half_move_clock -= 1;

        self.side_to_move = self.side_to_move.opponent();

        if self.winner.is_some() {
            self.winner = None;
        }
    }

    pub fn move_piece(&mut self, _move: &Move) {
        if self.get_piece_color(_move.get_from()) != self.side_to_move {
            panic!("Invalid player move, it's not your turn. {_move}")
        }

        self.save_current_state(_move.clone());

        let color = self.get_piece_color(_move.get_from());
        let mut piece_type = self.get_piece_type(_move.get_from());

        let from_square = _move.get_from();
        let to_square = _move.get_to();

        let from: u64 = 1 << from_square;
        let to: u64 = 1 << to_square;

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

        if self.en_passant_bb_position != 0 {
            // When a move is made and en passant is available, remove en passant option
            self.en_passant_bb_position = 0;
            self.en_passant_bb_piece_square = 0;
        }

        self.remove_piece(color, piece_type, from);

        if piece_type == PieceType::Pawn {
            if _move.is_double_pawn_push() {
                self.en_passant_bb_position = _move.get_en_passant_bb_position();
                self.en_passant_bb_piece_square = _move.get_en_passant_bb_piece_square();
            } else if is_pawn_promotion(color, from, to) {
                piece_type = get_piece_type_from_promotion_flag(_move.get_flags());
            }
        }

        let opponent_piece = self.get_piece_type(_move.get_to());

        if opponent_piece != PieceType::Empty {
            self.remove_piece(color.opponent(), opponent_piece, to);
        }

        self.place_piece(color, piece_type, to);

        if piece_type == PieceType::King {
            self.handle_king_move(color, _move);
        }

        if self.side_to_move.is_black() {
            self.full_move_clock += 1;
        }

        self.half_move_clock += 1;

        self.side_to_move = self.side_to_move.opponent();

        self.update_castling_rights(color, from, to);

        self.zobrist
            .update_hash_on_move(opponent_piece, color, from_square, piece_type, to_square);
    }

    fn update_castling_rights(&mut self, color: Color, from: u64, to: u64) {
        for &side in &[Side::KingSide, Side::QueenSide] {
            let (friendly_rook_pos, opponent_rook_pos) = match (color, side) {
                (Color::White, Side::KingSide) => (BBPositions::H1, BBPositions::H8),
                (Color::White, Side::QueenSide) => (BBPositions::A1, BBPositions::A8),
                (Color::Black, Side::KingSide) => (BBPositions::H8, BBPositions::H1),
                (Color::Black, Side::QueenSide) => (BBPositions::A8, BBPositions::A1),
            };

            let friendly_castling_right = get_castling_right_flag(color, side);

            if from == friendly_rook_pos && (self.castling_rights & friendly_castling_right) != 0 {
                self.castling_rights &= !friendly_castling_right;
                self.update_hash_on_lose_castle(color, side);
            }

            let opponent_castling_right = get_castling_right_flag(color.opponent(), side);

            if to == opponent_rook_pos && (self.castling_rights & opponent_castling_right) != 0 {
                self.castling_rights &= !opponent_castling_right;
                self.update_hash_on_lose_castle(color.opponent(), side);
            }
        }
    }

    fn update_hash_on_lose_castle(&mut self, color: Color, side: Side) {
        match (color, side) {
            (Color::White, Side::KingSide) => {
                self.zobrist.update_hash_on_white_lose_king_side_castle()
            }
            (Color::White, Side::QueenSide) => {
                self.zobrist.update_hash_on_white_lose_queen_side_castle()
            }
            (Color::Black, Side::KingSide) => {
                self.zobrist.update_hash_on_black_lose_king_side_castle()
            }
            (Color::Black, Side::QueenSide) => {
                self.zobrist.update_hash_on_black_lose_queen_side_castle()
            }
        }
    }

    fn handle_king_move(&mut self, color: Color, _move: &Move) {
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

    /// This function assumes that the square is not empty
    pub fn get_piece_color(&self, square: usize) -> Color {
        self.get_piece_color_by_bb_pos(1 << square)
    }

    /// This function assumes that the square is not empty
    pub fn get_piece_color_by_bb_pos(&self, bb_position: u64) -> Color {
        if self.bitboards[WHITE_IDX] & bb_position != 0 {
            return Color::White;
        }

        return Color::Black;
    }

    /// This function assumes that the square is not empty
    pub fn get_piece_type(&self, square: usize) -> PieceType {
        self.get_piece_type_by_bb_pos(1 << square)
    }

    /// This function assumes that the square is not empty
    pub fn get_piece_type_by_bb_pos(&self, bb_position: u64) -> PieceType {
        for piece_index in PIECE_INDEXES {
            if self.bitboards[piece_index] & bb_position != 0 {
                return get_piece_type_from_index(piece_index);
            }
        }

        PieceType::Empty
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Generate piece positions
        for row in (0..8).rev() {
            let mut empty_squares = 0;
            for col in 0..8 {
                let square = 8 * row + col;

                let piece_type = self.get_piece_type(square);

                if piece_type == PieceType::Empty {
                    empty_squares += 1;
                } else {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    let color = self.get_piece_color(square);

                    fen.push_str(&get_piece_letter(color, piece_type).as_str());
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if row > 0 {
                fen.push('/');
            }
        }

        // Add side to move
        fen.push(' ');
        fen.push_str(match self.side_to_move {
            Color::White => "w",
            Color::Black => "b",
        });

        // Add castling rights
        fen.push(' ');
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if self.castling_rights & 0x8 != 0 {
                fen.push('K');
            }
            if self.castling_rights & 0x4 != 0 {
                fen.push('Q');
            }
            if self.castling_rights & 0x2 != 0 {
                fen.push('k');
            }
            if self.castling_rights & 0x1 != 0 {
                fen.push('q');
            }
        }

        // Add en passant target square
        fen.push(' ');
        if self.en_passant_bb_position == 0 {
            fen.push('-');
        } else {
            let mut pos = self.en_passant_bb_position.clone();
            fen.push_str(&square_to_algebraic(pop_lsb(&mut pos) as usize)); // Assuming a helper for conversion
        }

        // Add halfmove clock
        fen.push(' ');
        fen.push_str(&self.half_move_clock.to_string());

        // Add fullmove number
        fen.push(' ');
        fen.push_str(&self.full_move_clock.to_string());

        fen
    }

    pub fn get_zobrist_hash(&self) -> u64 { self.zobrist.get_hash() }

    pub fn display(&self) { self.display_with_attacks(Vec::new()); }

    pub fn display_with_attacks(&self, attack_squares: Vec<usize>) {
        // println!(
        //     "\nen_passant_bb_position: {}",
        //     Squares::to_string(pop_lsb(&mut (self.en_passant_bb_position.clone())) as
        // usize) );
        // println!(
        //     "en_passant_bb_piece_square: {}",
        //     Squares::to_string(pop_lsb(&mut
        // (self.en_passant_bb_piece_square.clone())) as usize) );
        // println!("side_to_move: {}", self.side_to_move);
        // println!("black_king_moved: {}", self.black_king_moved);
        // println!("white_king_moved: {}", self.white_king_moved);

        // println!(
        //     "BLACK_KING_SIDE_CASTLING_RIGHT: {}",
        //     self.castling_rights & Board::BLACK_KING_SIDE_CASTLING_RIGHT != 0
        // );
        // println!(
        //     "BLACK_QUEEN_SIDE_CASTLING_RIGHT: {}",
        //     self.castling_rights & Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT != 0
        // );
        // println!(
        //     "WHITE_KING_SIDE_CASTLING_RIGHT: {}",
        //     self.castling_rights & Board::WHITE_KING_SIDE_CASTLING_RIGHT != 0
        // );
        // println!(
        //     "WHITE_QUEEN_SIDE_CASTLING_RIGHT: {}",
        //     self.castling_rights & Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT != 0
        // );

        println!("Zobrist: {:b}", self.zobrist.get_hash());
        println!("FEN: {}", self.to_fen());

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

fn get_castling_right_flag(color: Color, side: Side) -> u8 {
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
    use crate::game_bit_board::{
        _move::{_move::Move, move_contants::*},
        board::{Board, PAWNS_IDX, QUEENS_IDX},
        enums::{Color, PieceType},
        positions::{BBPositions, Squares},
    };

    #[test]
    fn test_castling() {
        // White side: king moved
        println!("\nTest - White side: king moved\n");
        let mut board = Board::new();

        assert_eq!(0xF, board.castling_rights);

        board.move_piece(&Move::dummy_from_to(Squares::D2, Squares::D4));
        board.move_piece(&Move::dummy_from_to(Squares::D7, Squares::D5));
        board.move_piece(&Move::dummy_from_to(Squares::E1, Squares::D2));

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

        board.move_piece(&Move::dummy_from_to(Squares::A1, Squares::B1));

        board.display();

        assert_eq!(
            0,
            board.castling_rights & Board::WHITE_QUEEN_SIDE_CASTLING_RIGHT
        );

        board.move_piece(&Move::dummy_from_to(Squares::H7, Squares::H6));
        board.move_piece(&Move::dummy_from_to(Squares::H1, Squares::G1));

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

        board.move_piece(&Move::dummy_from_to(Squares::E2, Squares::E3));
        board.move_piece(&Move::dummy_from_to(Squares::E8, Squares::D7));

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

        board.move_piece(&Move::dummy_from_to(Squares::A2, Squares::A3));
        board.move_piece(&Move::dummy_from_to(Squares::A8, Squares::B8));

        board.display();

        assert_eq!(
            0,
            board.castling_rights & Board::BLACK_QUEEN_SIDE_CASTLING_RIGHT
        );

        board.move_piece(&Move::dummy_from_to(Squares::A3, Squares::A4));
        board.move_piece(&Move::dummy_from_to(Squares::H8, Squares::G8));

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

        board.move_piece(&Move::dummy_from_to(Squares::D2, Squares::D4));
        board.move_piece(&Move::dummy_from_to(Squares::E7, Squares::E5));
        board.move_piece(&Move::dummy_from_to(Squares::D4, Squares::D5));

        let mut _move = Move::dummy_with_flags(DOUBLE_PAWN_PUSH, Squares::C7, Squares::C5);

        _move.set_en_passant_bb_position(BBPositions::C6);
        _move.set_en_passant_bb_piece_square(BBPositions::C5);

        board.move_piece(&_move);

        assert_eq!(PieceType::Pawn, board.get_piece_type(Squares::C5));

        board.display();

        board.move_piece(&Move::dummy_with_flags(
            EN_PASSANT,
            Squares::D5,
            Squares::C6,
        ));

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

        board.move_piece(&Move::dummy_from_to(Squares::A2, Squares::A4));

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

        board.move_piece(&Move::dummy_with_flags(QUEEN_PROMOTION, 55, 63));

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

        board.move_piece(&Move::dummy_from_to(8, 16));

        println!("After moving white pawn from a2 to a3:");

        board.display();
    }
}
