use std::collections::HashMap;

use crate::game_bit_board::{
    _move::Move,
    bitwise_utils::{north_one, pop_lsb, south_one, to_bitboard_position},
    board::Board,
    enums::{Color, PieceType},
    move_contants::*,
    utils::is_pawn_in_initial_position,
};

use super::{
    contants::{
        BISHOP_RELEVANT_SQUARES, BLACK_PAWN_ATTACKS, BLACK_PAWN_MOVES, KING_MOVES, KNIGHT_MOVES,
        ROOK_RELEVANT_SQUARES, WHITE_PAWN_ATTACKS, WHITE_PAWN_MOVES,
    },
    raw_move_generator::RawMoveGenerator,
};

pub struct MoveGenerator {
    bishop_lookup_table: HashMap<(u8, u64), u64>,
    rook_lookup_table: HashMap<(u8, u64), u64>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        let bishop_lookup_table: HashMap<(u8, u64), u64> =
            RawMoveGenerator::create_bishop_lookup_table();
        let rook_lookup_table: HashMap<(u8, u64), u64> =
            RawMoveGenerator::create_rook_lookup_table();

        Self {
            bishop_lookup_table,
            rook_lookup_table,
        }
    }

    pub fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        for square in 0..64 {
            let piece_type: PieceType = board.get_piece_type(square);
            let color = board.get_piece_color(square);

            if piece_type == PieceType::Pawn {
                MoveGenerator::get_pawn_moves(board, &mut moves, square, color);
            } else if piece_type == PieceType::Knight {
                MoveGenerator::get_knight_moves(board, &mut moves, square, color);
            } else if piece_type == PieceType::Rook {
                self.get_rook_moves(board, &mut moves, square, color);
            } else if piece_type == PieceType::Bishop {
                self.get_bishop_moves(board, &mut moves, square, color);
            } else if piece_type == PieceType::Queen {
                self.get_rook_moves(board, &mut moves, square, color);
                self.get_bishop_moves(board, &mut moves, square, color);
            } else if piece_type == PieceType::King {
                MoveGenerator::get_king_moves(board, &mut moves, square, color);
            }
        }

        moves
    }

    fn get_bishop_moves(&self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & BISHOP_RELEVANT_SQUARES[square];

        let mut attacks = *self
            .bishop_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        attacks = attacks & !friendly_pieces_bb;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_rook_moves(&self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & ROOK_RELEVANT_SQUARES[square];

        let mut attacks = *self
            .rook_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        attacks = attacks & !friendly_pieces_bb;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_knight_moves(board: &Board, moves: &mut Vec<Move>, square: usize, color: Color) {
        // TODO: handle pins
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let mut attacks = KNIGHT_MOVES[square] & !friendly_pieces_bb; // & PINS_BB

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_king_moves(board: &Board, moves: &mut Vec<Move>, square: usize, color: Color) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let mut attacks = KING_MOVES[square] & !friendly_pieces_bb;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_pawn_moves(board: &Board, moves: &mut Vec<Move>, square: usize, color: Color) {
        // TODO: handle en passant, promotions and pins
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_squares = friendly_pieces_bb | opponent_pieces_bb;

        let mut attacks = (MoveGenerator::look_up_pawn_attacks(color, square)
            & !friendly_pieces_bb)
            & opponent_pieces_bb;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            moves.push(Move::with_flags(CAPTURE, square, target_square as usize));
        }

        let bb_position = to_bitboard_position(square as u64);

        let offset_fn = if color.is_white() {
            north_one
        } else {
            south_one
        };

        let mut forward_one = offset_fn(bb_position) & !occupied_squares;

        if is_pawn_in_initial_position(bb_position, color.is_white()) && forward_one != 0 {
            let mut forward_two = offset_fn(forward_one);

            if forward_two & !occupied_squares != 0 {
                let en_passant_bb_piece_square = forward_two;

                let target_square = pop_lsb(&mut forward_two) as usize;

                let mut _move = Move::with_flags(DOUBLE_PAWN_PUSH, square, target_square);

                _move.set_en_passant_bb_position(forward_one);
                _move.set_en_passant_bb_piece_square(en_passant_bb_piece_square);

                moves.push(_move);
            }
        } else if board.get_en_passant() != 0 {
            let mut attacks = (MoveGenerator::look_up_pawn_attacks(color, square)
                & !friendly_pieces_bb)
                & board.get_en_passant();

            while attacks != 0 {
                let target_square = pop_lsb(&mut attacks);

                moves.push(Move::with_flags(EN_PASSANT, square, target_square as usize));
            }
        }

        if forward_one != 0 {
            moves.push(Move::from_to(square, pop_lsb(&mut forward_one) as usize));
        }
    }

    fn look_up_pawn_attacks(color: Color, square: usize) -> u64 {
        if color.is_white() {
            WHITE_PAWN_ATTACKS[square]
        } else {
            BLACK_PAWN_ATTACKS[square]
        }
    }

    fn look_up_pawn_moves(color: Color, square: usize) -> u64 {
        if color.is_white() {
            WHITE_PAWN_MOVES[square]
        } else {
            BLACK_PAWN_MOVES[square]
        }
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::game_bit_board::{
        _move::Move,
        board::Board,
        enums::{Color, PieceType},
        move_contants::CAPTURE,
        positions::{BBPositions, Squares},
    };

    use super::{MoveGenerator, DOUBLE_PAWN_PUSH, EN_PASSANT};

    static MOVE_GENERATOR: Lazy<MoveGenerator> = Lazy::new(|| MoveGenerator::new());

    fn assert_available_moves(
        board: &Board, expected_moves: Vec<Move>, not_expected_moves: Vec<Move>,
    ) {
        let moves = MOVE_GENERATOR.get_moves(board);

        expected_moves.iter().for_each(|expected_move| {
            assert!(
                moves.iter().any(|_move| *_move == *expected_move),
                "Move {expected_move} should exist"
            );
        });

        not_expected_moves.iter().for_each(|not_expected_move| {
            assert_eq!(
                false,
                moves.iter().any(|_move| *_move == *not_expected_move),
                "Move {not_expected_move} should not exist"
            );
        });
    }

    #[test]
    fn test_get_king_moves() {
        let mut board = Board::new();

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::D1));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::D2));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::E2));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::F2));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::F1));

        assert_available_moves(&board, Vec::new(), not_expected_moves);

        board = Board::empty();

        board.place_piece(Color::White, PieceType::King, BBPositions::E1);
        board.place_piece(Color::Black, PieceType::Rook, BBPositions::F2);
        board.place_piece(Color::White, PieceType::Rook, BBPositions::F1);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(Squares::E1, Squares::D1));
        expected_moves.push(Move::from_to(Squares::E1, Squares::D2));
        expected_moves.push(Move::from_to(Squares::E1, Squares::E2));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::F2));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::from_to(Squares::E1, Squares::F1));

        assert_available_moves(&board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_queen_moves() {
        let mut board = Board::empty();

        board.place_piece(Color::White, PieceType::Queen, BBPositions::D1);
        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::D4);
        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::F3);
        board.place_piece(Color::White, PieceType::Pawn, BBPositions::C2);
        board.place_piece(Color::White, PieceType::Bishop, BBPositions::C1);
        board.place_piece(Color::White, PieceType::King, BBPositions::E1);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(Squares::D1, Squares::D2));
        expected_moves.push(Move::from_to(Squares::D1, Squares::D3));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::D1, Squares::D4));

        expected_moves.push(Move::from_to(Squares::D1, Squares::E2));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::D1, Squares::F3));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::D1, Squares::C2));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::D1, Squares::C2));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::D1, Squares::C1));
        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::D1, Squares::E1));

        assert_available_moves(&board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_bishop_moves() {
        let mut board = Board::empty();

        board.place_piece(Color::White, PieceType::Bishop, BBPositions::A1);

        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::C3);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(Squares::A1, Squares::B2));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::A1, Squares::C3));

        assert_available_moves(&board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_rook_moves() {
        let mut board = Board::empty();

        board.place_piece(Color::White, PieceType::Rook, BBPositions::A1);
        board.place_piece(Color::White, PieceType::Bishop, BBPositions::C1);
        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::A3);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(Squares::A1, Squares::A2));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::A1, Squares::A3));
        expected_moves.push(Move::from_to(Squares::A1, Squares::B1));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::with_flags(CAPTURE, Squares::A1, Squares::C1));

        assert_available_moves(&board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_knight_moves() {
        let mut board = Board::new();

        let mut expected_moves = Vec::new();

        let white_knight_to_c3 = Move::from_to(Squares::B1, Squares::C3);

        expected_moves.push(white_knight_to_c3.clone());
        expected_moves.push(Move::from_to(Squares::B1, Squares::A3));

        assert_available_moves(&board, expected_moves, Vec::new());

        board.move_piece(white_knight_to_c3);
        board.move_piece(Move::from_to(Squares::D7, Squares::D5));

        let white_knight_to_d5 = Move::with_flags(CAPTURE, Squares::C3, Squares::D5);

        expected_moves = Vec::new();

        expected_moves.push(white_knight_to_d5.clone());

        assert_available_moves(&board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_pawn_moves() {
        let mut board = Board::new();

        let mut expected_moves = Vec::new();

        let mut white_pawn_to_d4 = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::D2, Squares::D4);

        white_pawn_to_d4.set_en_passant_bb_position(BBPositions::D3);
        white_pawn_to_d4.set_en_passant_bb_piece_square(BBPositions::D4);

        let mut black_pawn_to_e5 = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::E7, Squares::E5);

        black_pawn_to_e5.set_en_passant_bb_position(BBPositions::E6);
        black_pawn_to_e5.set_en_passant_bb_piece_square(BBPositions::E5);

        expected_moves.push(white_pawn_to_d4.clone());
        expected_moves.push(black_pawn_to_e5.clone());

        assert_available_moves(&board, expected_moves, Vec::new());

        board.move_piece(white_pawn_to_d4);
        board.move_piece(black_pawn_to_e5);

        expected_moves = Vec::new();

        let capture1 = Move::with_flags(CAPTURE, 27, 36);

        expected_moves.push(capture1);

        let capture2 = Move::with_flags(CAPTURE, 36, 27);

        expected_moves.push(capture2);

        assert_available_moves(&board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_en_passant() {
        let mut board = Board::new();

        board.move_piece(Move::with_flags(DOUBLE_PAWN_PUSH, Squares::D2, Squares::D4));
        board.move_piece(Move::from_to(Squares::D4, Squares::D5));
        board.move_piece(Move::with_flags(DOUBLE_PAWN_PUSH, Squares::F2, Squares::F4));
        board.move_piece(Move::from_to(Squares::F4, Squares::F5));

        let mut expected_moves = Vec::new();

        let mut black_double_push = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::E7, Squares::E5);

        black_double_push.set_en_passant_bb_position(BBPositions::E6);
        black_double_push.set_en_passant_bb_piece_square(BBPositions::E5);

        expected_moves.push(black_double_push.clone());

        assert_available_moves(&board, expected_moves, Vec::new());

        board.move_piece(black_double_push);

        expected_moves = Vec::new();

        // Assert that two en passant moves are available when the opponent pawn pushes
        // two squares between two friendly pawns.

        expected_moves.push(Move::with_flags(EN_PASSANT, Squares::D5, Squares::E6));
        expected_moves.push(Move::with_flags(EN_PASSANT, Squares::F5, Squares::E6));

        assert_available_moves(&board, expected_moves, Vec::new());
    }
}
