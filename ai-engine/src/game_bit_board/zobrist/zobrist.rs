extern crate rand;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game_bit_board::{
    board::Board,
    enums::{Color, PieceType},
    positions::BBPositions,
};

use super::zobrist_utils::get_piece_index;

#[derive(Debug, Clone)]
pub struct Zobrist {
    black_can_king_castle: u64,
    black_can_queen_castle: u64,
    black_pawn_en_passant: u64,
    hash: u64,
    table: Vec<Vec<u64>>,
    white_can_king_castle: u64,
    white_can_queen_castle: u64,
    white_pawn_en_passant: u64,
    white_to_move: u64,
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(222);

        let mut table = vec![vec![0u64; 12]; 64];

        for row in table.iter_mut() {
            for j in 0..row.len() {
                row[j] = rng.gen::<u64>();
            }
        }

        let black_can_king_castle = rng.gen::<u64>();
        let black_can_queen_castle = rng.gen::<u64>();
        let black_pawn_en_passant = rng.gen::<u64>();

        let white_can_king_castle = rng.gen::<u64>();
        let white_can_queen_castle = rng.gen::<u64>();
        let white_pawn_en_passant = rng.gen::<u64>();
        let white_to_move = rng.gen::<u64>();

        Self {
            black_can_king_castle,
            black_can_queen_castle,
            black_pawn_en_passant,
            hash: 0,
            table,
            white_can_king_castle,
            white_can_queen_castle,
            white_pawn_en_passant,
            white_to_move,
        }
    }

    pub fn get_hash(&self) -> u64 { self.hash }

    pub fn update_hash_on_move(
        &mut self, captured_piece: PieceType, color: Color, from_index: usize,
        moved_piece: PieceType, to_index: usize,
    ) {
        let moved_piece_index = get_piece_index(color, moved_piece);

        // XOR out the old position of the moved piece
        self.hash ^= self.table[from_index][moved_piece_index];

        // XOR in the new position of the moved piece
        self.hash ^= self.table[to_index][moved_piece_index];

        // If a piece was captured, XOR it out
        if captured_piece != PieceType::Empty {
            let captured_piece_index = get_piece_index(color.opponent(), captured_piece);

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

    pub fn update_hash_on_black_lose_king_side_castle(&mut self) {
        self.hash ^= self.black_can_king_castle;
    }

    pub fn update_hash_on_black_lose_queen_side_castle(&mut self) {
        self.hash ^= self.black_can_queen_castle;
    }

    pub fn update_hash_on_white_lose_king_side_castle(&mut self) {
        self.hash ^= self.white_can_king_castle;
    }

    pub fn update_hash_on_white_lose_queen_side_castle(&mut self) {
        self.hash ^= self.white_can_queen_castle;
    }

    pub fn compute_hash(&mut self, board: &Board) -> u64 {
        let mut hash = 0u64;

        if board.get_side_to_move().is_white() {
            hash ^= self.white_to_move;
        }

        if board.has_king_side_castle_right(Color::Black) {
            hash ^= self.black_can_king_castle;
        }

        if board.has_queen_side_castle_right(Color::Black) {
            hash ^= self.black_can_queen_castle;
        }

        let en_passant_bb_position = board.get_en_passant();

        if BBPositions::is_en_passant_position(Color::Black, en_passant_bb_position) {
            hash ^= self.black_pawn_en_passant;
        }

        if BBPositions::is_en_passant_position(Color::White, en_passant_bb_position) {
            hash ^= self.white_pawn_en_passant;
        }

        if board.has_king_side_castle_right(Color::White) {
            hash ^= self.white_can_king_castle;
        }

        if board.has_queen_side_castle_right(Color::White) {
            hash ^= self.white_can_queen_castle;
        }

        for square in 0..64_usize {
            let piece_type = board.get_piece_type(square);
            let piece_color = board.get_piece_color(square);

            if piece_type != PieceType::Empty {
                hash ^= self.table[square][get_piece_index(piece_color, piece_type)];
            }
        }

        self.hash = hash;

        hash
    }
}
