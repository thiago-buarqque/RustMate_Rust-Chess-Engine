pub fn to_bitboard_position(position: u64) -> u64 { 1 << position }

pub fn to_decimal_position(position: u64) -> u64 { 1 >> position }

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub fn east_one(b: u64) -> u64 { (b << 1) & NOT_A_FILE }

pub fn no_ea_one(b: u64) -> u64 { (b << 9) & NOT_A_FILE }

pub fn so_ea_one(b: u64) -> u64 { (b >> 7) & NOT_A_FILE }

pub fn west_one(b: u64) -> u64 { (b >> 1) & NOT_H_FILE }

pub fn so_we_one(b: u64) -> u64 { (b >> 9) & NOT_H_FILE }

pub fn no_we_one(b: u64) -> u64 { (b << 7) & NOT_H_FILE }

pub fn sout_one(b: u64) -> u64 { b >> 8 }

pub fn nort_one(b: u64) -> u64 { b << 8 }

pub fn upper_bits(b: u64, square: u64) -> u64 { (!b) << square }

pub fn lower_bits(b: u64, square: u64) -> u64 { (1 << square) - 1 }

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        board::Board,
        enums::{Color, PieceType},
        positions::{H2, H3, H8},
    };

    use super::nort_one;

    #[test]
    fn test_nort_one() {
        // See https://www.chessprogramming.org/Square_Mapping_Considerations
        // https://www.chessprogramming.org/General_Setwise_Operations#UpdateByMove:~:text=_mm256_srlv_epi64-,One%20Step%20Only,-The%20advantage%20with
        assert_eq!(H3, nort_one(H2));

        let board: Board = Board::new();

        let white_pawns: u64 = board.get_piece_positions(Color::White, PieceType::Pawn);

        let white_pieces: u64 = board.get_player_pieces_positions(Color::White);
        let black_pieces: u64 = board.get_player_pieces_positions(Color::Black);

        let empty_squares: u64 = !white_pieces & !black_pieces;

        let pawn_single_push_targets: u64 = nort_one(white_pawns) & empty_squares;

        assert_eq!(0x0000000000FF0000, pawn_single_push_targets);

        let pawn_double_push_targets: u64 = nort_one(pawn_single_push_targets) & empty_squares;

        assert_eq!(0x00000000FF000000, pawn_double_push_targets)
    }
}
