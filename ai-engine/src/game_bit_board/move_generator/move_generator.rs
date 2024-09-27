use std::{collections::HashMap, u64, usize};
use crate::game_bit_board::{
    _move::{_move::Move, move_contants::*}, board::Board, enums::{Color, PieceType}, move_generator::utils::print_board, positions::{same_anti_diagonal, same_diagonal, same_file, same_rank, Squares}, utils::{
        bitwise_utils::{east_one, get_direction_to_square, north_one, pop_lsb, south_one, to_bitboard_position, west_one},
        utils::is_pawn_in_initial_position,
    }
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
#[derive(Clone)]
pub struct MoveGenerator {
    bishop_lookup_table: HashMap<(u8, u64), u64>,
    rook_lookup_table: HashMap<(u8, u64), u64>,

    in_check: bool,
    in_double_check: bool,
    push_bb: u64,
    attack_bb: u64,
    friendly_pins_moves_bbs: [u64; 64],
    opponent_pin_bb_pos: u64,
    king_allowed_squares: u64
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

            in_check: false,
            in_double_check: false,
            push_bb: 0,
            attack_bb: 0,
            friendly_pins_moves_bbs: [u64::MAX; 64],
            opponent_pin_bb_pos: 0,
            king_allowed_squares: u64::MAX
        }
    }

    // TODO
    // # Approach 1
    // 1. Check if the king is being directly attacked. If so, store that info and
    // the squares that can be occupied by a friendly piece to defend him.
    // 2. If there is a piece along the way from the attacking piece to the king,
    // add it to the bitboard to state it's pinned.
    // # Approach 2
    // [ ] Is the rook/bishop/queen in the same rank/file/diagonal as the king? If not, just skip it
    // To build a defender's bitboard stating all squares that can defend the king.
    // I would only generate moves that have the destination square as one of those
    // in the defender's bb.
    // To build the defender's:
        // 1. Check if king is directly being attacked;
        // 2. If is, add that path from the attacking piece until the one before the king
        // to the defender's bb.
        // 3. If not, check if the friendly defender is pinned (it could not be pinned
        // if there is more than one defending)
        // 4. If it is pinned, the defender's will contain all squares, except the ones
        // the pinned piece could move. The only available will be for slinding pieces
        // that don't leave the column/row/diagonal.
    // General idea: do not generate moves that don't have the destination square
    // as one of the defender's bitboard.
        // If the pinned piece is a pawn, it won't move forward/diagonaly (capture) since the
        // destination square is not in the bitboard.
        // If the pinned piece is a rook, bishop or queen, it will only have moves in the
        // same file/rank and captures of the attacking piece.
        // If the pinned piece is a knight, it won't be able to move out of the way.

    fn _calculate_attack_data(&mut self, board: &mut Board) {
        self.in_check = false;
        self.in_double_check = false;
        self.push_bb = 0;
        self.attack_bb = 0;
        self.friendly_pins_moves_bbs = [u64::MAX; 64];
        self.opponent_pin_bb_pos = 0;
        self.king_allowed_squares = u64::MAX;

        let king_bb_pos = board.get_piece_positions(board.get_side_to_move(), PieceType::King);
        let king_square = pop_lsb(&mut king_bb_pos.clone()) as usize;
        let opponent = board.get_side_to_move().opponent();

        // Handle Rook Attacks
        self.check_sliding_attacks(
            board, opponent, PieceType::Rook, king_square, &king_bb_pos
        );

        // Handle Rook Attacks
        self.check_sliding_attacks(
            board, opponent, PieceType::Bishop, king_square, &king_bb_pos
        );

        // Handle Queen Attacks
        self.check_sliding_attacks(
            board, opponent, PieceType::Queen, king_square, &king_bb_pos
        );

        if self.push_bb == 0 {
            self.push_bb = u64::MAX;
        }

        if self.attack_bb == 0 {
            self.attack_bb = u64::MAX;
        }

        println!("\nPush bb");

        print_board(Color::White, king_square as u64, PieceType::King, self.push_bb);

        println!("\nAttack bb");

        print_board(Color::White, king_square as u64, PieceType::King, self.attack_bb);

        // println!("\nOpponent pins");

        // print_board(Color::White, u64::MAX, PieceType::Empty, self.opponent_pin_bb_pos);

        // self.friendly_pins_moves_bbs.iter().enumerate().for_each(|(i, bb)| {
        //     if *bb != u64::MAX {
        //         println!("\nFriendly pin at {}", Squares::to_string(i));

        //         print_board(Color::White, i as u64, board.get_piece_type(i), *bb);
        //     }
        // });

        // println!("In Check: {}", self.in_check);

        // println!("In Double Check: {}", self.in_double_check);

        // (push_bb, in_check, double_check)
    }

    // Helper function for sliding piece attacks (rooks, bishops, queens)
    fn check_sliding_attacks(
        &mut self, board: &mut Board, opponent: Color, piece_type: PieceType,
        king_square: usize, king_bb_pos: &u64
    ) {
        if self.in_double_check {
            return;
        }

        let mut opponent_pieces = board.get_piece_positions(opponent, piece_type);

        // if opponent_pieces & king_attacker_positions == 0 {
        //     // println!("There is no {} attacking king at {}", piece_type, Squares::to_string(king_square));
        //     return;
        // }

        while opponent_pieces != 0 {
            if self.in_double_check {
                break;
            }

            let square = pop_lsb(&mut opponent_pieces) as usize;

            let mut attacks = 0;

            let same_orthogonal_ray = same_rank(square, king_square) || same_file(square, king_square);
            let same_diagonal_ray = same_diagonal(square, king_square) || same_anti_diagonal(square, king_square);

            if piece_type == PieceType::Queen && (same_orthogonal_ray || same_diagonal_ray){
                attacks |= self.get_orthogonal_attacks(board, opponent, square);
                attacks |= self.get_diagonal_attacks(board, opponent, square);
            }
            else if piece_type == PieceType::Rook && same_orthogonal_ray {
                attacks |= self.get_orthogonal_attacks(board, opponent, square);
            } else if piece_type == PieceType::Bishop && same_diagonal_ray {
                attacks |= self.get_diagonal_attacks(board, opponent, square);
            } else {
                continue;
            }

            // print_board(opponent.opponent(), square as u64, piece_type, attacks);

            if attacks & king_bb_pos != 0 {
                // println!("{} at {} is attacking king at {}", piece_type, Squares::to_string(square), Squares::to_string(king_square));
                self.attack_bb |= 1 << square;

                self.handle_check(king_bb_pos, square, king_square);
            } else {
                self.handle_pins(board, square, king_square);
            }
        }
    }

    fn handle_check(&mut self, king_bb_pos: &u64, square: usize, king_square: usize) {
        self.in_double_check = self.in_check;
        self.in_check = true;
        let direction = get_direction_to_square(square, king_square);
        let mut path_to_king = direction(1 << square);
        let mut current_pos = path_to_king;

        while current_pos & king_bb_pos == 0 {
            path_to_king |= current_pos;
            current_pos = direction(current_pos);
        }

        current_pos = direction(current_pos);

        while current_pos != 0 {
            self.king_allowed_squares &= !current_pos;

            current_pos = direction(current_pos);
        }

        self.push_bb |= path_to_king;
    }

    fn handle_pins(&mut self, board: &mut Board, square: usize, king_square: usize) {
        let direction_fn = get_direction_to_square(square, king_square);

        let attacker_bb_pos = 1 << square;

        let king_bb_pos = 1 << king_square;

        let mut path_to_king = attacker_bb_pos;
        let mut current_pos = direction_fn(path_to_king);

        let side_to_move = board.get_side_to_move();

        let mut friendly_pin_bb_pos = 0;
        let mut opponent_pin_bb_pos = 0;
        while current_pos != 0 {
            if king_bb_pos == current_pos {
                break;
            }

            let piece_type = board.get_piece_type_by_bb_pos(current_pos);

            if piece_type != PieceType::Empty {
                let piece_color = board.get_piece_color_by_bb_pos(current_pos);

                if piece_color == side_to_move {
                    if friendly_pin_bb_pos != 0 || opponent_pin_bb_pos != 0 {
                        return;
                    }

                    friendly_pin_bb_pos = current_pos;
                } else {
                    if friendly_pin_bb_pos != 0 || opponent_pin_bb_pos != 0 {
                        return;
                    }

                    opponent_pin_bb_pos = current_pos;
                }
            }

            path_to_king |= current_pos;
            current_pos = direction_fn(current_pos);
        }

        if friendly_pin_bb_pos != 0 {
            self.friendly_pins_moves_bbs[pop_lsb(&mut friendly_pin_bb_pos) as usize] = path_to_king;
            // self.push_bb |= path_to_king;
        }
        else if opponent_pin_bb_pos != 0 {
            self.opponent_pin_bb_pos |= opponent_pin_bb_pos;
        }
    }

    pub fn get_moves(&mut self, board: &mut Board) -> Vec<Move> {
        if board.is_game_finished() {
            panic!("Can't generate moves. Game has already ended.");
        }

        // Calculate attack data
        self._calculate_attack_data(board);

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
                self.get_pawn_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::Knight {
                self.get_knight_moves(board, &mut moves, square, color, attacks);
            } else if piece_type == PieceType::Rook {
                self.get_orthogonal_moves(board, &mut moves, square, color, attacks, PieceType::Rook);
            } else if piece_type == PieceType::Bishop {
                self.get_diagonal_moves(board, &mut moves, square, color, attacks, PieceType::Bishop);
            } else if piece_type == PieceType::Queen {
                self.get_orthogonal_moves(board, &mut moves, square, color, attacks, PieceType::Queen);
                self.get_diagonal_moves(board, &mut moves, square, color, attacks, PieceType::Queen);
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
            );
        }

        moves.retain(|_move| !opponent_piece_squares.contains(&_move.get_from()));

        if self.in_double_check {
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
        attacked_squares: &mut u64, piece_type: PieceType
    ) {
        let attacks = self.get_diagonal_attacks(board, color, square) & (self.push_bb | self.attack_bb);

        *attacked_squares |= attacks;

        _create_moves(attacks, board.get_player_pieces_positions(color.opponent()), moves, square, color, piece_type);
    }

    fn get_diagonal_attacks(&self, board: &Board, color: Color, square: usize) -> u64 {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & BISHOP_RELEVANT_SQUARES[square];

        let attacks = *self
            .bishop_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        (attacks & !friendly_pieces_bb) & self.friendly_pins_moves_bbs[square]
    }

    fn get_orthogonal_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64, piece_type: PieceType
    ) {
        let attacks = self.get_orthogonal_attacks(board, color, square) & (self.push_bb | self.attack_bb);

        *attacked_squares |= attacks;

        _create_moves(attacks, board.get_player_pieces_positions(color.opponent()), moves, square, color, piece_type);
    }

    fn get_orthogonal_attacks(&self, board: &Board, color: Color, square: usize) -> u64 {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & ROOK_RELEVANT_SQUARES[square];

        let attacks = *self
            .rook_lookup_table
            .get(&(square as u8, occupied_relevant_squares))
            .unwrap();

        (attacks & !friendly_pieces_bb) & self.friendly_pins_moves_bbs[square]
    }

    fn get_knight_moves(
        &self,board: &Board, moves: &mut Vec<Move>, square: usize, color: Color,
        attacked_squares: &mut u64,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let attacks = (KNIGHT_MOVES[square] & !friendly_pieces_bb) & self.friendly_pins_moves_bbs[square] & (self.push_bb | self.attack_bb);

        *attacked_squares |= attacks;

        _create_moves(attacks, opponent_pieces_bb, moves, square, color, PieceType::Knight);
    }

    fn get_king_moves(
        &mut self, board: &Board, moves: &mut Vec<Move>, square: usize, color: Color, opponent_attacks: &u64,
        attacked_squares: &mut u64,
    ) {
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_squares = friendly_pieces_bb | opponent_pieces_bb;

        // I guess I can just use & self.friendly_pins_moves_bbs[square] instead of having a king allowed squares var
        let attacks = ((KING_MOVES[square] & !friendly_pieces_bb) & !opponent_attacks) & self.king_allowed_squares;

        *attacked_squares |= attacks;

        _create_moves(attacks, opponent_pieces_bb, moves, square, color, PieceType::King);

        if to_bitboard_position(square as u64) & opponent_attacks != 0 {
            self.in_double_check = self.in_check;
            self.in_check = true;

            if attacks == 0 {
                return;
            }
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
        attacked_squares: &mut u64,
    ) {
        // TODO: promotions
        let friendly_pieces_bb = board.get_player_pieces_positions(color);
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_squares = friendly_pieces_bb | opponent_pieces_bb;

        let raw_attacks = (MoveGenerator::look_up_pawn_attacks(color, square) & !friendly_pieces_bb) & self.friendly_pins_moves_bbs[square] & (self.push_bb | self.attack_bb);

        // if self.friendly_pins_moves_bbs[square] != u64::MAX {
        //     println!("Pins moves bb is not all!! Generating for square {}", Squares::to_string(square));
        //     print_board(color, square as u64, PieceType::Pawn, self.friendly_pins_moves_bbs[square]);
        // }

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
        let mut forward_one = raw_forward_one & self.friendly_pins_moves_bbs[square] & self.push_bb;

        if is_pawn_in_initial_position(bb_position, color.is_white()) {
            let mut forward_two = (offset_fn(raw_forward_one) & !occupied_squares) & self.friendly_pins_moves_bbs[square] & self.push_bb;

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
                & board.get_en_passant() & self.friendly_pins_moves_bbs[square] & (self.push_bb | self.attack_bb);

            // *attacked_squares |= attacks;

            while attacks != 0 {
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

fn _create_moves(mut attacks: u64, opponent_pieces_bb: u64, moves: &mut Vec<Move>, square: usize, color: Color, piece_type: PieceType) {
    while attacks != 0 {
        let target_square = pop_lsb(&mut attacks);

        let mut flags: u16 = 0;
        if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
            flags = CAPTURE;
        }

        moves.push(Move::with_flags(
            flags,
            square,
            target_square as usize,
            color,
            piece_type,
        ));
    }
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

    static MOVE_GENERATOR: Lazy<Mutex<MoveGenerator>> = Lazy::new(|| Mutex::new(MoveGenerator::new()));

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
