extern crate rand;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game_bit_board::{
    board::Board,
    enums::{Color, PieceType},
    utils::bitwise_utils::pop_lsb,
};

use super::zobrist_utils::{
    get_piece_index, BLACK_KING_SIDE_CASTLE_INDEX, BLACK_QUEEN_SIDE_CASTLE_INDEX,
    WHITE_KING_SIDE_CASTLE_INDEX, WHITE_QUEEN_SIDE_CASTLE_INDEX,
};

// https://www.chessprogramming.org/Zobrist_Hashing
//
// At program initialization, we generate an array of pseudorandom numbers:
// 1. One number for each piece at each square
// 2. One number to indicate the side to move is black
// 3. Four numbers to indicate the castling rights, though usually 16 (2^4) are
//    used for speed
// 4. Eight numbers to indicate the file of a valid En passant square, if any
//
// This leaves us with an array with 781 (12*64 + 1 + 4 + 8) random numbers.
// Since pawns don't happen on first and eighth rank, one might be fine with
// 12*64 though.

#[derive(Debug, Clone)]
pub struct Zobrist {
    board_hashes: [[u64; 64]; 12],
    castling_rights_hash: [u64; 4],
    en_passant_hash: [u64; 8],
    hash: u64,
    side_to_move_hash: u64,
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(222);

        let mut board_hashes = [[0u64; 64]; 12];

        for row in board_hashes.iter_mut() {
            for j in 0..row.len() {
                row[j] = rng.gen::<u64>();
            }
        }

        let side_to_move_hash: u64 = rng.gen::<u64>();

        // Castling rights
        let mut castling_rights_hash: [u64; 4] = [0; 4];
        for i in 0..4 {
            castling_rights_hash[i] = rng.gen::<u64>();
        }

        // En passant (8 possible files)
        let mut en_passant_hash: [u64; 8] = [0; 8];
        for i in 0..8 {
            en_passant_hash[i] = rng.gen::<u64>();
        }

        Self {
            board_hashes,
            castling_rights_hash,
            en_passant_hash,
            hash: 0,
            side_to_move_hash,
        }
    }

    pub fn get_hash(&self) -> u64 { self.hash }

    pub fn set_hash(&mut self, hash: u64) { self.hash = hash }

    pub fn update_hash_on_move(
        &mut self, captured_piece: PieceType, color: Color, from_index: usize,
        moved_piece: PieceType, to_index: usize, promotion_piece: Option<PieceType>,
    ) {
        let moved_piece_index = get_piece_index(color, moved_piece);

        // XOR out the old position of the moved piece
        self.hash ^= self.board_hashes[moved_piece_index][from_index];

        // If a piece was captured, XOR it out
        if captured_piece != PieceType::Empty {
            let captured_piece_index = get_piece_index(color.opponent(), captured_piece);

            self.hash ^= self.board_hashes[captured_piece_index][to_index];
        }

        // XOR in the new position of the moved piece
        if promotion_piece.is_some() {
            self.hash ^=
                self.board_hashes[get_piece_index(color, promotion_piece.unwrap())][to_index];
        } else {
            self.hash ^= self.board_hashes[moved_piece_index][to_index];
        }

        if color.opponent().is_black() {
            self.hash ^= self.side_to_move_hash;
        }
    }

    pub fn update_en_passant_hash(&mut self, en_passant_square_bb: u64) {
        let square = pop_lsb(&mut (en_passant_square_bb.clone())) as usize;

        let file = square % 8;

        self.hash ^= self.en_passant_hash[file];
    }

    pub fn update_hash_on_black_lose_king_side_castle(&mut self) {
        self.hash ^= self.castling_rights_hash[BLACK_KING_SIDE_CASTLE_INDEX];
    }

    pub fn update_hash_on_black_lose_queen_side_castle(&mut self) {
        self.hash ^= self.castling_rights_hash[BLACK_QUEEN_SIDE_CASTLE_INDEX];
    }

    pub fn update_hash_on_white_lose_king_side_castle(&mut self) {
        self.hash ^= self.castling_rights_hash[WHITE_KING_SIDE_CASTLE_INDEX];
    }

    pub fn update_hash_on_white_lose_queen_side_castle(&mut self) {
        self.hash ^= self.castling_rights_hash[WHITE_QUEEN_SIDE_CASTLE_INDEX];
    }

    pub fn compute_hash(&mut self, board: &Board) -> u64 {
        let mut hash = 0u64;

        if board.get_side_to_move().is_black() {
            hash ^= self.side_to_move_hash;
        }

        if board.has_king_side_castle_right(Color::Black) {
            hash ^= self.castling_rights_hash[BLACK_KING_SIDE_CASTLE_INDEX];
        }

        if board.has_queen_side_castle_right(Color::Black) {
            hash ^= self.castling_rights_hash[BLACK_QUEEN_SIDE_CASTLE_INDEX];
        }

        if board.has_king_side_castle_right(Color::White) {
            hash ^= self.castling_rights_hash[WHITE_KING_SIDE_CASTLE_INDEX];
        }

        if board.has_queen_side_castle_right(Color::White) {
            hash ^= self.castling_rights_hash[WHITE_QUEEN_SIDE_CASTLE_INDEX];
        }

        let mut en_passant_square_bb = board.get_en_passant_piece_square_bb();

        if en_passant_square_bb != 0 {
            let square = pop_lsb(&mut en_passant_square_bb) as usize;

            let file = square % 8;

            hash ^= self.en_passant_hash[file];
        }

        for square in 0..64_usize {
            let piece_type = board.get_piece_type(square);
            let piece_color = board.get_piece_color(square);

            if piece_type != PieceType::Empty {
                hash ^= self.board_hashes[get_piece_index(piece_color, piece_type)][square];
            }
        }

        self.hash = hash;

        hash
    }
}
