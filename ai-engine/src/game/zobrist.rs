extern crate rand;

use rand::{Rng, rngs::StdRng, SeedableRng};

use crate::common::contants::EMPTY_PIECE;

use super::{board_state::BoardState, zobrist_utils::get_piece_index};

#[derive(Debug, Clone)]
pub struct Zobrist {
    black_can_rook_castle: u64,
    black_can_queen_castle: u64,
    black_pawn_en_passant: u64,
    hash: u64,
    table: Vec<Vec<u64>>,
    white_can_rook_castle: u64,
    white_can_queen_castle: u64,
    white_pawn_en_passant: u64,
    white_to_move: u64,
}

// fn random_bitstring() -> u64 {
//     let mut rng = StdRng::seed_from_u64(222);
//     rng.gen::<u64>()
// }

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(222);

        let mut table = vec![vec![0u64; 12]; 64];

        for row in table.iter_mut() {
            for j in 0..row.len() {
                row[j] = rng.gen::<u64>();
            }
        }

        let black_can_rook_castle = rng.gen::<u64>();
        let black_can_queen_castle = rng.gen::<u64>();
        let black_pawn_en_passant = rng.gen::<u64>();

        let white_can_rook_castle = rng.gen::<u64>();
        let white_can_queen_castle = rng.gen::<u64>();
        let white_pawn_en_passant = rng.gen::<u64>();
        let white_to_move = rng.gen::<u64>();

        Self {
            black_can_rook_castle,
            black_can_queen_castle,
            black_pawn_en_passant,
            hash: 0,
            table,
            white_can_rook_castle,
            white_can_queen_castle,
            white_pawn_en_passant,
            white_to_move,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            black_can_rook_castle: self.black_can_rook_castle,
            black_can_queen_castle: self.black_can_queen_castle,
            black_pawn_en_passant: self.black_pawn_en_passant,
            hash: self.hash,
            table: self.table.clone(),
            white_can_rook_castle: self.white_can_rook_castle,
            white_can_queen_castle: self.white_can_queen_castle,
            white_pawn_en_passant: self.white_pawn_en_passant,
            white_to_move: self.white_to_move,
        }
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    pub fn update_hash_on_move(
        &mut self,
        from_index: usize,
        to_index: usize,
        moved_piece: i8,
        captured_piece: i8,
    ) {
        let moved_piece_index = get_piece_index(moved_piece);

        // XOR out the old position of the moved piece
        self.hash ^= self.table[from_index][moved_piece_index];

        // XOR in the new position of the moved piece
        self.hash ^= self.table[to_index][moved_piece_index];

        // If a piece was captured, XOR it out
        if captured_piece != EMPTY_PIECE {
            let captured_piece_index = get_piece_index(captured_piece);

            self.hash ^= self.table[to_index][captured_piece_index];
        }

        self.hash ^= self.white_to_move;
    }

    pub fn update_hash_on_black_en_passant_change(&mut self) {
        self.hash ^= self.black_pawn_en_passant;
    }

    pub fn update_hash_on_white_en_passant_change(&mut self) {
        self.hash ^= self.white_pawn_en_passant;
    }

    pub fn update_hash_on_black_lose_rook_side_castle(&mut self) {
        self.hash ^= self.black_can_rook_castle;
    }

    pub fn update_hash_on_black_lose_queen_side_castle(&mut self) {
        self.hash ^= self.black_can_queen_castle;
    }

    pub fn update_hash_on_white_lose_rook_side_castle(&mut self) {
        self.hash ^= self.white_can_rook_castle;
    }

    pub fn update_hash_on_white_lose_queen_side_castle(&mut self) {
        self.hash ^= self.white_can_queen_castle;
    }

    pub fn compute_hash(&mut self, board_state: &BoardState) -> u64 {
        let mut hash = 0u64;

        if board_state.is_white_move() {
            hash ^= self.white_to_move;
        }

        if board_state.is_black_able_to_king_side_castle() {
            hash ^= self.black_can_rook_castle;
        }

        if board_state.is_black_able_to_queen_side_castle() {
            hash ^= self.black_can_queen_castle;
        }

        if board_state.get_black_en_passant() != -1 {
            hash ^= self.black_pawn_en_passant;
        }

        if board_state.is_white_able_to_king_side_castle() {
            hash ^= self.white_can_rook_castle;
        }

        if board_state.is_white_able_to_queen_side_castle() {
            hash ^= self.white_can_queen_castle;
        }

        if board_state.get_white_en_passant() != -1 {
            hash ^= self.white_pawn_en_passant;
        }

        for (i, piece_value) in board_state.get_squares().iter().enumerate() {
            if *piece_value != EMPTY_PIECE {
                hash ^= self.table[i][get_piece_index(*piece_value)];
            }
        }

        self.hash = hash;

        hash
    }
}
