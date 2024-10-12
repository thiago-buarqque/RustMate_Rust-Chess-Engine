use rand::Rng;

use super::{
    attack_data::AttackData, contants::{
        BISHOP_RELEVANT_SQUARES, BLACK_KING_SIDE_PATH_TO_ROOK, BLACK_QUEEN_SIDE_PATH_TO_ROOK,
        KING_MOVES, KNIGHT_MOVES, ROOK_RELEVANT_SQUARES, WHITE_KING_SIDE_PATH_TO_ROOK,
        WHITE_QUEEN_SIDE_PATH_TO_ROOK,
    }, magics::{BISHOP_MAGICS, BISHOP_SHIFTS, ROOK_MAGICS, ROOK_SHIFTS}, raw_move_generator::RawMoveGenerator, utils::{is_en_passant_discovered_check, is_promotion_square, look_up_pawn_attacks}
};
use crate::game_bit_board::{
    _move::{_move::Move, move_contants::*}, board::Board, enums::{Color, PieceType}, move_generator::random::Random, utils::{
        bitwise_utils::{east_one, north_one, pop_lsb, south_one, to_bitboard_position, west_one},
        utils::{estimate_memory_usage_in_bytes, is_pawn_in_initial_position},
    }
};
use std::{collections::HashMap, u64, usize};

#[derive(Clone)]
pub struct MoveGenerator {
    bishop_lookup_hash_table: HashMap<(u8, u64), u64>,
    rook_lookup_hash_table: HashMap<(u8, u64), u64>,
    bishop_lookup_table: [Vec<u64>; 64],
    rook_lookup_table: [Vec<u64>; 64],
    friendly_pieces_bb: u64,
    opponent_pieces_bb: u64,
    occupied_squares: u64,
    side_to_move: Color,
    attack_data: AttackData,
}

#[inline(always)]
fn is_collision_detected(actual: &[u64], hash: usize, attacks: u64) -> bool {
    actual[hash] != 0 && actual[hash] != attacks
}

impl MoveGenerator {
    pub fn new() -> Self {
        let bishop_lookup_hash_table: HashMap<(u8, u64), u64> =
            RawMoveGenerator::create_bishop_lookup_table();
        let rook_lookup_hash_table: HashMap<(u8, u64), u64> =
            RawMoveGenerator::create_rook_lookup_table();

        let mut bishop_lookup_table= [const { Vec::new() }; 64];
        let mut rook_lookup_table= [const { Vec::new() }; 64];

        for square in 0..64 {
            let magic = ROOK_MAGICS[square];
            let shift = ROOK_SHIFTS[square];

            let mut keys = rook_lookup_hash_table.keys().filter(|key| key.0 == (square as u8)).collect::<Vec<&(u8, u64)>>();

            keys.sort();

            rook_lookup_table[square].reserve_exact(keys.len());

            rook_lookup_table[square] = vec![0; keys.len()];

            for key in keys {
                let hash = (key.1.wrapping_mul(magic) >> shift) as usize;
                
                if rook_lookup_table[square][hash] != 0 {
                    panic!("(Rook conflict) Hash: {} key: {:?} value: {} stored: {}", hash, key, *rook_lookup_hash_table.get(key).unwrap(), rook_lookup_table[square][hash]);
                }

                rook_lookup_table[square][hash] = *rook_lookup_hash_table.get(key).unwrap();
            }
        }

        for square in 0..64 {
            let magic = BISHOP_MAGICS[square];
            let shift = BISHOP_SHIFTS[square];

            let mut keys = bishop_lookup_hash_table.keys().filter(|key| key.0 == (square as u8)).collect::<Vec<&(u8, u64)>>();

            keys.sort();

            bishop_lookup_table[square].reserve_exact(keys.len());

            bishop_lookup_table[square] = vec![0; keys.len()];

            for key in keys {
                let hash = (key.1.wrapping_mul(magic) >> shift) as usize;

                if bishop_lookup_table[square][hash] != 0 {
                    panic!("(Bishop conflict) Hash: {} key: {:?} value: {} stored: {}", hash, key, *bishop_lookup_hash_table.get(key).unwrap(), bishop_lookup_table[square][hash]);
                }

                bishop_lookup_table[square][hash] = *bishop_lookup_hash_table.get(key).unwrap();
            }
        }

        Self {
            bishop_lookup_hash_table,
            rook_lookup_hash_table,
            bishop_lookup_table,
            rook_lookup_table,
            friendly_pieces_bb: 0,
            opponent_pieces_bb: 0,
            occupied_squares: 0,
            side_to_move: Color::White,
            attack_data: AttackData::new(),
        }
    }

