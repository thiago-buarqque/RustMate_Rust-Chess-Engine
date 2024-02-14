use crate::common::{
    contants::{EMPTY_PIECE, INVALID_BOARD_POSITION}, enums::PieceType, fen_utils::{get_fen_piece_value, translate_pieces_to_fen}, piece_utils::{is_piece_of_type, is_white_piece}
};

use super::{
    board_fen_utils::get_position_fen, contants::{
        BLACK_KING_INITIAL_POSITION, BLACK_KING_VALUE, LETTER_A_UNICODE,
        WHITE_KING_INITIAL_POSITION, WHITE_KING_VALUE,
    }, zobrist::Zobrist
};

#[derive(Debug, Clone)]
pub struct BoardState {
    black_able_to_king_side_castle: bool,
    black_able_to_queen_side_castle: bool,
    black_captures: Vec<u8>,
    black_en_passant: i8,
    black_king_in_check: bool,
    black_king_moved: bool,
    black_king_position: i8,
    full_moves: usize,
    half_moves: usize,
    white_move: bool,
    squares: [u8; 64],
    white_able_to_king_side_castle: bool,
    white_able_to_queen_side_castle: bool,
    white_captures: Vec<u8>,
    white_en_passant: i8,
    white_king_in_check: bool,
    white_king_moved: bool,
    white_king_position: i8,
    winner: u8,
    zobrist: Zobrist,
}

impl BoardState {
    pub fn new() -> Self {
        let mut zobrist = Zobrist::new();

        let mut board_state = BoardState {
            black_able_to_king_side_castle: true,
            black_able_to_queen_side_castle: true,
            black_captures: Vec::with_capacity(16),
            black_en_passant: INVALID_BOARD_POSITION,
            black_king_in_check: false,
            black_king_moved: false,
            black_king_position: BLACK_KING_INITIAL_POSITION,
            full_moves: 0,
            half_moves: 0,
            white_move: true,
            squares: [0; 64],
            white_able_to_king_side_castle: true,
            white_able_to_queen_side_castle: true,
            white_captures: Vec::with_capacity(16),
            white_en_passant: INVALID_BOARD_POSITION,
            white_king_in_check: false,
            white_king_moved: false,
            white_king_position: WHITE_KING_INITIAL_POSITION,
            winner: 0,
            zobrist,
        };

        zobrist = Zobrist::new();

        zobrist.compute_hash(&board_state);

        board_state.zobrist = zobrist;

        board_state
    }

    pub fn is_able_to_castle_queen_side(&self, white_king: bool) -> bool {
        (white_king && self.is_white_able_to_queen_side_castle())
            || (!white_king && self.is_black_able_to_queen_side_castle())
    }

    pub fn is_able_to_castle_king_side(&self, white_king: bool) -> bool {
        (white_king && self.is_white_able_to_king_side_castle())
            || (!white_king && self.is_black_able_to_king_side_castle())
    }

    pub fn get_white_captures_fen(&self) -> Vec<char> {
        translate_pieces_to_fen(&self.white_captures)
    }

    pub fn get_black_captures_fen(&self) -> Vec<char> {
        translate_pieces_to_fen(&self.black_captures)
    }

    pub fn get_piece(&self, position: i8) -> u8 {
        if self.is_valid_position(position) {
            return self.squares[position as usize];
        }

        EMPTY_PIECE
    }

    pub fn place_piece(&mut self, position: i8, piece: u8) {
        if piece == BLACK_KING_VALUE {
            self.black_king_position = position;
        } else if piece == WHITE_KING_VALUE {
            self.white_king_position = position;
        }

        self.squares[position as usize] = piece;
    }

    pub fn move_piece(&mut self, from_position: i8, rook_castling: bool, piece: u8, to_position: i8) {
        let moved_piece = self.get_piece(from_position);
        let captured_piece = self.get_piece(to_position);

        self.place_piece(to_position, piece);

        self.place_piece(from_position, EMPTY_PIECE);

        self.zobrist.update_hash_on_move(
            from_position as usize,
            to_position as usize,
            moved_piece,
            captured_piece,
        );

        if !rook_castling {
            if !self.is_white_move() {
                self.increment_full_moves();
            }
    
            self.set_white_move(!self.is_white_move());
    
            if is_piece_of_type(moved_piece, PieceType::Pawn) {
                self.half_moves = 0;
            } else {
                self.half_moves += 1;
            }
        }

        if is_piece_of_type(captured_piece, PieceType::Empty) {
            return;
        }

        self.half_moves = 0;

        let is_white = is_white_piece(captured_piece);

        if is_white {
            self.append_black_capture(captured_piece);
        } else {
            self.append_white_capture(captured_piece);
        }
    }

