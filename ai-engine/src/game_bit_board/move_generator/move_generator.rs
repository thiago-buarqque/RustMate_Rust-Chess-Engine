use super::{
    attack_data::AttackData,
    contants::{
        BISHOP_RELEVANT_SQUARES, BLACK_KING_SIDE_PATH_TO_ROOK, BLACK_PAWN_ATTACKS,
        BLACK_PAWN_MOVES, BLACK_QUEEN_SIDE_PATH_TO_ROOK, KING_MOVES, KNIGHT_MOVES,
        ROOK_RELEVANT_SQUARES, WHITE_KING_SIDE_PATH_TO_ROOK, WHITE_PAWN_ATTACKS, WHITE_PAWN_MOVES,
        WHITE_QUEEN_SIDE_PATH_TO_ROOK,
    },
    raw_move_generator::RawMoveGenerator,
    utils::{create_moves, print_board},
};
use crate::game_bit_board::{
    _move::{_move::Move, move_contants::*},
    board::Board,
    enums::{Color, PieceType},
    positions::{same_rank, BBPositions, Squares},
    utils::{
        bitwise_utils::{east_one, north_one, pop_lsb, south_one, to_bitboard_position, west_one},
        utils::is_pawn_in_initial_position,
    },
};
use std::{collections::HashMap, u64, usize};
#[derive(Clone)]
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

    pub fn get_moves(&mut self, board: &mut Board) -> Vec<Move> {
        if board.is_game_finished() {
            panic!("Can't generate moves. Game has already ended.");
        }

        // Calculate attack data
        // self.calculate_attack_data(board);
        let mut attack_data = AttackData::new();

        attack_data.calculate_attack_data(board, self);

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
                self.get_pawn_moves(board, &mut moves, square, color, attacks, &attack_data);
            } else if piece_type == PieceType::Knight {
                self.get_knight_moves(board, &mut moves, square, color, attacks, &attack_data);
            } else if piece_type == PieceType::Rook {
                self.get_orthogonal_moves(
                    board,
                    &mut moves,
                    square,
                    color,
                    attacks,
                    PieceType::Rook,
                    &attack_data,
                );
            } else if piece_type == PieceType::Bishop {
                self.get_diagonal_moves(
                    board,
                    &mut moves,
                    square,
                    color,
                    attacks,
                    PieceType::Bishop,
                    &attack_data,
                );
            } else if piece_type == PieceType::Queen {
                self.get_orthogonal_moves(
                    board,
                    &mut moves,
                    square,
                    color,
                    attacks,
                    PieceType::Queen,
                    &attack_data,
                );
                self.get_diagonal_moves(
                    board,
                    &mut moves,
                    square,
                    color,
                    attacks,
                    PieceType::Queen,
                    &attack_data,
                );
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
            self.get_king_moves(
                board,
                &mut moves,
                opponent_king_square,
                friendly_color.opponent(),
                &friendly_attacks,
                &mut opponent_attacks,
                &attack_data,
            );
        }

        if friendly_king_square < 64 {
            self.get_king_moves(
                board,
                &mut moves,
                friendly_king_square,
                friendly_color,
                &opponent_attacks,
                &mut friendly_attacks,
                &attack_data,
            );
        }

        moves.retain(|_move| !opponent_piece_squares.contains(&_move.get_from()));

        if attack_data.in_double_check {
            moves.retain(|_move| _move.get_from() != friendly_king_square);
        }

        let side_to_move = board.get_side_to_move();

        if moves.is_empty() {
            // Game ended
            board.set_winner(Some(side_to_move.opponent()));
        }

        moves
    }

    fn get_diagonal_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64, piece_type: PieceType, attack_data: &AttackData,
    ) {
        // if square == Squares::F3 {
        //     println!("DEBUG::::");
        //     print_board(color, square as u64, piece_type, self.get_diagonal_attacks(board, color, square));
        //     print_board(color, square as u64, piece_type, attack_data.friendly_pins_moves_bbs[square]);
        //     print_board(color, square as u64, piece_type, attack_data.defenders_bb | attack_data.attack_bb);
        // }

        let friendly_pieces_bb = board.get_player_pieces_positions(color);

        let attacks = self.get_diagonal_attacks(board, color, square, &friendly_pieces_bb)
            & !friendly_pieces_bb
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        *attacked_squares |= attacks;

        create_moves(
            attacks,
            board.get_player_pieces_positions(color.opponent()),
            moves,
            square,
            color,
            piece_type,
        );
    }

    pub fn get_diagonal_attacks(&self, board: &Board, color: Color, square: usize, friendly_pieces_bb: &u64) -> u64 {
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & BISHOP_RELEVANT_SQUARES[square];

        let attacks = *self
            .bishop_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        attacks
    }

    fn get_orthogonal_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64, piece_type: PieceType, attack_data: &AttackData,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);

        let attacks = self.get_orthogonal_attacks(board, color, square, &friendly_pieces_bb)
            & !friendly_pieces_bb
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        *attacked_squares |= attacks;

        create_moves(
            attacks,
            board.get_player_pieces_positions(color.opponent()),
            moves,
            square,
            color,
            piece_type,
        );
    }

    pub fn get_orthogonal_attacks(&self, board: &Board, color: Color, square: usize, friendly_pieces_bb: &u64) -> u64 {
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & ROOK_RELEVANT_SQUARES[square];

        let attacks = *self
            .rook_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        attacks
    }

    fn get_knight_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64, attack_data: &AttackData,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let raw_attacks = KNIGHT_MOVES[square];

        *attacked_squares |= raw_attacks;

        let attacks = (raw_attacks & !friendly_pieces_bb)
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        create_moves(
            attacks,
            opponent_pieces_bb,
            moves,
            square,
            color,
            PieceType::Knight,
        );
    }

    fn get_king_moves(
        &mut self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        opponent_attacks: &u64, attacked_squares: &mut u64, attack_data: &AttackData,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_squares = friendly_pieces_bb | opponent_pieces_bb;

        // I guess I can just use & self.friendly_pins_moves_bbs[square] instead of
        // having a king allowed squares var
        let attacks = ((KING_MOVES[square] & !friendly_pieces_bb) & !opponent_attacks)
            & attack_data.king_allowed_squares;

        *attacked_squares |= attacks;

        create_moves(
            attacks,
            opponent_pieces_bb,
            moves,
            square,
            color,
            PieceType::King,
        );

        if attack_data.in_check || (to_bitboard_position(square as u64) & opponent_attacks != 0 && attacks == 0) {
            return;
        }

        if board.has_queen_side_castle_right(color) {
            if ((occupied_squares & BLACK_QUEEN_SIDE_PATH_TO_ROOK) == 0 && color.is_black())
                || ((occupied_squares & WHITE_QUEEN_SIDE_PATH_TO_ROOK) == 0 && color.is_white())
            {
                let _west_one = west_one(1 << square);
                let mut west_two = west_one(_west_one);

                if _west_one & opponent_attacks == 0 && west_two & opponent_attacks == 0 {
                    let _move = Move::with_flags(
                        QUEEN_CASTLE,
                        square,
                        pop_lsb(&mut west_two) as usize,
                        color,
                        PieceType::King,
                    );

                    moves.push(_move);
                }
            }
        }

        if board.has_king_side_castle_right(color) {
            if ((occupied_squares & BLACK_KING_SIDE_PATH_TO_ROOK) == 0 && color.is_black())
                || ((occupied_squares & WHITE_KING_SIDE_PATH_TO_ROOK) == 0 && color.is_white())
            {
                let _east_one = east_one(1 << square);
                let mut east_two = east_one(_east_one);

                if _east_one & opponent_attacks == 0 && east_two & opponent_attacks == 0 {
                    let _move = Move::with_flags(
                        KING_CASTLE,
                        square,
                        pop_lsb(&mut east_two) as usize,
                        color,
                        PieceType::King,
                    );

                    moves.push(_move);
                }
            }
        }
    }

    fn is_promotion_square(color: Color, square: usize) -> bool {
        if square >= Squares::A8 && square <= Squares::H8 && color.is_white() {
            return true;
        }

        square >= Squares::A1 && square <= Squares::H1 && color.is_black()
    }

    fn add_promotion_moves(
        color: Color, capture: bool, from: usize, moves: &mut Vec<Move>, target_square: usize,
    ) {
        let flags = if capture {
            vec![
                KNIGHT_PROMOTION_CAPTURE,
                BISHOP_PROMOTION_CAPTURE,
                ROOK_PROMOTION_CAPTURE,
                QUEEN_PROMOTION_CAPTURE,
            ]
        } else {
            vec![
                KNIGHT_PROMOTION,
                BISHOP_PROMOTION,
                ROOK_PROMOTION,
                QUEEN_PROMOTION,
            ]
        };

        for flag in flags {
            moves.push(Move::with_flags(
                flag,
                from,
                target_square as usize,
                color,
                PieceType::Pawn,
            ));
        }
    }

    fn get_pawn_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64, attack_data: &AttackData,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_squares = friendly_pieces_bb | opponent_pieces_bb;

        let raw_attacks = (MoveGenerator::look_up_pawn_attacks(color, square)
            & !friendly_pieces_bb)
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        *attacked_squares |= raw_attacks;

        let mut attacks = raw_attacks & opponent_pieces_bb;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            if MoveGenerator::is_promotion_square(color, target_square as usize) {
                MoveGenerator::add_promotion_moves(
                    color,
                    true,
                    square,
                    moves,
                    target_square as usize,
                );
            } else {
                moves.push(Move::with_flags(
                    CAPTURE,
                    square,
                    target_square as usize,
                    color,
                    PieceType::Pawn,
                ));
            }
        }

        let bb_position = to_bitboard_position(square as u64);

        let offset_fn = if color.is_white() {
            north_one
        } else {
            south_one
        };

        let raw_forward_one = offset_fn(bb_position) & !occupied_squares;
        let mut forward_one = raw_forward_one
            & attack_data.friendly_pins_moves_bbs[square]
            & attack_data.defenders_bb;

        if is_pawn_in_initial_position(bb_position, color.is_white()) {
            let mut forward_two = (offset_fn(raw_forward_one) & !occupied_squares)
                & attack_data.friendly_pins_moves_bbs[square]
                & attack_data.defenders_bb;

            // *attacked_squares |= forward_two;

            if forward_two != 0 {
                let en_passant_bb_piece_square = forward_two;

                let target_square = pop_lsb(&mut forward_two) as usize;

                let mut _move = Move::with_flags(
                    DOUBLE_PAWN_PUSH,
                    square,
                    target_square,
                    color,
                    PieceType::Pawn,
                );

                _move.set_en_passant_bb_position(raw_forward_one);
                _move.set_en_passant_bb_piece_square(en_passant_bb_piece_square);

                moves.push(_move);
            }
        } else if board.get_en_passant() != 0 {
            let mut attacks = (MoveGenerator::look_up_pawn_attacks(color, square)
                & !friendly_pieces_bb)
                & board.get_en_passant()
                & attack_data.friendly_pins_moves_bbs[square]
                & (attack_data.defenders_bb | attack_data.attack_bb);

            // A pawn will never be able to have more than one
            // en passant move at the same time
            while attacks != 0
                && !is_en_passant_discovered_check(color, attack_data, square, board) {

                let target_square = pop_lsb(&mut attacks);

                moves.push(Move::with_flags(
                    EN_PASSANT,
                    square,
                    target_square as usize,
                    color,
                    PieceType::Pawn,
                ));
            }
        }

        // *attacked_squares |= forward_one;

        if forward_one != 0 {
            let target_square = pop_lsb(&mut forward_one);

            if MoveGenerator::is_promotion_square(color, target_square as usize) {
                MoveGenerator::add_promotion_moves(
                    color,
                    false,
                    square,
                    moves,
                    target_square as usize,
                );
            } else {
                moves.push(Move::from_to(
                    square,
                    target_square as usize,
                    color,
                    PieceType::Pawn,
                ));
            }
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

fn is_en_passant_discovered_check(color: Color, attack_data: &AttackData, square: usize, board: &Board) -> bool {
    if color != attack_data.side_to_move || !same_rank(square, attack_data.king_square) {
        return false;
    }

    let row = BBPositions::get_row_bb(attack_data.king_bb_position);

    let opponent = attack_data.side_to_move.opponent();

    let opponent_queens = board.get_piece_positions(
        opponent, PieceType::Queen)
        & row;

    let opponent_rooks = board.get_piece_positions(
        opponent, PieceType::Rook)
        & row;

    if opponent_rooks == 0 && opponent_queens == 0 {
        return false;
    }

    let friendly_pieces = board.get_player_pieces_positions(attack_data.side_to_move);
    let opponent_pieces = board.get_player_pieces_positions(opponent);
    let mut row_occupied_squares = (friendly_pieces | opponent_pieces) & row;

    // Remove friendly and enemy pawns involved in en passant
    row_occupied_squares &= !board.get_en_passant_square();
    row_occupied_squares &= !to_bitboard_position(square as u64);

    let mut squares_between_rook_and_king = 0;
    if opponent_rooks != 0 {
        let closest_rook_square = get_closest_square(attack_data.king_square, opponent_rooks);

        squares_between_rook_and_king = squares_between(
            attack_data.king_square, closest_rook_square);
    }

    let mut squares_between_queen_and_king = 0;
    if opponent_queens != 0 {
        let closest_queen_square = get_closest_square(attack_data.king_square, opponent_queens);

        if closest_queen_square != 0 {
            squares_between_queen_and_king = squares_between(
                attack_data.king_square, closest_queen_square);
        }

    }

    let squares_between_king = squares_between_rook_and_king | squares_between_queen_and_king;

    // After removing all those pieces, if the row is empty, it means there aren't
    // other pieces (either friendly or not) that would protect the king
    if squares_between_king & row_occupied_squares == 0 {
        return true;
    }

    false
}

fn squares_between(sq1: usize, sq2: usize) -> u64 {
    if sq1 < sq2 {
        ((sq1 + 1)..sq2).fold(0, |acc, sq| acc | (1 << sq))
    } else {
        ((sq2 + 1)..sq1).fold(0, |acc, sq| acc | (1 << sq))
    }
}

fn get_closest_square(base_square: usize, pieces: u64) -> usize {
    let mut closest_square = usize::MAX;

    let mut pieces = pieces.clone();

    while pieces != 0 {
        let square = pop_lsb(&mut pieces) as usize;

        if base_square.abs_diff(square) < closest_square {
            closest_square = square;
        }
    }

    closest_square
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::game_bit_board::{
        _move::{_move::Move, move_contants::*},
        board::Board,
        enums::{Color, PieceType},
        positions::{BBPositions, Squares},
    };

    use super::{MoveGenerator, DOUBLE_PAWN_PUSH, EN_PASSANT};

    static MOVE_GENERATOR: Lazy<Mutex<MoveGenerator>> =
        Lazy::new(|| Mutex::new(MoveGenerator::new()));

    fn assert_available_moves(
        board: &mut Board, expected_moves: Vec<Move>, not_expected_moves: Vec<Move>,
    ) {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let moves = move_generator.get_moves(board);

        expected_moves.iter().for_each(|expected_move| {
            assert!(
                moves.iter().any(|_move| *_move == *expected_move),
                "Move {expected_move} should exist. Available moves: {:#?}",
                moves
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
        not_expected_moves.push(queen_castle);
        not_expected_moves.push(king_castle);

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

        board.move_piece(&white_king_side_castle);

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        assert_eq!(PieceType::King, board.get_piece_type(Squares::G1));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::F1));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(&white_queen_side_castle);

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        assert_eq!(PieceType::King, board.get_piece_type(Squares::C1));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::D1));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(&Move::dummy_from_to(Squares::E1, Squares::E2));

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(&Move::dummy_from_to(Squares::A1, Squares::B1));

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        board.unmake_last_move();

        board.move_piece(&Move::dummy_from_to(Squares::H1, Squares::G1));

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        // Black: Make sure castle rights are lost after castle is performed

        board.move_piece(&black_king_side_castle);

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        assert_eq!(PieceType::King, board.get_piece_type(Squares::G8));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::F8));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(&black_queen_side_castle);

        assert_eq!(PieceType::King, board.get_piece_type(Squares::C8));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::D8));

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(&Move::dummy_from_to(Squares::E8, Squares::E7));

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(&Move::dummy_from_to(Squares::A8, Squares::B8));

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        board.move_piece(&Move::dummy_from_to(Squares::H8, Squares::G8));

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();
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

        board.move_piece(&Move::dummy_from_to(Squares::D2, Squares::D4));
        board.move_piece(&Move::dummy_from_to(Squares::D7, Squares::D5));
        board.move_piece(&Move::dummy_from_to(Squares::E2, Squares::E4));
        board.move_piece(&Move::dummy_from_to(Squares::E7, Squares::E5));
        board.move_piece(&Move::dummy_from_to(Squares::C1, Squares::H6));
        board.move_piece(&Move::dummy_from_to(Squares::C8, Squares::H3));
        board.move_piece(&Move::dummy_from_to(Squares::F1, Squares::A6));
        board.move_piece(&Move::dummy_from_to(Squares::F8, Squares::A3));
        board.move_piece(&Move::dummy_from_to(Squares::D1, Squares::D2));
        board.move_piece(&Move::dummy_from_to(Squares::D8, Squares::D7));
        board.move_piece(&Move::dummy_from_to(Squares::B1, Squares::C3));
        board.move_piece(&Move::dummy_from_to(Squares::B8, Squares::C6));
        board.move_piece(&Move::dummy_from_to(Squares::G1, Squares::F3));
        board.move_piece(&Move::dummy_from_to(Squares::G8, Squares::F6));

        board.display();

        let white_king_side_castle = Move::with_flags(
            KING_CASTLE,
            Squares::E1,
            Squares::G1,
            Color::White,
            PieceType::King,
        );
        let white_queen_side_castle = Move::with_flags(
            QUEEN_CASTLE,
            Squares::E1,
            Squares::C1,
            Color::White,
            PieceType::King,
        );

        let black_king_side_castle = Move::with_flags(
            KING_CASTLE,
            Squares::E8,
            Squares::G8,
            Color::Black,
            PieceType::King,
        );
        let black_queen_side_castle = Move::with_flags(
            QUEEN_CASTLE,
            Squares::E8,
            Squares::C8,
            Color::Black,
            PieceType::King,
        );
        (
            board,
            white_king_side_castle,
            white_queen_side_castle,
            black_king_side_castle,
            black_queen_side_castle,
        )
    }

    #[test]
    fn test_pins() {
        let mut board = Board::new();

        board.move_piece(&Move::dummy_from_to(Squares::A2, Squares::A3));
        board.move_piece(&Move::dummy_from_to(Squares::E7, Squares::E6));
        board.move_piece(&Move::dummy_from_to(Squares::A3, Squares::A4));
        board.move_piece(&Move::dummy_from_to(Squares::D8, Squares::H4));

        board.display();

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::from_to(
            Squares::F2,
            Squares::F3,
            Color::White,
            PieceType::Pawn,
        ));
        not_expected_moves.push(Move::with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::F2,
            Squares::F4,
            Color::White,
            PieceType::Pawn,
        ));

        assert_available_moves(&mut board, Vec::new(), not_expected_moves);
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
        board.move_piece(&Move::dummy_from_to(Squares::B2, Squares::B4));

        // Assert black have both castle moves available

        let mut black_king_castle_moves = Vec::new();

        black_king_castle_moves.push(black_king_side_castle);
        black_king_castle_moves.push(black_queen_side_castle);

        assert_available_moves(&mut board, black_king_castle_moves.clone(), Vec::new());

        board.move_piece(&Move::dummy_from_to(Squares::B7, Squares::B5));
        board.move_piece(&Move::dummy_from_to(Squares::G2, Squares::G4));

        assert_available_moves(&mut board, Vec::new(), white_king_castle_moves);

        board.move_piece(&Move::dummy_from_to(Squares::G7, Squares::G5));

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

        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::E1,
            Squares::D1,
            Color::White,
            PieceType::King,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::E1,
            Squares::D2,
            Color::White,
            PieceType::King,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::E1,
            Squares::E2,
            Color::White,
            PieceType::King,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::E1,
            Squares::F2,
            Color::White,
            PieceType::King,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::E1,
            Squares::F1,
            Color::White,
            PieceType::King,
        ));

        assert_available_moves(&mut board, Vec::new(), not_expected_moves);

        board = Board::empty();

        board.place_piece(Color::White, PieceType::King, BBPositions::E1);
        board.place_piece(Color::Black, PieceType::Rook, BBPositions::F2);
        board.place_piece(Color::White, PieceType::Rook, BBPositions::F1);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(
            Squares::E1,
            Squares::D1,
            Color::White,
            PieceType::King,
        ));
        expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::E1,
            Squares::F2,
            Color::White,
            PieceType::King,
        ));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::from_to(
            Squares::E1,
            Squares::F1,
            Color::White,
            PieceType::King,
        ));

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

        /*
            8 . . . . . . . .
            7 . . . . . . . .
            6 . . . . . . . .
            5 . . . . . . . .
            4 . . . ♟ . . . .
            3 . . . . . ♟ . .
            2 . . ♙ . . . . .
            1 . . ♗ ♕ ♔ . . .
              a b c d e f g h
        */

        board.display();

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(
            Squares::D1,
            Squares::D2,
            Color::White,
            PieceType::Queen,
        ));
        expected_moves.push(Move::from_to(
            Squares::D1,
            Squares::D3,
            Color::White,
            PieceType::Queen,
        ));
        expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D1,
            Squares::D4,
            Color::White,
            PieceType::Queen,
        ));

        expected_moves.push(Move::from_to(
            Squares::D1,
            Squares::E2,
            Color::White,
            PieceType::Queen,
        ));
        expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D1,
            Squares::F3,
            Color::White,
            PieceType::Queen,
        ));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D1,
            Squares::C2,
            Color::White,
            PieceType::Queen,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D1,
            Squares::C2,
            Color::White,
            PieceType::Queen,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D1,
            Squares::C1,
            Color::White,
            PieceType::Queen,
        ));
        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D1,
            Squares::E1,
            Color::White,
            PieceType::Queen,
        ));

        assert_available_moves(&mut board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_bishop_moves() {
        let mut board = Board::empty();

        board.place_piece(Color::White, PieceType::Bishop, BBPositions::A1);

        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::C3);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(
            Squares::A1,
            Squares::B2,
            Color::White,
            PieceType::Bishop,
        ));
        expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::A1,
            Squares::C3,
            Color::White,
            PieceType::Bishop,
        ));

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_rook_moves() {
        let mut board = Board::empty();

        board.place_piece(Color::White, PieceType::Rook, BBPositions::A1);
        board.place_piece(Color::White, PieceType::Bishop, BBPositions::C1);
        board.place_piece(Color::Black, PieceType::Pawn, BBPositions::A3);

        let mut expected_moves = Vec::new();

        expected_moves.push(Move::from_to(
            Squares::A1,
            Squares::A2,
            Color::White,
            PieceType::Rook,
        ));
        expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::A1,
            Squares::A3,
            Color::White,
            PieceType::Rook,
        ));
        expected_moves.push(Move::from_to(
            Squares::A1,
            Squares::B1,
            Color::White,
            PieceType::Rook,
        ));

        let mut not_expected_moves = Vec::new();

        not_expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::A1,
            Squares::C1,
            Color::White,
            PieceType::Rook,
        ));

        assert_available_moves(&mut board, expected_moves, not_expected_moves);
    }

    #[test]
    fn test_get_knight_moves() {
        let mut board = Board::new();

        let mut expected_moves = Vec::new();

        let white_knight_to_c3 =
            Move::from_to(Squares::B1, Squares::C3, Color::White, PieceType::Knight);

        expected_moves.push(white_knight_to_c3.clone());
        expected_moves.push(Move::from_to(
            Squares::B1,
            Squares::A3,
            Color::White,
            PieceType::Knight,
        ));

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(&white_knight_to_c3);
        board.move_piece(&Move::dummy_from_to(Squares::D7, Squares::D5));

        let white_knight_to_d5 = Move::with_flags(
            CAPTURE,
            Squares::C3,
            Squares::D5,
            Color::White,
            PieceType::Knight,
        );

        expected_moves = Vec::new();

        expected_moves.push(white_knight_to_d5.clone());

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_pawn_moves() {
        let mut board = Board::new();

        let mut expected_moves = Vec::new();

        let mut white_pawn_to_d4 = Move::with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::D2,
            Squares::D4,
            Color::White,
            PieceType::Pawn,
        );

        white_pawn_to_d4.set_en_passant_bb_position(BBPositions::D3);
        white_pawn_to_d4.set_en_passant_bb_piece_square(BBPositions::D4);

        expected_moves.push(white_pawn_to_d4.clone());

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(&white_pawn_to_d4);

        expected_moves = Vec::new();

        let mut black_pawn_to_e5 = Move::with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::E7,
            Squares::E5,
            Color::Black,
            PieceType::Pawn,
        );

        black_pawn_to_e5.set_en_passant_bb_position(BBPositions::E6);
        black_pawn_to_e5.set_en_passant_bb_piece_square(BBPositions::E5);

        expected_moves.push(black_pawn_to_e5.clone());

        board.move_piece(&black_pawn_to_e5);

        expected_moves = Vec::new();

        expected_moves.push(Move::with_flags(
            CAPTURE,
            Squares::D4,
            Squares::E5,
            Color::White,
            PieceType::Pawn,
        ));

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }

    #[test]
    fn test_get_en_passant() {
        let mut board = Board::new();

        board.move_piece(&Move::dummy_with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::D2,
            Squares::D4,
        ));
        board.move_piece(&Move::dummy_from_to(Squares::A7, Squares::A6));
        board.move_piece(&Move::dummy_from_to(Squares::D4, Squares::D5));
        board.move_piece(&Move::dummy_from_to(Squares::A6, Squares::A5));
        board.move_piece(&Move::dummy_with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::F2,
            Squares::F4,
        ));
        board.move_piece(&Move::dummy_from_to(Squares::A5, Squares::A4));
        board.move_piece(&Move::dummy_from_to(Squares::F4, Squares::F5));

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

        let mut black_double_push = Move::with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::E7,
            Squares::E5,
            Color::Black,
            PieceType::Pawn,
        );

        black_double_push.set_en_passant_bb_position(BBPositions::E6);
        black_double_push.set_en_passant_bb_piece_square(BBPositions::E5);

        expected_moves.push(black_double_push.clone());

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(&black_double_push);

        expected_moves = Vec::new();

        // Assert that two en passant moves are available when the opponent pawn pushes
        // two squares between two friendly pawns.

        expected_moves.push(Move::with_flags(
            EN_PASSANT,
            Squares::D5,
            Squares::E6,
            Color::White,
            PieceType::Pawn,
        ));
        expected_moves.push(Move::with_flags(
            EN_PASSANT,
            Squares::F5,
            Squares::E6,
            Color::White,
            PieceType::Pawn,
        ));

        assert_available_moves(&mut board, expected_moves, Vec::new());
    }
}
