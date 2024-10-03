pub fn get_direction_fn_to_square(from: usize, to: usize) -> fn(u64) -> u64 {
    let from_rank = from / 8;
    let from_file = from % 8;
    let to_rank = to / 8;
    let to_file = to % 8;

    let rank_diff = to_rank as isize - from_rank as isize;
    let file_diff = to_file as isize - from_file as isize;

    match (rank_diff, file_diff) {
        (0, f) if f > 0 => east_one,
        (0, f) if f < 0 => west_one,
        (r, 0) if r > 0 => north_one,
        (r, 0) if r < 0 => south_one,
        (r, f) if r > 0 && f > 0 => no_ea_one,
        (r, f) if r > 0 && f < 0 => no_we_one,
        (r, f) if r < 0 && f > 0 => so_ea_one,
        (r, f) if r < 0 && f < 0 => so_we_one,
        _ => |bb| bb,
    }
}

#[inline(always)]
pub fn to_bitboard_position(square: u64) -> u64 { 1 << square }

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

#[inline(always)]
pub fn east_one(bb: u64) -> u64 { (bb << 1) & NOT_A_FILE }

#[inline(always)]
pub fn no_ea_one(bb: u64) -> u64 { (bb << 9) & NOT_A_FILE }

#[inline(always)]
pub fn so_ea_one(bb: u64) -> u64 { (bb >> 7) & NOT_A_FILE }

#[inline(always)]
pub fn west_one(bb: u64) -> u64 { (bb >> 1) & NOT_H_FILE }

#[inline(always)]
pub fn so_we_one(bb: u64) -> u64 { (bb >> 9) & NOT_H_FILE }

#[inline(always)]
pub fn no_we_one(bb: u64) -> u64 { (bb << 7) & NOT_H_FILE }

#[inline(always)]
pub fn south_one(bb: u64) -> u64 { bb >> 8 }

#[inline(always)]
pub fn north_one(bb: u64) -> u64 { bb << 8 }

#[inline(always)]
pub fn upper_bits(square: u64) -> u64 { !1 << square }

#[inline(always)]
pub fn lower_bits(square: u64) -> u64 { (1 << square) - 1 }

pub fn get_direction_name(f: fn(u64) -> u64) -> &'static str {
    let f = f as fn(u64) -> u64;

    if f == east_one as fn(u64) -> u64 {
        "east"
    } else if f == no_ea_one as fn(u64) -> u64 {
        "north east"
    } else if f == so_ea_one as fn(u64) -> u64 {
        "so east"
    } else if f == west_one as fn(u64) -> u64 {
        "west"
    } else if f == so_we_one as fn(u64) -> u64 {
        "so west"
    } else if f == no_we_one as fn(u64) -> u64 {
        "north west"
    } else if f == south_one as fn(u64) -> u64 {
        "south"
    } else if f == north_one as fn(u64) -> u64 {
        "north"
    } else {
        "unkwnown direction"
    }
}

// Boworred from someone on StackOverflow, I lost the link
static DEBRUIJ_T: &'static [u8] = &[
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

const DEBRUIJ_M: u64 = 0x03f7_9d71_b4cb_0a89;

#[inline(always)]
pub fn pop_lsb(bits: &mut u64) -> u8 {
    let bitss = *bits;
    *bits &= *bits - 1;
    DEBRUIJ_T[(((bitss ^ bitss.wrapping_sub(1)).wrapping_mul(DEBRUIJ_M)).wrapping_shr(58)) as usize]
}

// pub fn pop_lsb(bitboard: &mut u64) -> u8 {
//     if *bitboard == 0 {
//         u8::MAX
//     } else {
//         let lsb_index = bitboard.trailing_zeros();
//         *bitboard &= *bitboard - 1;

//         lsb_index as u8
//     }
// }

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        positions::BBPositions,
        utils::bitwise_utils::{lower_bits, to_bitboard_position, upper_bits},
    };

    use super::{
        east_one, no_ea_one, no_we_one, north_one, so_ea_one, so_we_one, south_one, west_one,
    };

    #[test]
    fn test_to_bitboard_position() {
        assert_eq!(BBPositions::A1, to_bitboard_position(0));
    }

    #[test]
    fn test_north_one() {
        assert_eq!(BBPositions::E3, north_one(BBPositions::E2));

        assert_eq!(0, north_one(BBPositions::H8));
    }

    #[test]
    fn test_south_one() {
        assert_eq!(BBPositions::E1, south_one(BBPositions::E2));

        assert_eq!(0, south_one(BBPositions::E1));
    }

    #[test]
    fn test_east_one() {
        assert_eq!(BBPositions::F2, east_one(BBPositions::E2));

        assert_eq!(0, east_one(BBPositions::H8));
    }

    #[test]
    fn test_west_one() {
        assert_eq!(BBPositions::D2, west_one(BBPositions::E2));

        assert_eq!(0, west_one(BBPositions::A8));
    }

    #[test]
    fn test_no_we_one() {
        assert_eq!(BBPositions::D3, no_we_one(BBPositions::E2));

        assert_eq!(0, no_we_one(BBPositions::A8));
    }

    #[test]
    fn test_so_we_one() {
        assert_eq!(BBPositions::D1, so_we_one(BBPositions::E2));

        assert_eq!(0, so_we_one(BBPositions::A1));
    }

    #[test]
    fn test_no_ea_one() {
        assert_eq!(BBPositions::F3, no_ea_one(BBPositions::E2));

        assert_eq!(0, no_ea_one(BBPositions::H8));
    }

    #[test]
    fn test_so_ea_one() {
        assert_eq!(BBPositions::F1, so_ea_one(BBPositions::E2));

        assert_eq!(0, so_ea_one(BBPositions::H1));
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
