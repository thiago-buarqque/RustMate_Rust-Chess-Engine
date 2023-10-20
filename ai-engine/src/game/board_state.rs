use crate::common::{
    contants::EMPTY_PIECE,
    enums::PieceType,
    piece_utils::{is_piece_of_type, is_white_piece, piece_value_from_fen, pieces_to_fen},
};

use super::{
    contants::{
        BLACK_KING_INITIAL_POSITION, BLACK_KING_VALUE, LETTER_A_UNICODE,
        WHITE_KING_INITIAL_POSITION, WHITE_KING_VALUE,
    },
    zobrist::Zobrist,
};

#[derive(Debug, Clone)]
pub struct BoardState {
    black_able_to_king_side_castle: bool,
    black_able_to_queen_side_castle: bool,
    black_captures: Vec<i8>,
    black_en_passant: i8,
    black_king_moved: bool,
    black_king_position: i8,
    full_moves: i8,
    half_moves: i8,
    white_move: bool,
    squares: [i8; 64],
    white_able_to_king_side_castle: bool,
    white_able_to_queen_side_castle: bool,
    white_captures: Vec<i8>,
    white_en_passant: i8,
    white_king_moved: bool,
    white_king_position: i8,
    winner: i8,
    zobrist: Zobrist,
}

impl BoardState {
    pub fn new() -> Self {
        let mut zobrist = Zobrist::new();

        let mut board_state = BoardState {
            black_able_to_king_side_castle: true,
            black_able_to_queen_side_castle: true,
            black_captures: Vec::new(),
            black_en_passant: -1,
            black_king_moved: false,
            black_king_position: BLACK_KING_INITIAL_POSITION,
            full_moves: 0,
            half_moves: 0,
            white_move: true,
            squares: [0; 64],
            white_able_to_king_side_castle: true,
            white_able_to_queen_side_castle: true,
            white_captures: Vec::new(),
            white_en_passant: -1,
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

    pub fn clone(&self) -> Self {
        Self {
            black_able_to_king_side_castle: self.black_able_to_king_side_castle,
            black_able_to_queen_side_castle: self.black_able_to_queen_side_castle,
            black_captures: self.black_captures.clone(),
            black_en_passant: self.black_en_passant,
            black_king_moved: self.black_king_moved,
            black_king_position: self.black_king_position,
            full_moves: self.full_moves,
            half_moves: self.half_moves,
            white_move: self.white_move,
            squares: self.squares,
            white_able_to_king_side_castle: self.white_able_to_king_side_castle,
            white_able_to_queen_side_castle: self.white_able_to_queen_side_castle,
            white_captures: self.white_captures.clone(),
            white_en_passant: self.white_en_passant,
            white_king_moved: self.white_king_moved,
            white_king_position: self.white_king_position,
            winner: self.winner,
            zobrist: self.zobrist.clone(),
        }
    }

    pub fn is_able_to_castle_queen_side(&self, white_king: bool) -> bool {
        (white_king && self.is_white_able_to_queen_side_castle())
            || (!white_king && self.is_black_able_to_queen_side_castle())
    }

    pub fn is_able_to_castle_king_side(&self, white_king: bool) -> bool {
        (white_king && self.is_white_able_to_king_side_castle())
            || (!white_king && self.is_black_able_to_king_side_castle())
    }

    pub fn white_captures_to_fen(&self) -> Vec<char> {
        pieces_to_fen(&self.white_captures)
    }

    pub fn black_captures_to_fen(&self) -> Vec<char> {
        pieces_to_fen(&self.black_captures)
    }

    pub fn get_piece(&self, index: i8) -> i8 {
        if self.is_valid_position(index) {
            return self.squares[index as usize];
        }

        -1
    }

    pub fn place_piece(&mut self, index: i8, piece: i8) {
        if piece == BLACK_KING_VALUE {
            self.black_king_position = index;
        } else if piece == WHITE_KING_VALUE {
            self.white_king_position = index;
        }

        self.squares[index as usize] = piece;
    }

    pub fn move_piece(&mut self, from_index: i8, piece: i8, to_index: i8) {
        let moved_piece = self.get_piece(from_index);
        let captured_piece = self.get_piece(to_index);

        self.place_piece(to_index, piece);

        self.place_piece(from_index, EMPTY_PIECE);

        self.zobrist.update_hash_on_move(
            from_index as usize,
            to_index as usize,
            moved_piece,
            captured_piece,
        );

        if is_piece_of_type(captured_piece, PieceType::Empty) {
            return;
        }

        let is_white = is_white_piece(captured_piece);

        if is_white {
            self.append_black_capture(captured_piece);
        } else {
            self.append_white_capture(captured_piece);
        }
    }

    pub fn is_valid_position(&self, index: i8) -> bool {
        index >= 0 && index < self.squares.len() as i8
    }

    pub fn load_position(&mut self, fen_position: &str) {
        let fields: Vec<&str> = fen_position.split_whitespace().collect();

        self.generate_pieces_from_fen(fields[0]);
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
    }

    fn load_half_move_clock(&mut self, half_move: &str) {
        if let Ok(value) = half_move.parse::<i8>() {
            self.half_moves = value;
        } else {
            self.half_moves = 0;
        }
    }

    fn load_full_move_number(&mut self, moves: &str) {
        if let Ok(value) = moves.parse::<i8>() {
            self.full_moves = value;
        } else {
            self.full_moves = 0;
        }
    }

    fn load_en_passant(&mut self, en_passant: &str) {
        if en_passant == "-" {
            self.white_en_passant = -1;
            self.black_en_passant = -1;
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
                self.black_en_passant = -1;
            } else {
                self.black_en_passant = position as i8;
                self.white_en_passant = -1;
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

    fn generate_pieces_from_fen(&mut self, board_rows: &str) {
        let rows: Vec<&str> = board_rows.split('/').collect();

        let mut index = 0;

        for row in rows.iter() {
            self.generate_row_pieces_fen(row, &mut index);
        }
    }

    fn generate_row_pieces_fen(&mut self, row: &&str, index: &mut usize) {
        for char in row.chars() {
            if char.is_numeric() {
                *index += char.to_digit(10).unwrap() as usize;
            } else {
                self.squares[*index] = piece_value_from_fen(&char);

                if char == 'k' {
                    self.black_king_position = *index as i8;
                } else if char == 'K' {
                    self.white_king_position = *index as i8;
                }

                *index += 1;
            }
        }
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

    pub fn get_squares(&self) -> &[i8; 64] {
        &self.squares
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

    pub fn winner(&self) -> i8 {
        self.winner
    }

    pub fn set_winner(&mut self, value: i8) {
        self.winner = value;
    }

    pub fn set_white_move(&mut self, white_move: bool) {
        self.white_move = white_move;
    }

    pub fn increment_full_moves(&mut self) {
        self.full_moves += 1;
    }

    pub fn append_black_capture(&mut self, piece_value: i8) {
        self.black_captures.push(piece_value)
    }

    pub fn append_white_capture(&mut self, piece_value: i8) {
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
                if self.black_able_to_queen_side_castle {
                    self.zobrist.update_hash_on_black_lose_queen_side_castle();
                }

                self.black_able_to_queen_side_castle = false;
            }
            (7, true, true) => {
                if self.black_able_to_king_side_castle {
                    self.zobrist.update_hash_on_black_lose_rook_side_castle();
                }

                self.black_able_to_king_side_castle = false;
            }
            (56, false, false) => {
                if self.white_able_to_queen_side_castle {
                    self.zobrist.update_hash_on_white_lose_queen_side_castle()
                }

                self.white_able_to_queen_side_castle = false;
            }
            (63, false, true) => {
                if self.white_able_to_king_side_castle {
                    self.zobrist.update_hash_on_white_lose_rook_side_castle()
                }

                self.white_able_to_king_side_castle = false;
            }
            _ => {}
        }
    }
}