    pub fn is_valid_position(&self, position: i8) -> bool {
        position >= 0 && position < self.squares.len() as i8
    }

    pub fn load_position(&mut self, fen_position: &str) {        
        let fields: Vec<&str> = fen_position.split_whitespace().collect();

        self.load_pieces(fields[0]);
        self.load_active_color(fields[1]);
        self.load_castling(fields[2]);
        self.load_en_passant(fields[3]);

        // Ideally every fen shoul have all fields, but sometimes
        // I copy some that don't.
        if fields.len() > 4 {
            self.load_half_move_clock(fields[4]);

            if fields.len() > 5 {
                self.load_full_move_number(fields[5]);
            }
        }

        self.zobrist.compute_hash(&self.clone());
    }

    fn load_half_move_clock(&mut self, half_move: &str) {
        if let Ok(value) = half_move.parse::<usize>() {
            self.half_moves = value;
        } else {
            self.half_moves = 0;
        }
    }

    fn load_full_move_number(&mut self, moves: &str) {
        if let Ok(value) = moves.parse::<usize>() {
            self.full_moves = value;
        } else {
            self.full_moves = 0;
        }
    }

    fn load_en_passant(&mut self, en_passant: &str) {
        if en_passant == "-" {
            self.white_en_passant = INVALID_BOARD_POSITION;
            self.black_en_passant = INVALID_BOARD_POSITION;
        } else {
            let column = en_passant.chars().nth(0).unwrap();
            let row: u8 = en_passant.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

            let mut is_white = false;

            let row = if row == 3 {
                is_white = true;
                4
            } else {
                3
            };

            let position = (column as u8 - LETTER_A_UNICODE + (row * 8)) - 8;

            if is_white {
                self.white_en_passant = position as i8;
                self.black_en_passant = INVALID_BOARD_POSITION;
            } else {
                self.black_en_passant = position as i8;
                self.white_en_passant = INVALID_BOARD_POSITION;
            }
        }
    }

    fn load_castling(&mut self, castling: &str) {
        if castling == "-" {
            self.black_able_to_queen_side_castle = false;
            self.black_able_to_king_side_castle = false;
            self.white_able_to_queen_side_castle = false;
            self.white_able_to_king_side_castle = false;
            self.black_king_moved = true;
            self.white_king_moved = true;
        } else {
            self.white_able_to_king_side_castle = castling.contains('K');
            self.white_able_to_queen_side_castle = castling.contains('Q');
            self.black_able_to_king_side_castle = castling.contains('k');
            self.black_able_to_queen_side_castle = castling.contains('q');

            self.white_king_moved = false;
            self.black_king_moved = false;
        }
    }

    fn load_active_color(&mut self, active_color: &str) {
        match active_color {
            "w" => self.white_move = true,
            "b" => self.white_move = false,
            _ => self.white_move = true,
        }
    }

    fn load_pieces(&mut self, board_rows: &str) {
        let rows: Vec<&str> = board_rows.split('/').collect();

        let mut index: u8 = 0;

        for row in rows.iter() {
            self.generate_row_pieces_fen(row, &mut index);
        }
    }

    fn generate_row_pieces_fen(&mut self, row: &&str, index: &mut u8) {
        for char in row.chars() {
            if char.is_numeric() {
                *index += char.to_digit(10).unwrap() as u8;
            } else {
                self.squares[*index as usize] = get_fen_piece_value(&char);

                if char == 'k' {
                    self.black_king_position = *index as i8;
                } else if char == 'K' {
                    self.white_king_position = *index as i8;
                }

                *index += 1;
            }
        }
    }

    pub fn get_fen(&self) -> String {
        get_position_fen(&self)
    }

    pub fn is_black_able_to_king_side_castle(&self) -> bool {
        self.black_able_to_king_side_castle
    }

    pub fn is_black_able_to_queen_side_castle(&self) -> bool {
        self.black_able_to_queen_side_castle
    }

    pub fn get_black_en_passant(&self) -> i8 {
        self.black_en_passant
    }

    pub fn has_black_king_moved(&self) -> bool {
        self.black_king_moved
    }

    pub fn get_black_king_position(&self) -> i8 {
        self.black_king_position
    }