    fn generate_candidate_magic(&self, rng: &mut rand::rngs::ThreadRng) -> u64 {
        loop {
            let magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
            // Ensure certain bits are set
            if (magic & 0xFF00000000000000) != 0 && (magic & 0x00000000000000FF) != 0 {
                return magic;
            }
        }
    }

    pub fn find_magics(&self) {
        let mut rng = rand::thread_rng();

        let mut magics = [0; 64];
        let mut shifts = [0; 64];
        let mut sizes = [0;64];
        let mut size = 0;

        for square in 0..64 {
            let mask = ROOK_RELEVANT_SQUARES[square];

            let shift = 64 - mask.count_ones();
            
            let mut keys = self.rook_lookup_hash_table.keys().filter(|key| key.0 == square as u8).collect::<Vec<&(u8, u64)>>();

            keys.sort();

            let mut colisions_count = 0;

            for _ in 0..1_000_000 {
                let magic = self.generate_candidate_magic(&mut rng);
                
                if (mask.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
                    continue;
                }
                

                let mut actual = vec![0; keys.len()];

                let mut colision = false;

                for key in &keys {
                    let hash = (key.1.wrapping_mul(magic) >> shift) as usize;

                    let attacks = self.rook_lookup_hash_table.get(*key).unwrap();

                    if is_collision_detected(&actual, hash, *attacks) {
                        colision = true;
                        colisions_count += 1;

                        break;
                    }

                    actual[hash] = *attacks;
                }

                if !colision {
                    magics[square] = magic;
                    shifts[square] = shift;

                    let _size =estimate_memory_usage_in_bytes::<u64>(keys.len());

                    sizes[square] = _size;

                    size += _size;

                    break;
                }
            }

            println!("Colision count for square {square}: {colisions_count}");
        }

        println!("const MAGIC_NUMBERS = {:?};", magics);
        println!("const SHIFTS = {:?};", shifts);
        println!("Total lookup table size {:?}kb", size / 1024);
    }

    pub fn init(&mut self, board: &mut Board) {
        self.side_to_move = board.get_side_to_move();

        self.friendly_pieces_bb = board.get_player_pieces_positions(self.side_to_move);
        self.opponent_pieces_bb = board.get_player_pieces_positions(self.side_to_move.opponent());
        self.occupied_squares = self.friendly_pieces_bb | self.opponent_pieces_bb;
    }

    pub fn get_moves(&mut self, board: &mut Board) -> Vec<Move> {
        if board.is_game_finished() {
            panic!("Can't generate moves. Game has already ended.");
        }

        self.init(board);

        // Calculate attack data
        let mut attack_data = AttackData::new();

        attack_data.calculate_attack_data(board, self);

        let mut moves = Vec::with_capacity(32);

        let mut friendly_king_square = usize::MAX;

        let mut friendly_pieces = self.friendly_pieces_bb.clone();

        while friendly_pieces != 0 {
            let square = pop_lsb(&mut friendly_pieces) as usize;

            let piece_type: PieceType = board.get_piece_type(square);

            if piece_type == PieceType::Pawn {
                self.generate_pawn_moves(board, &mut moves, square, &attack_data);
            } else if piece_type == PieceType::Knight {
                self.generate_knight_moves(&mut moves, square, &attack_data);
            } else if piece_type == PieceType::Rook {
                self.generate_orthogonal_moves(
                    board,
                    &mut moves,
                    square,
                    PieceType::Rook,
                    &attack_data,
                );
            } else if piece_type == PieceType::Bishop {
                self.generate_diagonal_moves(
                    board,
                    &mut moves,
                    square,
                    PieceType::Bishop,
                    &attack_data,
                );
            } else if piece_type == PieceType::Queen {
                self.generate_orthogonal_moves(
                    board,
                    &mut moves,
                    square,
                    PieceType::Queen,
                    &attack_data,
                );
                self.generate_diagonal_moves(
                    board,
                    &mut moves,
                    square,
                    PieceType::Queen,
                    &attack_data,
                );
            } else if piece_type == PieceType::King {
                friendly_king_square = square
            }
        }

        let mut opponent_king =
            board.get_piece_positions(self.side_to_move.opponent(), PieceType::King);

        let opponent_king_square = pop_lsb(&mut opponent_king) as usize;

        if opponent_king_square < 64 {
            attack_data.king_allowed_squares &= !KING_MOVES[opponent_king_square];
        }

        if friendly_king_square < 64 {
            self.generate_king_moves(board, &mut moves, friendly_king_square, &attack_data);
        }

        if attack_data.in_double_check {
            moves.retain(|_move| _move.get_from() == friendly_king_square);
        }

        if moves.is_empty() {
            // Game ended
            board.set_winner(Some(self.side_to_move.opponent()));
        }

        moves
    }

