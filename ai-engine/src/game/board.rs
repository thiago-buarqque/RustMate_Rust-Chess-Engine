use crate::common::{
    contants::{EMPTY_PIECE, INITIAL_FEN, INVALID_BOARD_POSITION}, enums::{PieceColor, PieceType}, piece::Piece, piece_move::PieceMove, piece_utils::{get_piece_type, is_piece_of_type, is_white_piece}
};

use super::{
    board_state::BoardState,
    contants::{
        BLACK_KING_SIDE_ROOK_POSITION, BLACK_QUEEN_SIDE_ROOK_POSITION,
        WHITE_KING_SIDE_ROOK_POSITION, WHITE_QUEEN_SIDE_ROOK_POSITION,
    },
    move_generator::MoveGenerator,
};

#[derive(Debug, Clone)]
pub struct Board {
    state: BoardState,
    state_history: Vec<BoardState>,
}

impl Board {
    pub fn new() -> Self {
        let mut state = BoardState::new();

        state.load_position(INITIAL_FEN);

        Board {
            state,
            state_history: Vec::new(),
        }
    }

    pub fn get_state_reference(&self) -> &BoardState {
        &self.state
    }

    fn get_move_generator(&self) -> MoveGenerator {
        MoveGenerator::new(self.state.clone())
    }

    pub fn get_pieces(&mut self) -> Vec<Piece> {
        let mut move_generator = self.get_move_generator();

        move_generator.get_available_moves(self)
    }

    pub fn set_winner(&mut self, is_king_in_check: bool, is_white_move: bool) {
        self.state.set_winner(if is_king_in_check {
            if is_white_move {
                PieceColor::Black.value()
            } else {
                PieceColor::White.value()
            }
        } else {
            PieceColor::Black.value() | PieceColor::White.value() // Draw
        });
    }

    pub fn get_winner_fen(&self) -> char {
        match self.state.get_winner() {
            x if x == (PieceColor::White.value()) => 'w',
            x if x == (PieceColor::Black.value()) => 'b',
            x if x == (PieceColor::Black.value() | PieceColor::White.value()) => 'd', // draw
            _ => '-',
        }
    }

    pub fn move_piece(&mut self, piece_move: &PieceMove) -> Result<(), &'static str> {
        self.state_history.push(self.state.clone());