    pub fn get_white_king_position(&self) -> i8 {
        self.white_king_position
    }

    pub fn is_white_move(&self) -> bool {
        self.white_move
    }

    pub fn get_squares(&self) -> &[u8; 64] {
        &self.squares
    }

    pub fn get_half_moves(&self) -> usize {
        self.half_moves
    }

    pub fn get_full_moves(&self) -> usize {
        self.full_moves
    }

    pub fn is_white_able_to_king_side_castle(&self) -> bool {
        self.white_able_to_king_side_castle
    }

    pub fn is_white_able_to_queen_side_castle(&self) -> bool {
        self.white_able_to_queen_side_castle
    }

    pub fn get_white_en_passant(&self) -> i8 {
        self.white_en_passant
    }

    pub fn get_zobrist_hash(&self) -> u64 {
        self.zobrist.get_hash()
    }

    pub fn has_white_king_moved(&self) -> bool {
        self.white_king_moved
    }

    pub fn get_winner(&self) -> u8 {
        self.winner
    }

    pub fn set_black_king_in_check(&mut self, black_king_in_check: bool) {
        self.black_king_in_check = black_king_in_check;
    }

    pub fn set_white_king_in_check(&mut self, white_king_in_check: bool) {        
        self.white_king_in_check = white_king_in_check;
    }

    pub fn is_black_king_in_check(&self) -> bool {
        self.black_king_in_check
    }

    pub fn is_white_king_in_check(&self) -> bool {
        self.white_king_in_check
    }

    pub fn set_winner(&mut self, value: u8) {
        self.winner = value;
    }

    pub fn set_white_move(&mut self, white_move: bool) {
        self.white_move = white_move;
    }

    pub fn increment_full_moves(&mut self) {
        self.full_moves += 1;
    }

    pub fn append_black_capture(&mut self, piece_value: u8) {
        self.black_captures.push(piece_value)
    }

    pub fn append_white_capture(&mut self, piece_value: u8) {
        self.white_captures.push(piece_value)
    }

    pub fn set_black_en_passant(&mut self, value: i8) {
        if self.black_en_passant != value {
            self.zobrist.update_hash_on_black_en_passant_change();
        }

        self.black_en_passant = value;
    }

    pub fn set_white_en_passant(&mut self, value: i8) {
        if self.white_en_passant != value {
            self.zobrist.update_hash_on_white_en_passant_change();
        }

        self.white_en_passant = value;
    }

    pub fn set_black_king_moved(&mut self, value: bool) {
        self.black_king_moved = value;
    }

    pub fn set_white_king_moved(&mut self, value: bool) {
        self.white_king_moved = value;
    }

    pub fn set_white_able_to_king_side_castle(&mut self, value: bool) {
        self.white_able_to_king_side_castle = value;
    }

    pub fn set_white_able_to_queen_side_castle(&mut self, value: bool) {
        self.white_able_to_queen_side_castle = value;
    }

    pub fn set_black_able_to_king_side_castle(&mut self, value: bool) {
        self.black_able_to_king_side_castle = value;
    }

    pub fn set_black_able_to_queen_side_castle(&mut self, value: bool) {
        self.black_able_to_queen_side_castle = value;
    }

    pub fn update_castling_ability(&mut self, index: i8, is_black: bool, is_king_side: bool) {
        match (index, is_black, is_king_side) {
            (0, true, false) => {
                // BLACK_QUEEN_SIDE_ROOK_POSITION
                if self.black_able_to_queen_side_castle {
                    self.zobrist.update_hash_on_black_lose_queen_side_castle();
                }

                self.black_able_to_queen_side_castle = false;
            }
            (7, true, true) => {
                // BLACK_KING_SIDE_ROOK_POSITION
                if self.black_able_to_king_side_castle {
                    self.zobrist.update_hash_on_black_lose_rook_side_castle();
                }

                self.black_able_to_king_side_castle = false;
            }
            (56, false, false) => {
                // WHITE_QUEEN_SIDE_ROOK_POSITION
                if self.white_able_to_queen_side_castle {
                    self.zobrist.update_hash_on_white_lose_queen_side_castle()
                }

                self.white_able_to_queen_side_castle = false;
            }
            (63, false, true) => {
                // WHITE_KING_SIDE_ROOK_POSITION
                if self.white_able_to_king_side_castle {
                    self.zobrist.update_hash_on_white_lose_rook_side_castle()
                }

                self.white_able_to_king_side_castle = false;
            }
            _ => {}
        }
    }
}
