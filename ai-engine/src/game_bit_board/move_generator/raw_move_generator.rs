use std::collections::HashMap;

use crate::game_bit_board::utils::{
    bitwise_utils::{
        east_one, no_ea_one, no_we_one, north_one, so_ea_one, so_we_one, south_one,
        to_bitboard_position, west_one,
    },
    utils::is_pawn_in_initial_position,
};

use super::contants::{BISHOP_RELEVANT_SQUARES, ROOK_RELEVANT_SQUARES};

pub struct RawMoveGenerator {}

impl RawMoveGenerator {
    pub fn new() -> Self { RawMoveGenerator {} }

    /// Borrowed from Sebastian Lague https://youtu.be/_vqlIPDR2TU?feature=shared&t=1902
    fn get_blockers_bitboards(bb_moves: u64) -> Vec<u64> {
        let mut square_indices = Vec::new();

        for i in 0..=63 {
            if ((bb_moves >> i) & 1) == 1 {
                square_indices.push(i);
            }
        }

        let num_patterns = 1 << square_indices.len();
        let mut blockers_bitboards = Vec::with_capacity(num_patterns);

        for pattern_i in 0..num_patterns {
            blockers_bitboards.push(0);

            for bit_i in 0..square_indices.len() {
                let bit = (pattern_i >> bit_i) & 1;

                blockers_bitboards[pattern_i] |= (bit << square_indices[bit_i]) as u64;
            }
        }

        blockers_bitboards
    }

    fn create_lookup_table(
        generate: &dyn Fn(u64, u64, u64) -> u64, relevant_squares: &[u64; 64],
    ) -> HashMap<(u8, u64), u64> {
        // TODO: use the magic number approach instead of a Hash Map
        let mut lookup_table = HashMap::new();

        for square in 0..=63 {
            let position: u64 = to_bitboard_position(square);

            let moves_bb = generate(0, 0, position) & relevant_squares[square as usize];
            let blockers = RawMoveGenerator::get_blockers_bitboards(moves_bb);

            blockers.iter().for_each(|blocker_bb| {
                lookup_table.insert(
                    (square as u8, *blocker_bb),
                    generate(*blocker_bb, 0, position),
                );
            });
        }

        lookup_table
    }

    pub fn create_bishop_lookup_table() -> HashMap<(u8, u64), u64> {
        RawMoveGenerator::create_lookup_table(
            &RawMoveGenerator::generate_bishop_moves,
            &BISHOP_RELEVANT_SQUARES,
        )
    }

    pub fn create_rook_lookup_table() -> HashMap<(u8, u64), u64> {
        RawMoveGenerator::create_lookup_table(
            &RawMoveGenerator::generate_rook_moves,
            &ROOK_RELEVANT_SQUARES,
        )
    }

    fn generate_bishop_relevant_squares() -> [u64; 64] {
        let mut masks = [0u64; 64];

        for square in 0..64 {
            let rank: u8 = square / 8;
            let file: u8 = square % 8;

            let mut mask = 0u64;

            // Main diagonal (from top-left to bottom-right)
            let mut r = rank;
            let mut f = file;

            // Bottom-left direction
            while r > 1 && f > 1 {
                r -= 1;
                f -= 1;
                mask |= 1u64 << (r * 8 + f);
            }

            // Top-right direction
            r = rank;
            f = file;
            while r < 6 && f < 6 {
                r += 1;
                f += 1;
                mask |= 1u64 << (r * 8 + f);
            }

            // Anti-diagonal (from top-right to bottom-left)
            r = rank;
            f = file;

            // Top-left direction
            while r > 1 && f < 6 {
                r -= 1;
                f += 1;
                mask |= 1u64 << (r * 8 + f);
            }

            // Bottom-right direction
            r = rank;
            f = file;
            while r < 6 && f > 1 {
                r += 1;
                f -= 1;
                mask |= 1u64 << (r * 8 + f);
            }

            masks[square as usize] = mask;
        }

        masks
    }

