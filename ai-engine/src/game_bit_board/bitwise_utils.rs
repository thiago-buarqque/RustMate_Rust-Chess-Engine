pub fn to_bitboard_position(square: u64) -> u64 { 1 << square }

pub fn to_decimal_position(square: u64) -> u64 { 1 >> square }

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

const A_FILE: u64 = 0x0101010101010101;
const H_FILE: u64 = 0x8080808080808080;
const FIRST_RANK: u64 = 0x00000000000000FF;
const EIGHTH_RANK: u64 = 0xFF00000000000000;
const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
const H1_A8_ANTIDIAGONAL: u64 = 0x0102040810204080;
const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

pub fn east_one(bb: u64) -> u64 { (bb << 1) & NOT_A_FILE }

pub fn no_ea_one(bb: u64) -> u64 { (bb << 9) & NOT_A_FILE }

pub fn so_ea_one(bb: u64) -> u64 { (bb >> 7) & NOT_A_FILE }

pub fn west_one(bb: u64) -> u64 { (bb >> 1) & NOT_H_FILE }

pub fn so_we_one(bb: u64) -> u64 { (bb >> 9) & NOT_H_FILE }

pub fn no_we_one(bb: u64) -> u64 { (bb << 7) & NOT_H_FILE }

pub fn south_one(bb: u64) -> u64 { bb >> 8 }

pub fn north_one(bb: u64) -> u64 { bb << 8 }

pub fn upper_bits(square: u64) -> u64 { !1 << square }

pub fn lower_bits(square: u64) -> u64 { (1 << square) - 1 }

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        bitwise_utils::{lower_bits, to_bitboard_position, to_decimal_position, upper_bits},
        positions::{A1, A8, D1, D2, D3, E1, E2, E3, F1, F2, F3, H1, H8},
    };

    use super::{
        east_one, no_ea_one, no_we_one, north_one, so_ea_one, so_we_one, south_one, west_one,
    };

    #[test]
    fn test_to_bitboard_position() {
        assert_eq!(A1, to_bitboard_position(0));
    }

    #[test]
    fn test_to_decimal_position() {
        assert_eq!(0, to_decimal_position(A1));
    }

    #[test]
    fn test_north_one() {
        assert_eq!(E3, north_one(E2));

        assert_eq!(0, north_one(H8));
    }

    #[test]
    fn test_south_one() {
        assert_eq!(E1, south_one(E2));

        assert_eq!(0, south_one(E1));
    }

    #[test]
    fn test_east_one() {
        assert_eq!(F2, east_one(E2));

        assert_eq!(0, east_one(H8));
    }

    #[test]
    fn test_west_one() {
        assert_eq!(D2, west_one(E2));

        assert_eq!(0, west_one(A8));
    }

    #[test]
    fn test_no_we_one() {
        assert_eq!(D3, no_we_one(E2));

        assert_eq!(0, no_we_one(A8));
    }

    #[test]
    fn test_so_we_one() {
        assert_eq!(D1, so_we_one(E2));

        assert_eq!(0, so_we_one(A1));
    }

    #[test]
    fn test_no_ea_one() {
        assert_eq!(F3, no_ea_one(E2));

        assert_eq!(0, no_ea_one(H8));
    }

    #[test]
    fn test_so_ea_one() {
        assert_eq!(F1, so_ea_one(E2));

        assert_eq!(0, so_ea_one(H1));
    }

    #[test]
    fn test_upper_bits() {
        assert_eq!(0xFFFFFFFFFFFF0000, upper_bits(15));
    }

    #[test]
    fn test_lower_bits() {
        assert_eq!(0x0000000000007FFF, lower_bits(15));
    }
}
