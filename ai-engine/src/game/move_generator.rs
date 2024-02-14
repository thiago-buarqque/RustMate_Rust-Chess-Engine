use crate::common::{
    contants::{EMPTY_PIECE, INVALID_BOARD_POSITION}, enums::{PieceColor, PieceType}, piece::Piece, piece_move::PieceMove, piece_utils::{get_piece_type, is_same_color, is_white_piece}
};

use super::{
    board::Board,
    board_state::BoardState,
    contants::{
        BLACK_KING_ROOK_POSITION, BLACK_PAWN_VALUE, BLACK_QUEEN_ROOK_POSITION,
        WHITE_KING_ROOK_POSITION, WHITE_PAWN_VALUE, WHITE_QUEEN_ROOK_POSITION,
    },
    move_generator_helper::{
        get_adjacent_position, get_knight_move, is_diagonal_offset, is_king_in_check, is_king_straight_attacked_by_sliding_piece, is_path_clear, is_pawn_first_move, is_the_same_diagonal, position_is_not_attacked
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SquareOffset {
    LineBelow = 8,
    LineAbove = -8,
    TopRight = -7,
    TopLeft = -9,
    BottomRight = 9,
    BottomLeft = 7,
    Left = -1,
    Right = 1,
}

impl SquareOffset {
    fn value(&self) -> i8 {
        *self as i8
    }
}

#[derive(Debug, Clone)]
pub struct MoveGenerator {
    board_state: BoardState,
}

impl MoveGenerator {
    pub fn new(board_state: BoardState) -> Self {
        Self { board_state }
    }

    pub fn get_board_state(&self) -> BoardState {
        self.board_state.clone()
    }

    fn generate_moves(&self, piece_type: PieceType, position: i8) -> Vec<PieceMove> {
        // King moves are generated after generating the other pieces moves
        match piece_type {
            PieceType::Bishop => self.generate_bishop_moves(position),
            PieceType::Knight => self.generate_knight_moves(position),
            PieceType::Pawn => self.generate_pawn_moves(position),
            PieceType::Queen => self.generate_queen_moves(position),
            PieceType::Rook => self.generate_rook_moves(position),
            _ => vec![],
        }
    }

    pub fn get_available_moves(&mut self, board: &mut Board) -> Vec<Piece> {
        self.board_state.set_white_king_in_check(false);
        self.board_state.set_black_king_in_check(false);

        let (black_moves, white_moves, mut pieces) = self.generate_pieces_moves(board);

        // Only the two kings on the board
        if black_moves.is_empty() && white_moves.is_empty() && self.has_only_the_two_kings(&pieces)
        {
            // Game ends in draw
            board.set_winner(false, board.is_white_move());
        } else {
            self.get_king_available_moves(&black_moves, &mut pieces, &white_moves);

            self.remove_locked_and_invalid_moves(&mut pieces, board);
        }

        pieces
    }

    fn has_only_the_two_kings(&self, pieces: &Vec<Piece>) -> bool {
        pieces.iter().map(|piece| piece.get_value()).sum::<u8>()
            == ((PieceColor::Black.value() | PieceType::King.value())
                + (PieceColor::White.value() | PieceType::King.value()))
    }

    fn generate_pieces_moves(
        &mut self,
        board: &mut Board,
    ) -> (Vec<PieceMove>, Vec<PieceMove>, Vec<Piece>) {
        let mut black_moves: Vec<PieceMove> = vec![];
        let mut white_moves: Vec<PieceMove> = vec![];
        let mut pieces: Vec<Piece> = Vec::with_capacity(64);

        let squares = board.get_state_reference().get_squares();

        for (position, &piece_value) in squares.iter().enumerate() {
            let mut moves = vec![];
            let white_piece = is_white_piece(piece_value);
            let piece_type = get_piece_type(piece_value);

            if piece_type != PieceType::Empty && piece_type != PieceType::King {
                moves = self.generate_moves(piece_type, position as i8);

                if white_piece {
                    white_moves.extend(moves.clone());
                } else {
                    black_moves.extend(moves.clone());
                }
            }

            pieces.push(Piece::new(
                moves,
                position as i8,
                piece_value,
                white_piece,
            ));
        }
        (black_moves, white_moves, pieces)
    }

    fn get_king_available_moves(
        &mut self,
        black_moves: &[PieceMove],
        board_pieces: &mut [Piece],
        white_moves: &[PieceMove],
    ) {
        let white_king_position = self.board_state.get_white_king_position();
        let black_king_position = self.board_state.get_black_king_position();

        let mut white_king_moves = self.generate_king_raw_moves(black_moves, white_king_position);
        let mut black_king_moves = self.generate_king_raw_moves(white_moves, black_king_position);

        let common_moves: Vec<PieceMove> = white_king_moves
            .iter()
            .cloned()
            .filter(|x| black_king_moves.contains(x))
            .collect();

        white_king_moves.retain(|x| !common_moves.contains(x));
        black_king_moves.retain(|x| !common_moves.contains(x));

        // TODO the king should always be present or am i going to allow FEN with no kings?
        board_pieces[white_king_position as usize].set_moves(white_king_moves);
        board_pieces[black_king_position as usize].set_moves(black_king_moves);
    }

    pub fn remove_locked_and_invalid_moves(
        &mut self,
        pieces: &mut [Piece],
        board: &mut Board,
    ) {
        let (white_move, king_position) = self.determine_move_and_king_position();

        let mut no_moves_available: bool = true;

        let king_in_check = is_king_in_check(pieces, king_position, white_move);

        if king_in_check {
            if white_move {
                self.board_state.set_white_king_in_check(true);
            } else {
                self.board_state.set_black_king_in_check(true);
            }
        }

        for board_piece in pieces.iter_mut() {
            self.filter_invalid_moves(
                board_piece,
                king_position,
                board,
                &mut no_moves_available,
                white_move,
            );
        }

        if no_moves_available {
            board.set_winner(king_in_check, white_move);
        }
    }

    fn determine_move_and_king_position(&self) -> (bool, i8) {
        let is_white_move = self.board_state.is_white_move();

        let king_position = if is_white_move {
            self.board_state.get_white_king_position()
        } else {
            self.board_state.get_black_king_position()
        };

        (is_white_move, king_position)
    }

    fn filter_invalid_moves(
        &mut self,
        piece: &mut Piece,
        king_position: i8,
        board: &mut Board,
        no_moves_available: &mut bool,
        white_move: bool,
    ) {
        let piece_value = piece.get_value();

        if piece_value == EMPTY_PIECE || piece.is_white() != white_move {
            return;
        }

        let mut valid_moves = Vec::new();

        for piece_move in piece.get_moves_reference() {
            let is_king = get_piece_type(piece_value) == PieceType::King;

            self.validate_move(
                piece_move,
                king_position,
                board,
                &mut valid_moves,
                is_king,
                piece_value,
            );
        }

        if !valid_moves.is_empty() {
            *no_moves_available = false;
        }

        piece.set_moves(valid_moves);
    }

    fn validate_move(
        &mut self,
        piece_move: &PieceMove,
        king_position: i8,
        board: &mut Board,
        valid_moves: &mut Vec<PieceMove>,
        is_king: bool,
        piece_value: u8,
    ) {
        let mut piece_move_clone = piece_move.clone();

        let target_position = if is_king {
            piece_move.get_to_position()
        } else {
            king_position
        };

        if piece_move_clone.is_promotion() {
            piece_move_clone.set_promotion_value(piece_value);
        }

        let _ = board.move_piece(&piece_move_clone);

        if !self.is_king_attacked_on_state(target_position, board.get_state_clone()) {
            valid_moves.push(piece_move.clone());
        }

        board.undo_last_move();
    }

    fn is_king_attacked_on_state(
        &mut self,
        king_position: i8,
        new_board_state: BoardState,
    ) -> bool {
        let old_board_state = self.board_state.clone();

        // Set new state for the functions that reads it to generate the moves
        self.board_state = new_board_state;

        let is_white_move = self.board_state.is_white_move();

        let squares = self.board_state.get_squares();

        for (position, &piece) in squares.iter().enumerate() {
            if (piece == (EMPTY_PIECE)) || is_white_piece(piece) != is_white_move {
                continue;
            }

            let piece_type = get_piece_type(piece);

            let moves = if piece_type != PieceType::King {
                self.generate_moves(piece_type, position as i8)
            } else {
                // Generates the current player king possible moves, even if they're invalid.
                // This is just to prevent kings to be aside with each other
                self.generate_king_raw_moves(&Vec::new(), position as i8)
            };

            if moves
                .iter()
                .any(|_move| _move.get_to_position() == king_position)
            {
                self.board_state = old_board_state;

                return true;
            }
        }

        self.board_state = old_board_state;

        false
    }

    pub fn generate_knight_moves(&self, position: i8) -> Vec<PieceMove> {
        let positions = [
            get_knight_move(2, position - 17, position),
            get_knight_move(2, position - 15, position),
            get_knight_move(1, position - 10, position),
            get_knight_move(1, position - 6, position),
            get_knight_move(1, position + 6, position),
            get_knight_move(1, position + 10, position),
            get_knight_move(2, position + 15, position),
            get_knight_move(2, position + 17, position),
        ];

        let mut moves: Vec<PieceMove> = Vec::with_capacity(8);

        let knight_piece = self.board_state.get_piece(position);

        for new_position in positions {
            if !self.board_state.is_valid_position(new_position) {
                continue;
            }

            let existing_piece = self.board_state.get_piece(new_position);

            let same_color = is_same_color(knight_piece, existing_piece);

            if existing_piece == EMPTY_PIECE || !same_color {
                let mut _move = PieceMove::new(position, knight_piece, new_position);

                _move.set_is_capture(!same_color && existing_piece != EMPTY_PIECE);

                moves.push(_move);
            }
        }

        moves
    }

    pub fn generate_king_raw_moves(
        &self,
        opponent_moves: &[PieceMove],
        king_position: i8,
    ) -> Vec<PieceMove> {
        let positions = [
            get_adjacent_position(king_position, king_position - 1),
            get_adjacent_position(king_position, king_position + 1),
            get_adjacent_position(king_position, king_position - 9),
            get_adjacent_position(king_position, king_position - 8),
            get_adjacent_position(king_position, king_position - 7),
            get_adjacent_position(king_position, king_position + 7),
            get_adjacent_position(king_position, king_position + 8),
            get_adjacent_position(king_position, king_position + 9),
        ];

        let mut moves: Vec<PieceMove> = Vec::with_capacity(8);

        let king = self.board_state.get_piece(king_position);

        for position in positions {
            if !self.board_state.is_valid_position(position) {
                continue;
            }

            self.add_new_king_position(opponent_moves, position, &mut moves, king_position, king);
        }

        if !opponent_moves
            .iter()
            .any(|_move| _move.get_to_position() == king_position)
        {
            self.generate_castle_moves(king, &mut moves, opponent_moves, king_position);
        }

        moves
    }

    fn add_new_king_position(
        &self,
        opponent_moves: &[PieceMove],
        position: i8,
        moves: &mut Vec<PieceMove>,
        king_position: i8,
        king_value: u8,
    ) {
        if let Some(attacking_move) = opponent_moves
            .iter()
            .find(|_move| _move.get_to_position() == position)
        {
            let possible_pawn = self
                .board_state
                .get_piece(attacking_move.get_from_position());

            // Is a pawn straight attacking the position?
            // The king can move in front of the pawn
            if get_piece_type(possible_pawn) == PieceType::Pawn {
                moves.push(PieceMove::new(king_position, king_value, position));
            }
        } else {
            let piece_value = self.board_state.get_piece(position);

            let same_color = is_same_color(king_value, piece_value);

            if piece_value == EMPTY_PIECE || !same_color {
                let mut _move = PieceMove::new(king_position, king_value, position);

                _move.set_is_capture(!same_color && piece_value != EMPTY_PIECE);

                moves.push(_move);
            }
        }
    }

    fn generate_castle_moves(
        &self,
        king_piece: u8,
        moves: &mut Vec<PieceMove>,
        opponent_moves: &[PieceMove],
        king_position: i8,
    ) {
        let is_white_king = is_white_piece(king_piece);
        let has_king_moved = if is_white_king {
            self.board_state.has_white_king_moved()
        } else {
            self.board_state.has_black_king_moved()
        };

        if has_king_moved {
            return;
        }

        let (queen_side_rook_position, king_side_rook_position) = if is_white_king {
            (WHITE_QUEEN_ROOK_POSITION, WHITE_KING_ROOK_POSITION)
        } else {
            (BLACK_QUEEN_ROOK_POSITION, BLACK_KING_ROOK_POSITION)
        };

        let able_to_castle_queen_side =
            self.board_state.is_able_to_castle_queen_side(is_white_king);
        let able_to_castle_king_side = self.board_state.is_able_to_castle_king_side(is_white_king);

        // Check for queen side castling
        if able_to_castle_queen_side
            && is_path_clear(
                &self.board_state,
                king_position - 1,
                queen_side_rook_position,
                -1,
            )
        {
            self.generate_castle_move(
                king_position,
                king_piece,
                false,
                king_position - 2,
                opponent_moves,
                moves,
            );
        }

        // Check for king side castling
        if able_to_castle_king_side
            && is_path_clear(
                &self.board_state,
                king_position + 1,
                king_side_rook_position,
                1,
            )
        {
            self.generate_castle_move(
                king_position,
                king_piece,
                true,
                king_position + 2,
                opponent_moves,
                moves,
            );
        }
    }

    fn generate_castle_move(
        &self,
        king_position: i8,
        king_value: u8,
        is_king_side: bool,
        new_position: i8,
        opponent_moves: &[PieceMove],
        moves: &mut Vec<PieceMove>,
    ) {
        if !position_is_not_attacked(new_position, opponent_moves)
            || !position_is_not_attacked(
                king_position + if new_position < king_position { -1 } else { 1 },
                opponent_moves,
            )
        {
            return;
        }

        let is_white_king = is_white_piece(king_value);

        let offset = if is_white_king { -8 } else { 8 };

        let frontal_pawn_position = king_position + offset;
        let side_diagonal_pawn_position = if is_king_side {
            frontal_pawn_position + 2
        } else {
            frontal_pawn_position - 2
        };

        let pawn_value = if is_white_king {
            BLACK_PAWN_VALUE
        } else {
            WHITE_PAWN_VALUE
        };

        // Check if there is pawn attacking the side squares
        if self.board_state.get_piece(frontal_pawn_position) != pawn_value
            && self.board_state.get_piece(side_diagonal_pawn_position) != pawn_value
        {
            moves.push(PieceMove::new(king_position, king_value, new_position));
        }
    }

    pub fn generate_queen_moves(&self, position: i8) -> Vec<PieceMove> {
        let mut moves = Vec::with_capacity(27);

        moves.extend(self.generate_bishop_moves(position));
        moves.extend(self.generate_rook_moves(position));

        moves
    }

    pub fn generate_bishop_moves(&self, position: i8) -> Vec<PieceMove> {
        let piece = self.board_state.get_piece(position);

        let mut moves: Vec<PieceMove> = Vec::with_capacity(14);

        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::TopLeft);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::TopRight);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::BottomLeft);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::BottomRight);

        moves
    }

    pub fn generate_rook_moves(&self, position: i8) -> Vec<PieceMove> {
        let piece = self.board_state.get_piece(position);

        let mut moves = Vec::with_capacity(14);

        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::LineAbove);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::Left);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::Right);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::LineBelow);

        moves
    }

    fn generate_sliding_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        piece: u8,
        position: i8,
        offset: SquareOffset,
    ) {
        for i in 0..7 {
            // Should go somewhere on the right but is on the right side of the board
            if (offset == SquareOffset::BottomRight
                || offset == SquareOffset::TopRight
                || offset == SquareOffset::Right)
                && (position + 1) % 8 == 0
            {
                break;
            }

            // Should go somewhere on the left but is on the left side of the board
            if (offset == SquareOffset::BottomLeft
                || offset == SquareOffset::TopLeft
                || offset == SquareOffset::Left)
                && position % 8 == 0
            {
                break;
            }

            let new_position = position + ((i + 1) * offset.value());

            if !self.board_state.is_valid_position(new_position) {
                break;
            }

            let white_piece = is_white_piece(piece);

            let existing_piece = self.board_state.get_piece(new_position);

            if existing_piece == EMPTY_PIECE {
                let mut _move = PieceMove::new(position, piece, new_position);

                if white_piece == self.board_state.is_white_move() {
                    let opponent_king_position = if self.board_state.is_white_move() {
                        self.board_state.get_black_king_position()
                    } else {
                        self.board_state.get_white_king_position()
                    };

                    let diagonal_offset = is_diagonal_offset(offset);

                    if diagonal_offset && is_the_same_diagonal(new_position, opponent_king_position) {
                        _move.set_puts_king_in_check(true);
                    } else if !diagonal_offset && 
                        is_king_straight_attacked_by_sliding_piece(&self.board_state, opponent_king_position, new_position) {
                        
                        _move.set_puts_king_in_check(true);
                    }
                }

                moves.push(_move);
            } else if !is_same_color(piece, existing_piece) {
                let mut _move = PieceMove::new(position, piece, new_position);

                _move.set_is_capture(true);

                moves.push(_move);

                break;
            } else {
                break;
            }

            if offset == SquareOffset::LineAbove || offset == SquareOffset::LineBelow {
                continue;
            }

            let righty_offset = offset == SquareOffset::Right
                || offset == SquareOffset::TopRight
                || offset == SquareOffset::BottomRight;

            // Arrived at the edge of the board
            if (new_position + if righty_offset { 1 } else { 0 }) % 8 == 0 {
                break;
            }
        }
    }

    pub fn generate_pawn_moves(&self, position: i8) -> Vec<PieceMove> {
        let white_piece = is_white_piece(self.board_state.get_piece(position));

        let offset = if white_piece { -8 } else { 8 };

        let next_line_position = position + offset;

        // The pawn should be already promoted
        if !self.board_state.is_valid_position(next_line_position) {
            return Vec::new();
        }

        let mut moves: Vec<PieceMove> = Vec::with_capacity(4);

        let pawn_value = self.board_state.get_piece(position);

        self.generate_pawn_regular_moves(
            &mut moves,
            next_line_position,
            offset,
            pawn_value,
            position,
            white_piece,
        );

        self.generate_pawn_capturing_moves(
            &mut moves,
            next_line_position,
            pawn_value,
            position,
            white_piece,
        );

        self.generate_en_passant_moves(&mut moves, offset, pawn_value, position, white_piece);

        moves
    }

    fn generate_pawn_regular_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        next_line_position: i8,
        offset: i8,
        pawn_value: u8,
        position: i8,
        white_piece: bool,
    ) {
        let existing_piece = self.board_state.get_piece(next_line_position);

        if existing_piece != EMPTY_PIECE {
            return;
        }

        moves.push(PieceMove::new(position, pawn_value, next_line_position));

        if is_pawn_first_move(white_piece, position) {
            let two_lines_position = position + (offset * 2);

            let existing_piece = self.board_state.get_piece(two_lines_position);

            if existing_piece == EMPTY_PIECE {
                moves.push(PieceMove::new(position, pawn_value, two_lines_position));
            }

            // Verify if pawn is attacks king after moving. This will be user 
            // to generate the move notation. I'm too lazy to build it right now
        } else if (0..=7).contains(&next_line_position) || (56..=63).contains(&next_line_position) {
            let last_index_pos = moves.len() - 1;

            moves[last_index_pos].set_is_promotion(true);
        }
    }

    fn generate_pawn_captures(
        &self,
        moves: &mut Vec<PieceMove>,
        next_line_position: i8,
        pawn_value: u8,
        position: i8,
        white_piece: bool,
        left_diagonal: bool,
    ) {
        if (left_diagonal && (position % 8 == 0)) || !left_diagonal && ((position + 1) % 8 == 0) {
            return;
        }

        let diagonal = next_line_position - if left_diagonal { 1 } else { -1 };

        let existing_piece = self.board_state.get_piece(diagonal);

        if self.board_state.is_valid_position(diagonal)
            && existing_piece != EMPTY_PIECE
            && is_white_piece(existing_piece) != white_piece
        {
            let mut _move = PieceMove::new(position, pawn_value, diagonal);

            _move.set_is_capture(true);

            moves.push(_move);

            if !(0..=7).contains(&diagonal) && !(56..=63).contains(&diagonal) {
                return;
            }

            let last_index_pos = moves.len() - 1;

            moves[last_index_pos].set_is_promotion(true);
        }
    }

    fn generate_pawn_capturing_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        next_line_position: i8,
        pawn_value: u8,
        position: i8,
        white_piece: bool,
    ) {
        self.generate_pawn_captures(
            moves,
            next_line_position,
            pawn_value,
            position,
            white_piece,
            true,
        );

        self.generate_pawn_captures(
            moves,
            next_line_position,
            pawn_value,
            position,
            white_piece,
            false,
        );
    }

    fn generate_en_passant_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        offset: i8,
        pawn_value: u8,
        position: i8,
        white_piece: bool,
    ) {
        let en_passant = if white_piece {
            self.board_state.get_black_en_passant()
        } else {
            self.board_state.get_white_en_passant()
        };

        if en_passant == INVALID_BOARD_POSITION {
            return;
        }

        let left_square = if position % 8 != 0 {
            position - 1
        } else {
            INVALID_BOARD_POSITION
        };

        let right_square = if (position + 1) % 8 != 0 {
            position + 1
        } else {
            INVALID_BOARD_POSITION
        };

        let en_passant_target = if white_piece {
            en_passant + 8
        } else {
            en_passant - 8
        };

        let side_square;

        if left_square == en_passant_target {
            side_square = left_square;
        } else if right_square == en_passant_target {
            side_square = right_square;
        } else {
            return;
        }

        let mut _move = PieceMove::new(position, pawn_value, side_square + offset);

        _move.set_is_capture(true);
        _move.set_is_en_passant(true);

        moves.push(_move);
    }
}
// 801 lines