    fn generate_rook_relevant_squares() -> [u64; 64] {
        let mut masks = [0u64; 64];

        for square in 0..64 {
            let rank = square / 8;
            let file = square % 8;

            let mut mask = 0u64;

            // Horizontal (rank) mask
            for f in 1..7 {
                // Exclude edge files
                if f != file {
                    mask |= 1u64 << (rank * 8 + f);
                }
            }

            // Vertical (file) mask
            for r in 1..7 {
                // Exclude edge ranks
                if r != rank {
                    mask |= 1u64 << (r * 8 + file);
                }
            }

            masks[square] = mask;
        }

        masks
    }

    fn generate_queen_moves(
        enemy_positions: u64, friendly_positions: u64, initial_position: u64,
    ) -> u64 {
        RawMoveGenerator::generate_bishop_moves(
            enemy_positions,
            friendly_positions,
            initial_position,
        ) | RawMoveGenerator::generate_rook_moves(
            enemy_positions,
            friendly_positions,
            initial_position,
        )
    }

    fn generate_rook_moves(
        enemy_positions: u64, friendly_positions: u64, initial_position: u64,
    ) -> u64 {
        let mut moves: u64 = 0;

        [north_one, south_one, west_one, east_one]
            .iter()
            .for_each(|step_fn| {
                RawMoveGenerator::generate_sliding_moves(
                    enemy_positions,
                    friendly_positions,
                    initial_position,
                    &mut moves,
                    step_fn,
                );
            });

        moves
    }

    fn generate_bishop_moves(
        enemy_positions: u64, friendly_positions: u64, initial_position: u64,
    ) -> u64 {
        let mut moves: u64 = 0;

        [no_we_one, so_we_one, so_ea_one, no_ea_one]
            .iter()
            .for_each(|step_fn| {
                RawMoveGenerator::generate_sliding_moves(
                    enemy_positions,
                    friendly_positions,
                    initial_position,
                    &mut moves,
                    step_fn,
                );
            });

        moves
    }

    fn generate_sliding_moves(
        enemy_positions: u64, friendly_positions: u64, initial_position: u64, moves: &mut u64,
        step_one: &dyn Fn(u64) -> u64,
    ) {
        let mut new_position = step_one(initial_position);

        while new_position != 0 && (new_position & !friendly_positions) != 0 {
            *moves |= new_position;

            // Found an enemy piece and can capture it
            if new_position & enemy_positions != 0 {
                break;
            }

            new_position = step_one(new_position);
        }
    }

    fn pre_compute_slider_moves(generate: &dyn Fn(u64, u64, u64) -> u64) -> [u64; 64] {
        let mut moves = [0; 64];

        for square in 0..=63 {
            let position = to_bitboard_position(square);

            let bb_position = generate(0, 0, position);

            moves[square as usize] = bb_position;
        }

        moves
    }

    fn pre_compute_bishop_moves() -> [u64; 64] {
        RawMoveGenerator::pre_compute_slider_moves(&RawMoveGenerator::generate_bishop_moves)
    }

    fn pre_compute_queen_moves() -> [u64; 64] {
        RawMoveGenerator::pre_compute_slider_moves(&RawMoveGenerator::generate_queen_moves)
    }

    fn pre_compute_rook_moves() -> [u64; 64] {
        RawMoveGenerator::pre_compute_slider_moves(&RawMoveGenerator::generate_rook_moves)
    }

    /// This function was used once to generate king pseudo-legal moves
    /// It will be kept here for the sake of history.
    fn generate_king_moves(friendly_positions: u64, initial_position: u64) -> u64 {
        (north_one(initial_position)
            | no_ea_one(initial_position)
            | east_one(initial_position)
            | so_ea_one(initial_position)
            | south_one(initial_position)
            | so_we_one(initial_position)
            | west_one(initial_position)
            | no_we_one(initial_position))
            & !friendly_positions
    }

