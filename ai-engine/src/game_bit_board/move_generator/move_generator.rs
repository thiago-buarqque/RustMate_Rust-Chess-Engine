use std::{collections::HashMap, usize};

use crate::game_bit_board::{
    _move::Move,
    bitwise_utils::{east_one, north_one, pop_lsb, south_one, to_bitboard_position, west_one},
    board::Board,
    enums::{Color, PieceType},
    move_contants::*,
    utils::is_pawn_in_initial_position,
};

use super::{
    contants::{
        BISHOP_RELEVANT_SQUARES, BLACK_KING_SIDE_PATH_TO_ROOK, BLACK_PAWN_ATTACKS,
        BLACK_PAWN_MOVES, BLACK_QUEEN_SIDE_PATH_TO_ROOK, KING_MOVES, KNIGHT_MOVES,
        ROOK_RELEVANT_SQUARES, WHITE_KING_SIDE_PATH_TO_ROOK, WHITE_PAWN_ATTACKS, WHITE_PAWN_MOVES,
        WHITE_QUEEN_SIDE_PATH_TO_ROOK,
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

    pub fn get_moves(&self, board: &mut Board) -> Vec<Move> { self._get_moves(board, true) }

    fn _get_moves(&self, board: &mut Board, check_for_pins: bool) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        let mut friendly_king_square = usize::MAX;
        let mut opponent_king_square = usize::MAX;

        let mut friendly_attacks = 0;
        let mut opponent_attacks = 0;

        let mut opponent_piece_squares = Vec::new();

        for square in 0..64 {
            let piece_type: PieceType = board.get_piece_type(square);
            let color = board.get_piece_color(square);

            let attacks = if board.get_side_to_move() == color {
                &mut friendly_attacks
            } else {
                opponent_piece_squares.push(square);

                &mut opponent_attacks
            };

            if piece_type == PieceType::Pawn {
                MoveGenerator::get_pawn_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::Knight {
                MoveGenerator::get_knight_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::Rook {
                self.get_rook_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::Bishop {
                self.get_bishop_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::Queen {
                self.get_rook_moves(board, &mut moves, square, color, attacks);
                self.get_bishop_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::King {
                if board.get_side_to_move() == color {
                    friendly_king_square = square
                } else {
                    opponent_king_square = square
                }
            }

            // Stop early if a piece is already attacking the king?
        }

        let friendly_color = board.get_side_to_move();

        if opponent_king_square < 64 {
            MoveGenerator::get_king_moves(
                board,
                &mut moves,
                opponent_king_square,
                friendly_color.opponent(),
                &friendly_attacks,
                &mut opponent_attacks,
            );
        }

        if friendly_king_square < 64 {
            MoveGenerator::get_king_moves(
                board,
                &mut moves,
                friendly_king_square,
                friendly_color,
                &opponent_attacks,
                &mut friendly_attacks,
            );
        }

        if !check_for_pins {
            moves.retain(|_move| !opponent_piece_squares.contains(&_move.get_from()));

            return moves;
        }

        let mut moves_to_remove = Vec::new();

        // I could only check for moves from friendly pieces that are in
        // the same line, column or diagonal as the king. I.e.

        let color = board.get_side_to_move();
        let opponent_pieces = board.get_player_pieces_positions(color.opponent());

        for (i, _move) in moves.iter().enumerate() {
            if (1 << _move.get_from()) & opponent_pieces != 0 {
                moves_to_remove.push(i);
                continue;
            }

            // println!("Testing move: {_move}");

            // board.display();

            // board.move_piece(_move.clone());

            // board.display();

            let opponent_moves = self._get_moves(board, false);

            for opponent_move in opponent_moves {
                if opponent_move.get_to() == friendly_king_square {
                    println!("Removing {_move}");
                    moves_to_remove.push(i);

                    // If at least one opponent move is attacking the friendly king
                    // we can break the loop
                    break;
                }
            }

            // board.unmake_last_move();

            // println!("\nAfter unmaking:\n");

            // board.display();
        }

        let mut i = 0;
        for j in moves_to_remove {
            moves.remove(j - i);
            i += 1;
        }

        moves
    }

    fn get_bishop_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & BISHOP_RELEVANT_SQUARES[square];

        let mut attacks = *self
            .bishop_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        attacks = attacks & !friendly_pieces_bb;

        *attacked_squares |= attacks;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_rook_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & ROOK_RELEVANT_SQUARES[square];

        let mut attacks = *self
            .rook_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        attacks = attacks & !friendly_pieces_bb;

        *attacked_squares |= attacks;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_knight_moves(
        board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64,
    ) {
        // TODO: handle pins
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let mut attacks = KNIGHT_MOVES[square] & !friendly_pieces_bb; // & PINS_BB

        *attacked_squares |= attacks;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }
    }

    fn get_king_moves(
        board: &Board, moves: &mut Vec<Move>, square: usize, color: Color, opponent_attacks: &u64,
        attacked_squares: &mut u64,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let mut attacks = (KING_MOVES[square] & !friendly_pieces_bb) & !opponent_attacks;

        *attacked_squares |= attacks;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(flags, square, target_square as usize));
        }

        if board.has_queen_side_castle_right(color) {
            if ((friendly_pieces_bb & BLACK_QUEEN_SIDE_PATH_TO_ROOK) == 0 && color.is_black())
                || ((friendly_pieces_bb & WHITE_QUEEN_SIDE_PATH_TO_ROOK) == 0 && color.is_white())
            {
                let _west_one = west_one(1 << square);
                let mut west_two = west_one(_west_one);

                if _west_one & opponent_attacks == 0 && west_two & opponent_attacks == 0 {
                    let _move =
                        Move::with_flags(QUEEN_CASTLE, square, pop_lsb(&mut west_two) as usize);

                    moves.push(_move);
                }
            }
        }

        if board.has_king_side_castle_right(color) {
            if ((friendly_pieces_bb & BLACK_KING_SIDE_PATH_TO_ROOK) == 0 && color.is_black())
                || ((friendly_pieces_bb & WHITE_KING_SIDE_PATH_TO_ROOK) == 0 && color.is_white())
            {
                let _east_one = east_one(1 << square);
                let mut east_two = east_one(_east_one);

                if _east_one & opponent_attacks == 0 && east_two & opponent_attacks == 0 {
                    let _move =
                        Move::with_flags(KING_CASTLE, square, pop_lsb(&mut east_two) as usize);

                    moves.push(_move);
                }
            }
        }
    }

    fn get_pawn_moves(
        board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64,
    ) {
        // TODO: handle en passant, promotions and pins
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_squares = friendly_pieces_bb | opponent_pieces_bb;

        let mut attacks = (MoveGenerator::look_up_pawn_attacks(color, square)
            & !friendly_pieces_bb)
            & opponent_pieces_bb;

        *attacked_squares |= attacks;

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

            *attacked_squares |= forward_two;

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

            *attacked_squares |= attacks;

            while attacks != 0 {
                let target_square = pop_lsb(&mut attacks);

                moves.push(Move::with_flags(EN_PASSANT, square, target_square as usize));
            }
        }

        *attacked_squares |= forward_one;

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
        move_contants::{CAPTURE, KING_CASTLE, QUEEN_CASTLE},
        positions::{BBPositions, Squares},
    };

    use super::{MoveGenerator, DOUBLE_PAWN_PUSH, EN_PASSANT};

    static MOVE_GENERATOR: Lazy<MoveGenerator> = Lazy::new(|| MoveGenerator::new());

    fn assert_available_moves(
        board: &mut Board, expected_moves: Vec<Move>, not_expected_moves: Vec<Move>,
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

    fn setup_castle_test(queen_castle: Move, king_castle: Move) -> Board {
        let mut board = Board::new();

        let mut not_expected_moves = Vec::new();
        not_expected_moves.push(queen_castle.clone());
        not_expected_moves.push(king_castle.clone());

        assert_available_moves(&mut board, Vec::new(), not_expected_moves);

        board
    }

    #[test]
    fn test_castle() {
        let (
            mut board,
            white_king_side_castle,
            white_queen_side_castle,
            black_king_side_castle,
            black_queen_side_castle,
        ) = set_up_castle_position();

        // White: Make sure castle rights are lost after castle is performed

        board.move_piece(white_king_side_castle.clone());

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(white_queen_side_castle.clone());

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(Move::from_to(Squares::E1, Squares::E2));

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        // board.unmake_last_move();

        // Black: Make sure castle rights are lost after castle is performed

        board.move_piece(black_king_side_castle.clone());

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(black_queen_side_castle.clone());

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(Move::from_to(Squares::E8, Squares::E7));

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        // board.unmake_last_move();
    }

    fn set_up_castle_position() -> (Board, Move, Move, Move, Move) {
        let mut board = Board::new();

        // Assert both sides have all castling rights

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        /*
            Perform a bunch of moves to clear the paths to test castling
            After all these moves the board will be

            8 ♜ . . . ♚ . . ♜
            7 ♟ ♟ ♟ ♛ . ♟ ♟ ♟
            6 ♗ . ♞ . . ♞ . ♗
            5 . . . ♟ ♟ . . .
            4 . . . ♙ ♙ . . .
            3 ♝ . ♘ . . ♘ . ♝
            2 ♙ ♙ ♙ ♕ . ♙ ♙ ♙
            1 ♖ . . . ♔ . . ♖
              a b c d e f g h
        */

        board.move_piece(Move::from_to(Squares::D2, Squares::D4));
        board.move_piece(Move::from_to(Squares::D7, Squares::D5));
        board.move_piece(Move::from_to(Squares::E2, Squares::E4));
        board.move_piece(Move::from_to(Squares::E7, Squares::E5));
        board.move_piece(Move::from_to(Squares::C1, Squares::H6));
        board.move_piece(Move::from_to(Squares::C8, Squares::H3));
        board.move_piece(Move::from_to(Squares::F1, Squares::A6));
        board.move_piece(Move::from_to(Squares::F8, Squares::A3));
        board.move_piece(Move::from_to(Squares::D1, Squares::D2));
        board.move_piece(Move::from_to(Squares::D8, Squares::D7));
        board.move_piece(Move::from_to(Squares::B1, Squares::C3));
        board.move_piece(Move::from_to(Squares::B8, Squares::C6));
        board.move_piece(Move::from_to(Squares::G1, Squares::F3));
        board.move_piece(Move::from_to(Squares::G8, Squares::F6));

        board.display();

        let white_king_side_castle = Move::with_flags(KING_CASTLE, Squares::E1, Squares::G1);
        let white_queen_side_castle = Move::with_flags(QUEEN_CASTLE, Squares::E1, Squares::C1);

        let black_king_side_castle = Move::with_flags(KING_CASTLE, Squares::E8, Squares::G8);
        let black_queen_side_castle = Move::with_flags(QUEEN_CASTLE, Squares::E8, Squares::C8);
        (
            board,
            white_king_side_castle,
            white_queen_side_castle,
            black_king_side_castle,
            black_queen_side_castle,
        )
    }

    #[test]
    fn test_castle_blocking() {
        let (
            mut board,
            white_king_side_castle,
            white_queen_side_castle,
            black_king_side_castle,
            black_queen_side_castle,
        ) = set_up_castle_position();

        // Assert white have both castle moves available

        let mut white_king_castle_moves = Vec::new();

        white_king_castle_moves.push(white_king_side_castle);
        white_king_castle_moves.push(white_queen_side_castle);

        assert_available_moves(&mut board, white_king_castle_moves.clone(), Vec::new());

        // Make a move to give turn to Black
        board.move_piece(Move::from_to(Squares::B2, Squares::B4));

        // Assert black have both castle moves available

        let mut black_king_castle_moves = Vec::new();

        black_king_castle_moves.push(black_king_side_castle);
        black_king_castle_moves.push(black_queen_side_castle);

        assert_available_moves(&mut board, black_king_castle_moves.clone(), Vec::new());

        board.move_piece(Move::from_to(Squares::B7, Squares::B5));
        board.move_piece(Move::from_to(Squares::G2, Squares::G4));

        assert_available_moves(&mut board, Vec::new(), white_king_castle_moves);

        board.move_piece(Move::from_to(Squares::G7, Squares::G5));

        board.display();

        assert_available_moves(&mut board, Vec::new(), black_king_castle_moves);

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));
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

        assert_available_moves(&mut board, Vec::new(), not_expected_moves);

        board = Board::empty();

        board.place_piece(Color::White, PieceType::King, BBPositions::E1);
        board.place_piece(Color::Black, PieceType::Rook, BBPositions::F2);
        board.place_piece(Color::White, PieceType::Rook, BBPositions::F1);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(Squares::E1, Squares::D1));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::E1, Squares::F2));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::from_to(Squares::E1, Squares::F1));

        board.display();

        assert_available_moves(&mut board, expected_moves, not_expected_moves);
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

        assert_available_moves(&mut board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_bishop_moves() {
        let mut board = Board::empty();

        board.place_piece(Color::White, PieceType::Bishop, BBPositions::A1);

        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::C3);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(Squares::A1, Squares::B2));
        expected_moves.push(Move::with_flags(CAPTURE, Squares::A1, Squares::C3));

        assert_available_moves(&mut board, expected_moves, Vec::new());
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

        assert_available_moves(&mut board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_knight_moves() {
        let mut board = Board::new();

        let mut expected_moves = Vec::new();

        let white_knight_to_c3 = Move::from_to(Squares::B1, Squares::C3);

        expected_moves.push(white_knight_to_c3.clone());
        expected_moves.push(Move::from_to(Squares::B1, Squares::A3));

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(white_knight_to_c3);
        board.move_piece(Move::from_to(Squares::D7, Squares::D5));

        let white_knight_to_d5 = Move::with_flags(CAPTURE, Squares::C3, Squares::D5);

        expected_moves = Vec::new();

        expected_moves.push(white_knight_to_d5.clone());

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_pawn_moves() {
        let mut board = Board::new();

        let mut expected_moves = Vec::new();

        let mut white_pawn_to_d4 = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::D2, Squares::D4);

        white_pawn_to_d4.set_en_passant_bb_position(BBPositions::D3);
        white_pawn_to_d4.set_en_passant_bb_piece_square(BBPositions::D4);

        expected_moves.push(white_pawn_to_d4.clone());

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(white_pawn_to_d4);

        expected_moves = Vec::new();

        let mut black_pawn_to_e5 = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::E7, Squares::E5);

        black_pawn_to_e5.set_en_passant_bb_position(BBPositions::E6);
        black_pawn_to_e5.set_en_passant_bb_piece_square(BBPositions::E5);

        expected_moves.push(black_pawn_to_e5.clone());

        board.move_piece(black_pawn_to_e5);

        expected_moves = Vec::new();

        expected_moves.push(Move::with_flags(CAPTURE, Squares::D4, Squares::E5));

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_en_passant() {
        let mut board = Board::new();

        board.move_piece(Move::with_flags(DOUBLE_PAWN_PUSH, Squares::D2, Squares::D4));
        board.move_piece(Move::from_to(Squares::A7, Squares::A6));
        board.move_piece(Move::from_to(Squares::D4, Squares::D5));
        board.move_piece(Move::from_to(Squares::A6, Squares::A5));
        board.move_piece(Move::with_flags(DOUBLE_PAWN_PUSH, Squares::F2, Squares::F4));
        board.move_piece(Move::from_to(Squares::A5, Squares::A4));
        board.move_piece(Move::from_to(Squares::F4, Squares::F5));

        /*
            8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
            7 . ♟ ♟ ♟ ♟ ♟ ♟ ♟
            6 . . . . . . . .
            5 . . . ♙ . ♙ . .
            4 ♟ . . . . . . .
            3 . . . . . . . .
            2 ♙ ♙ ♙ . ♙ . ♙ ♙
            1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
              a b c d e f g h
        */
        board.display();

        let mut expected_moves = Vec::new();

        let mut black_double_push = Move::with_flags(DOUBLE_PAWN_PUSH, Squares::E7, Squares::E5);

        black_double_push.set_en_passant_bb_position(BBPositions::E6);
        black_double_push.set_en_passant_bb_piece_square(BBPositions::E5);

        expected_moves.push(black_double_push.clone());

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(black_double_push);

        expected_moves = Vec::new();

        // Assert that two en passant moves are available when the opponent pawn pushes
        // two squares between two friendly pawns.

        expected_moves.push(Move::with_flags(EN_PASSANT, Squares::D5, Squares::E6));
        expected_moves.push(Move::with_flags(EN_PASSANT, Squares::F5, Squares::E6));

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }
}