        self._make_move(piece_move, false)
    }

    pub fn undo_last_move(&mut self) {
        if let Some(state) = self.state_history.pop() {
            self.state = state
        }
    }

    pub fn get_state_clone(&self) -> BoardState {
        self.state.clone()
    }

    pub fn get_zobrist_hash(&self) -> u64 {
        self.state.get_zobrist_hash()
    }

    fn _make_move(
        &mut self,
        piece_move: &PieceMove,
        rook_castling: bool,
    ) -> Result<(), &'static str> {
        let from_index = piece_move.get_from_position();
        let to_index = piece_move.get_to_position();

        if !self.state.is_valid_position(from_index) || !self.state.is_valid_position(to_index) {
            return Err("Invalid board position");
        }

        let mut moving_piece = self.state.get_piece(from_index);
        let existing_piece = self.state.get_piece(to_index);

        if let Some(invalid_result) = validate_move_pieces(moving_piece, existing_piece) {
            return invalid_result;
        }

        if piece_move.is_en_passant() {
            self.capture_en_passant(moving_piece);
        } else if piece_move.is_promotion() {
            if piece_move.get_promotion_value() == EMPTY_PIECE {
                return Err("Pawn needs promotion type.");
            }

            moving_piece = piece_move.get_promotion_value();
        } else if get_piece_type(moving_piece) == PieceType::King {
            self.handle_king_move(from_index, moving_piece, to_index);
        }

        self.state.move_piece(from_index, rook_castling, moving_piece, to_index);

        if !rook_castling {
            self.handle_state_update_after(from_index, moving_piece, to_index, existing_piece);
        }

        Ok(())
    }

    fn handle_state_update_after(
        &mut self,
        from_position: i8,
        moving_piece: u8,
        to_position: i8,
        replaced_piece: u8,
    ) {
        self.handle_en_passant(from_position, moving_piece, to_position);

        // Remove hook ability due to rook move
        if get_piece_type(moving_piece) == PieceType::Rook {
            self.state.update_castling_ability(
                from_position,
                from_position < 8,
                from_position % 8 == 7,
            );
        } else if get_piece_type(replaced_piece) == PieceType::Rook {
            self.state
                .update_castling_ability(to_position, to_position < 8, to_position % 8 == 7);
        }
    }

    fn handle_king_move(&mut self, from_position: i8, moving_piece: u8, to_position: i8) {
        let white_piece = is_white_piece(moving_piece);

        let is_castle_move = (from_position - to_position).abs() == 2;

        if is_castle_move
            && ((white_piece && !self.state.has_white_king_moved())
                || (!white_piece && !self.state.has_black_king_moved()))
        {
            let _ = self.castle(from_position, to_position, white_piece);
        }

        if white_piece {
            self.state.set_white_king_moved(true);
        } else {
            self.state.set_black_king_moved(true);
        }
    }

    fn castle(
        &mut self,
        from_index: i8,
        to_index: i8,
        white_piece: bool,
    ) -> Result<(), &'static str> {
        let (queen_side_rook_position, king_side_rook_position) = if white_piece {
            (
                WHITE_QUEEN_SIDE_ROOK_POSITION,
                WHITE_KING_SIDE_ROOK_POSITION,
            )
        } else {
            (
                BLACK_QUEEN_SIDE_ROOK_POSITION,
                BLACK_KING_SIDE_ROOK_POSITION,
            )
        };

        let rook_position = if from_index > to_index {
            queen_side_rook_position
        } else {
            king_side_rook_position
        };

        let new_rook_position = if from_index > to_index {
            from_index - 1
        } else {
            from_index + 1
        };

        if white_piece {
            self.state.set_white_able_to_queen_side_castle(false);
            self.state.set_white_able_to_king_side_castle(false);
        } else {
            self.state.set_black_able_to_queen_side_castle(false);
            self.state.set_black_able_to_king_side_castle(false);
        }

        let rook_move = PieceMove::new(
            rook_position,
            self.state.get_piece(rook_position),
            new_rook_position,
        );

        self._make_move(&rook_move, true)
    }

    fn capture_en_passant(&mut self, moving_piece: u8) {
        let white_piece = is_white_piece(moving_piece);

        if white_piece {
            let position = self.state.get_black_en_passant() + 8;
            let piece_value = self.state.get_piece(position);

            self.state.append_white_capture(piece_value);
            self.state.place_piece(position, EMPTY_PIECE);
            self.state.set_black_en_passant(INVALID_BOARD_POSITION);
        } else {
            let position = self.state.get_white_en_passant() - 8;
            let piece_value = self.state.get_piece(position);

            self.state.append_black_capture(piece_value);
            self.state.place_piece(position, EMPTY_PIECE);
            self.state.set_white_en_passant(INVALID_BOARD_POSITION);
        }
    }

    fn handle_en_passant(&mut self, from_position: i8, piece_value: u8, to_position: i8) {
        self.state.set_white_en_passant(INVALID_BOARD_POSITION);
        self.state.set_black_en_passant(INVALID_BOARD_POSITION);

        if !is_piece_of_type(piece_value, PieceType::Pawn) {
            return;
        }

        let white_piece = is_white_piece(piece_value);

        if white_piece {
            // magic numbers...
            if (48..=55).contains(&from_position) && (32..=39).contains(&to_position) {
                self.state.set_white_en_passant(to_position + 8);
            }
        } else {
            // magic numbers...
            if (8..=15).contains(&from_position) && (24..=31).contains(&to_position) {
                self.state.set_black_en_passant(to_position - 8);
            }
        }
    }

    pub fn load_position(&mut self, fen_position: &str) {
        let mut state = BoardState::new();

        state.load_position(fen_position);

        self.state = state;
    }

    pub fn black_captures_to_fen(&self) -> Vec<char> {
        self.state.get_black_captures_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<char> {
        self.state.get_white_captures_fen()
    }

    pub fn is_white_move(&self) -> bool {
        self.state.is_white_move()
    }

    pub fn is_game_finished(&self) -> bool {
        self.get_winner_fen() != '-'
    }
}

fn validate_move_pieces(moving_piece: u8, existing_piece: u8) -> Option<Result<(), &'static str>> {
    if moving_piece == EMPTY_PIECE {
        return Some(Err("No piece at the position"));
    } 
    
    if get_piece_type(existing_piece) == PieceType::King {
        println!("Can't capture king!");
        return Some(Err("Can't capture king at position"));
    }

    None
}