    /// This function was used once to generate king pseudo-legal moves
    /// It will be kept here for the sake of history.
    fn pre_compute_king_moves(moves_vec: &mut [u64; 64]) {
        let start = 0;
        let end = 63;

        for square in start..=end {
            let position = to_bitboard_position(square);

            moves_vec[square as usize] = RawMoveGenerator::generate_king_moves(0, position);
        }
    }

    /// This function was used once to generate knight moves
    /// It will be kept here for the sake of history.
    fn pre_compute_knight_moves(moves_vec: &mut [u64; 64]) {
        let start = 0;
        let end = 63;

        for square in start..=end {
            let position = to_bitboard_position(square);

            RawMoveGenerator::get_knight_moves(
                position,
                &mut moves_vec[square as usize],
                &north_one,
            );

            RawMoveGenerator::get_knight_moves(
                position,
                &mut moves_vec[square as usize],
                &south_one,
            );
        }
    }

    /// This function was used once to generate knight moves
    /// It will be kept here for the sake of history.
    fn get_knight_moves(
        initial_position: u64, moves: &mut u64, north_or_south_one: &dyn Fn(u64) -> u64,
    ) {
        let north_or_south_8: u64 = north_or_south_one(initial_position);

        // north/south west 6
        *moves |= west_one(west_one(north_or_south_8));

        // north/south east 10
        *moves |= east_one(east_one(north_or_south_8));

        let north_16: u64 = north_or_south_one(north_or_south_8);

        // north/south west 15
        *moves |= west_one(north_16);

        // north/south east 17
        *moves |= east_one(north_16);
    }

    /// This function was used once to generate the pawn attacks
    /// It will be kept here for the sake of history.
    fn pre_compute_pawn_attacks(
        moves_vec: &mut [u64; 64], ea_one_fn: &dyn Fn(u64) -> u64, we_one_fn: &dyn Fn(u64) -> u64,
    ) {
        for square in 0..=63 {
            let position = to_bitboard_position(square);

            moves_vec[square as usize] = we_one_fn(position) | ea_one_fn(position);
        }
    }