    pub fn create_moves(
        &self, mut attacks: u64, moves: &mut Vec<Move>, square: usize, piece_type: PieceType,
    ) {
        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            let mut flags: u16 = 0;
            if to_bitboard_position(target_square as u64) & self.opponent_pieces_bb != 0 {
                flags = CAPTURE;
            }

            moves.push(Move::with_flags(
                flags,
                square,
                target_square as usize,
                self.side_to_move,
                piece_type,
            ));
        }
    }

    fn generate_diagonal_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, piece_type: PieceType,
        attack_data: &AttackData,
    ) {
        let raw_attacks =
            self.get_diagonal_attacks(board, self.side_to_move, square, &self.friendly_pieces_bb);

        let attacks = raw_attacks
            & !self.friendly_pieces_bb
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        self.create_moves(attacks, moves, square, piece_type);
    }

    /// This method is one of the two that I am not using `friendly_pieces_bb`
    /// and `opponent_pieces_bb` I have in this struct. I am lazy to
    /// work-around that now
    pub fn get_diagonal_attacks(
        &self, board: &Board, color: Color, square: usize, friendly_pieces_bb: &u64,
    ) -> u64 {
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());

        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & BISHOP_RELEVANT_SQUARES[square];

        let hash = (occupied_relevant_squares.wrapping_mul(BISHOP_MAGICS[square]) >> BISHOP_SHIFTS[square]) as usize;

        let attacks = self
            .bishop_lookup_table[square][hash];

        attacks
    }

    fn generate_orthogonal_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, piece_type: PieceType,
        attack_data: &AttackData,
    ) {
        let raw_attacks =
            self.get_orthogonal_attacks(board, self.side_to_move, square, &self.friendly_pieces_bb);

        let attacks = raw_attacks
            & !self.friendly_pieces_bb
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        self.create_moves(attacks, moves, square, piece_type);
    }

    /// This method is one of the two that I am not using `friendly_pieces_bb`
    /// and `opponent_pieces_bb` I have in this struct. I am lazy to
    /// work-around that now
    pub fn get_orthogonal_attacks(
        &self, board: &Board, color: Color, square: usize, friendly_pieces_bb: &u64,
    ) -> u64 {
        let opponent_pieces_bb = board.get_player_pieces_positions(color.opponent());
        let occupied_relevant_squares =
            (friendly_pieces_bb | opponent_pieces_bb) & ROOK_RELEVANT_SQUARES[square];

        let hash = (occupied_relevant_squares.wrapping_mul(ROOK_MAGICS[square]) >> ROOK_SHIFTS[square]) as usize;

        let attacks = self
            .rook_lookup_table[square][hash];

        attacks
    }

    fn generate_knight_moves(
        &self, moves: &mut Vec<Move>, square: usize, attack_data: &AttackData,
    ) {
        let attacks = (KNIGHT_MOVES[square] & !self.friendly_pieces_bb)
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb);

        self.create_moves(attacks, moves, square, PieceType::Knight);
    }

    fn generate_king_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, attack_data: &AttackData,
    ) {
        let attacks =
            (KING_MOVES[square] & !self.friendly_pieces_bb) & attack_data.king_allowed_squares;

        self.create_moves(attacks, moves, square, PieceType::King);

        if attack_data.in_check {
            return;
        }

        if board.has_queen_side_castle_right(self.side_to_move) {
            if ((self.occupied_squares & BLACK_QUEEN_SIDE_PATH_TO_ROOK) == 0
                && self.side_to_move.is_black())
                || ((self.occupied_squares & WHITE_QUEEN_SIDE_PATH_TO_ROOK) == 0
                    && self.side_to_move.is_white())
            {
                let _west_one = west_one(1 << square);
                let mut west_two = west_one(_west_one);

                if _west_one & attack_data.king_allowed_squares != 0
                    && west_two & attack_data.king_allowed_squares != 0
                {
                    let _move = Move::with_flags(
                        QUEEN_CASTLE,
                        square,
                        pop_lsb(&mut west_two) as usize,
                        self.side_to_move,
                        PieceType::King,
                    );

                    moves.push(_move);
                }
            }
        }

        if board.has_king_side_castle_right(self.side_to_move) {
            if ((self.occupied_squares & BLACK_KING_SIDE_PATH_TO_ROOK) == 0
                && self.side_to_move.is_black())
                || ((self.occupied_squares & WHITE_KING_SIDE_PATH_TO_ROOK) == 0
                    && self.side_to_move.is_white())
            {
                let _east_one = east_one(1 << square);
                let mut east_two = east_one(_east_one);

                if _east_one & attack_data.king_allowed_squares != 0
                    && east_two & attack_data.king_allowed_squares != 0
                {
                    let _move = Move::with_flags(
                        KING_CASTLE,
                        square,
                        pop_lsb(&mut east_two) as usize,
                        self.side_to_move,
                        PieceType::King,
                    );

                    moves.push(_move);
                }
            }
        }
    }

    fn generate_pawn_moves(
        &self, board: &Board, moves: &mut Vec<Move>, square: usize, attack_data: &AttackData,
    ) {
        self.generate_pawn_attacks(square, attack_data, moves);

        let pawn_bb_position = to_bitboard_position(square as u64);

        let offset_fn = if self.side_to_move.is_white() {
            north_one
        } else {
            south_one
        };

        let raw_forward_one = offset_fn(pawn_bb_position) & !self.occupied_squares;

        let mut forward_one = raw_forward_one
            & attack_data.friendly_pins_moves_bbs[square]
            & attack_data.defenders_bb;

        if is_pawn_in_initial_position(pawn_bb_position, self.side_to_move.is_white()) {
            self.generate_double_pawn_push_move(
                offset_fn,
                raw_forward_one,
                attack_data,
                square,
                moves,
            );
        } else if board.get_en_passant_bb_position() != 0 {
            self.generate_en_passant_move(square, board, attack_data, moves);
        }

        // Add "push one square" move

        if forward_one == 0 {
            return;
        }

        let target_square = pop_lsb(&mut forward_one);

        if is_promotion_square(self.side_to_move, target_square as usize) {
            self.add_promotion_moves(false, square, moves, target_square as usize);
        } else {
            moves.push(Move::from_to(
                square,
                target_square as usize,
                self.side_to_move,
                PieceType::Pawn,
            ));
        }
    }

    fn generate_pawn_attacks(
        &self, square: usize, attack_data: &AttackData, moves: &mut Vec<Move>,
    ) {
        let raw_attacks = look_up_pawn_attacks(self.side_to_move, square);

        let mut attacks = (raw_attacks & !self.friendly_pieces_bb)
            & attack_data.friendly_pins_moves_bbs[square]
            & (attack_data.defenders_bb | attack_data.attack_bb)
            & self.opponent_pieces_bb;

        while attacks != 0 {
            let target_square = pop_lsb(&mut attacks);

            if is_promotion_square(self.side_to_move, target_square as usize) {
                self.add_promotion_moves(true, square, moves, target_square as usize);
            } else {
                moves.push(Move::with_flags(
                    CAPTURE,
                    square,
                    target_square as usize,
                    self.side_to_move,
                    PieceType::Pawn,
                ));
            }
        }
    }

    fn add_promotion_moves(
        &self, capture: bool, from: usize, moves: &mut Vec<Move>, target_square: usize,
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
                self.side_to_move,
                PieceType::Pawn,
            ));
        }
    }

    fn generate_double_pawn_push_move(
        &self, offset_fn: fn(u64) -> u64, raw_forward_one: u64, attack_data: &AttackData,
        square: usize, moves: &mut Vec<Move>,
    ) {
        let mut forward_two = (offset_fn(raw_forward_one) & !self.occupied_squares)
            & attack_data.friendly_pins_moves_bbs[square]
            & attack_data.defenders_bb;

        if forward_two != 0 {
            let en_passant_bb_piece_square = forward_two;

            let target_square = pop_lsb(&mut forward_two) as usize;

            let mut _move = Move::with_flags(
                DOUBLE_PAWN_PUSH,
                square,
                target_square,
                self.side_to_move,
                PieceType::Pawn,
            );

            _move.set_en_passant_bb_position(raw_forward_one);
            _move.set_en_passant_bb_piece_square(en_passant_bb_piece_square);

            moves.push(_move);
        }
    }

    fn generate_en_passant_move(
        &self, square: usize, board: &Board, attack_data: &AttackData, moves: &mut Vec<Move>,
    ) {
        let mut attacks = (look_up_pawn_attacks(self.side_to_move, square)
            & !self.friendly_pieces_bb)
            & board.get_en_passant_bb_position()
            & attack_data.friendly_pins_moves_bbs[square];

        // King is under attack and en passant captures the attacking pawn
        if attack_data.defenders_bb == 0
            && (attack_data.attack_bb & board.get_en_passant_piece_square_bb() != 0)
        {
            attacks &= board.get_en_passant_bb_position();
        } else {
            attacks &= attack_data.defenders_bb | attack_data.attack_bb
        }

        // A pawn will never be able to have more than one
        // en passant move at the same time
        while attacks != 0
            && !is_en_passant_discovered_check(self.side_to_move, attack_data, square, board)
        {
            let target_square = pop_lsb(&mut attacks);

            moves.push(Move::with_flags(
                EN_PASSANT,
                square,
                target_square as usize,
                self.side_to_move,
                PieceType::Pawn,
            ));
        }
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
            mut white_king_side_castle,
            mut white_queen_side_castle,
            mut black_king_side_castle,
            mut black_queen_side_castle,
        ) = set_up_castle_position();

        // White: Make sure castle rights are lost after castle is performed

        board.move_piece(&mut white_king_side_castle);

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        assert_eq!(PieceType::King, board.get_piece_type(Squares::G1));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::F1));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(&mut white_queen_side_castle);

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        assert_eq!(PieceType::King, board.get_piece_type(Squares::C1));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::D1));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(&mut Move::dummy_from_to(Squares::E1, Squares::E2));

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        board.move_piece(&mut Move::dummy_from_to(Squares::A1, Squares::B1));

        assert_eq!(true, board.has_king_side_castle_right(Color::White));
        assert_eq!(false, board.has_queen_side_castle_right(Color::White));

        board.unmake_last_move();

        board.move_piece(&mut Move::dummy_from_to(Squares::H1, Squares::G1));

        assert_eq!(false, board.has_king_side_castle_right(Color::White));
        assert_eq!(true, board.has_queen_side_castle_right(Color::White));

        // Black: Make sure castle rights are lost after castle is performed

        board.move_piece(&mut black_king_side_castle);

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        assert_eq!(PieceType::King, board.get_piece_type(Squares::G8));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::F8));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(&mut black_queen_side_castle);

        assert_eq!(PieceType::King, board.get_piece_type(Squares::C8));
        assert_eq!(PieceType::Rook, board.get_piece_type(Squares::D8));

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(&mut Move::dummy_from_to(Squares::E8, Squares::E7));

        assert_eq!(false, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(true, board.has_queen_side_castle_right(Color::Black));

        board.move_piece(&mut Move::dummy_from_to(Squares::A8, Squares::B8));

        assert_eq!(true, board.has_king_side_castle_right(Color::Black));
        assert_eq!(false, board.has_queen_side_castle_right(Color::Black));

        board.unmake_last_move();

        board.move_piece(&mut Move::dummy_from_to(Squares::H8, Squares::G8));

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

        board.move_piece(&mut Move::dummy_from_to(Squares::D2, Squares::D4));
        board.move_piece(&mut Move::dummy_from_to(Squares::D7, Squares::D5));
        board.move_piece(&mut Move::dummy_from_to(Squares::E2, Squares::E4));
        board.move_piece(&mut Move::dummy_from_to(Squares::E7, Squares::E5));
        board.move_piece(&mut Move::dummy_from_to(Squares::C1, Squares::H6));
        board.move_piece(&mut Move::dummy_from_to(Squares::C8, Squares::H3));
        board.move_piece(&mut Move::dummy_from_to(Squares::F1, Squares::A6));
        board.move_piece(&mut Move::dummy_from_to(Squares::F8, Squares::A3));
        board.move_piece(&mut Move::dummy_from_to(Squares::D1, Squares::D2));
        board.move_piece(&mut Move::dummy_from_to(Squares::D8, Squares::D7));
        board.move_piece(&mut Move::dummy_from_to(Squares::B1, Squares::C3));
        board.move_piece(&mut Move::dummy_from_to(Squares::B8, Squares::C6));
        board.move_piece(&mut Move::dummy_from_to(Squares::G1, Squares::F3));
        board.move_piece(&mut Move::dummy_from_to(Squares::G8, Squares::F6));

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

        board.move_piece(&mut Move::dummy_from_to(Squares::A2, Squares::A3));
        board.move_piece(&mut Move::dummy_from_to(Squares::E7, Squares::E6));
        board.move_piece(&mut Move::dummy_from_to(Squares::A3, Squares::A4));
        board.move_piece(&mut Move::dummy_from_to(Squares::D8, Squares::H4));

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
        board.move_piece(&mut Move::dummy_from_to(Squares::B2, Squares::B4));

        // Assert black have both castle moves available

        let mut black_king_castle_moves = Vec::new();

        black_king_castle_moves.push(black_king_side_castle);
        black_king_castle_moves.push(black_queen_side_castle);

        assert_available_moves(&mut board, black_king_castle_moves.clone(), Vec::new());

        board.move_piece(&mut Move::dummy_from_to(Squares::B7, Squares::B5));
        board.move_piece(&mut Move::dummy_from_to(Squares::G2, Squares::G4));

        assert_available_moves(&mut board, Vec::new(), white_king_castle_moves);

        board.move_piece(&mut Move::dummy_from_to(Squares::G7, Squares::G5));

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

        let mut white_knight_to_c3 =
            Move::from_to(Squares::B1, Squares::C3, Color::White, PieceType::Knight);

        expected_moves.push(white_knight_to_c3.clone());
        expected_moves.push(Move::from_to(
            Squares::B1,
            Squares::A3,
            Color::White,
            PieceType::Knight,
        ));

        assert_available_moves(&mut board, expected_moves, Vec::new());

        board.move_piece(&mut white_knight_to_c3);
        board.move_piece(&mut Move::dummy_from_to(Squares::D7, Squares::D5));

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

        board.move_piece(&mut white_pawn_to_d4);

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

        board.move_piece(&mut black_pawn_to_e5);

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

        board.move_piece(&mut Move::dummy_with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::D2,
            Squares::D4,
        ));
        board.move_piece(&mut Move::dummy_from_to(Squares::A7, Squares::A6));
        board.move_piece(&mut Move::dummy_from_to(Squares::D4, Squares::D5));
        board.move_piece(&mut Move::dummy_from_to(Squares::A6, Squares::A5));
        board.move_piece(&mut Move::dummy_with_flags(
            DOUBLE_PAWN_PUSH,
            Squares::F2,
            Squares::F4,
        ));
        board.move_piece(&mut Move::dummy_from_to(Squares::A5, Squares::A4));
        board.move_piece(&mut Move::dummy_from_to(Squares::F4, Squares::F5));

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

        board.move_piece(&mut black_double_push);

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