    /// This function was used once to generate the pawn moves
    /// It will be kept here for the sake of history.
    fn pre_compute_pawn_moves(moves_vec: &mut [u64; 64], white: bool) {
        let offset_fn = if white { north_one } else { south_one };

        for square in 0..=63 {
            let position = to_bitboard_position(square);

            moves_vec[square as usize] = offset_fn(position);

            if is_pawn_in_initial_position(position, white) {
                moves_vec[square as usize] |= offset_fn(moves_vec[square as usize]);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::game_bit_board::{
        enums::{Color, PieceType},
        move_generator::{
            contants::{BISHOP_RELEVANT_SQUARES, ROOK_RELEVANT_SQUARES},
            utils::print_board,
        },
        positions::BBPositions,
        utils::utils::memory_usage_in_kb,
    };

    use super::RawMoveGenerator;

    #[test]
    fn test_create_rook_lookup_table() {
        test_create_lookup_table(
            &RawMoveGenerator::generate_rook_moves,
            PieceType::Rook,
            &ROOK_RELEVANT_SQUARES,
        )
    }

    #[test]
    fn test_create_bishop_lookup_table() {
        test_create_lookup_table(
            &RawMoveGenerator::generate_bishop_moves,
            PieceType::Bishop,
            &BISHOP_RELEVANT_SQUARES,
        )
    }

    fn test_create_lookup_table(
        generate: &dyn Fn(u64, u64, u64) -> u64, piece_type: PieceType,
        relevant_squares: &[u64; 64],
    ) {
        let table = RawMoveGenerator::create_lookup_table(generate, relevant_squares);

        println!(
            "{} look up table memory usage: {}KB\n",
            piece_type,
            memory_usage_in_kb(&table)
        );

        let mut i = 0;
        for key in table.keys() {
            println!("{:?} -> {:?}", key, table.get(key).unwrap());
            i += 1;

            if i == 3 {
                break;
            }

            print_board(
                Color::White,
                key.0.into(),
                piece_type,
                *table.get(key).unwrap(),
            );

            println!("\nBlockers:\n");

            print_board(Color::White, key.0.into(), PieceType::Empty, key.1);

            println!();
        }
    }

    #[test]
    fn test_get_blockers_bitboards() {
        let position = BBPositions::E4;

        let moves = RawMoveGenerator::generate_rook_moves(0, 0, position);

        print_board(Color::White, 28, PieceType::Rook, moves);

        let blockes = RawMoveGenerator::get_blockers_bitboards(moves);

        println!("Generated {}", blockes.len());

        print_board(
            Color::White,
            28,
            PieceType::Rook,
            *blockes.get(1256).unwrap(),
        )
    }

    #[test]
    fn test_pre_compute_bishop_moves() {
        let moves = RawMoveGenerator::pre_compute_bishop_moves();

        print_board(Color::White, 2, PieceType::Bishop, moves[2]);

        println!("{:#018X?}", moves)
    }

    #[test]
    fn test_pre_compute_queen_moves() {
        let moves = RawMoveGenerator::pre_compute_queen_moves();

        print_board(Color::White, 3, PieceType::Queen, moves[3]);

        println!("{:#018X?}", moves)
    }

    #[test]
    fn test_pre_compute_rook_moves() {
        let moves = RawMoveGenerator::pre_compute_rook_moves();

        print_board(Color::White, 0, PieceType::Rook, moves[0]);

        println!("{:#018X?}", moves)
    }

    #[test]
    fn test_generate_queen_moves() {
        let position = BBPositions::D1;

        let mut moves = RawMoveGenerator::generate_queen_moves(0, 0, position);

        println!(
            "{:?}",
            print_board(Color::Black, 3, PieceType::Queen, moves)
        );

        assert_eq!(0x08080888492A1CF7, moves);

        moves = RawMoveGenerator::generate_queen_moves(0x1020000, 0x800, position);

        println!(
            "{:?}",
            print_board(Color::Black, 3, PieceType::Queen, moves)
        );

        assert_eq!(0x00000080402214F7, moves);
    }

    #[test]
    fn test_generate_bishop_moves() {
        let position = BBPositions::C1;
        let mut moves = RawMoveGenerator::generate_bishop_moves(0, 0, position);

        println!(
            "{:?}",
            print_board(Color::Black, 2, PieceType::Bishop, moves)
        );

        assert_eq!(0x0000804020110A00, moves);

        moves = RawMoveGenerator::generate_bishop_moves(0, 0x800, position);

        println!(
            "{:?}",
            print_board(Color::Black, 2, PieceType::Bishop, moves)
        );

        assert_eq!(0x0000000000010200, moves);

        moves = RawMoveGenerator::generate_bishop_moves(0x800, 0, position);

        println!(
            "{:?}",
            print_board(Color::Black, 2, PieceType::Bishop, moves)
        );

        assert_eq!(0x0000000000010A00, moves);
    }

    #[test]
    fn test_generate_rook_moves() {
        let mut position = BBPositions::A1;

        let mut moves = RawMoveGenerator::generate_rook_moves(0, 0, position);

        println!("{:?}", print_board(Color::Black, 0, PieceType::Rook, moves));

        assert_eq!(0x01010101010101FE, moves);

        position = BBPositions::D5;

        moves = RawMoveGenerator::generate_rook_moves(0, 0xFFFF000000000000, position);

        println!(
            "{:?}",
            print_board(Color::Black, 35, PieceType::Rook, moves)
        );

        assert_eq!(0x000008F708080808, moves);

        moves =
            RawMoveGenerator::generate_rook_moves(0x0000000000FFF9F6, 0xFFFF000000000000, position);

        assert_eq!(0x000008F708080000, moves);

        println!(
            "{:?}",
            print_board(Color::Black, 35, PieceType::Rook, moves)
        );
    }
}
