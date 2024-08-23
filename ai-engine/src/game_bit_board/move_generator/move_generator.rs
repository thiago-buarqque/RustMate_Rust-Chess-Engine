use std::collections::HashMap;

use crate::game_bit_board::{
    bitwise_utils::{east_one, west_one},
    utils::{get_piece_symbol, is_pawn_in_initial_position},
};

use super::super::{
    bitwise_utils::*,
    enums::{Color, PieceType},
};

struct MoveGenerator {}

impl MoveGenerator {
    pub fn new() -> Self { MoveGenerator {} }

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

    fn create_lookup_table(generate: &dyn Fn(u64, u64, u64) -> u64) -> HashMap<(u8, u64), u64> {
        // TODO: use the magic number approach instead of a Hash Map
        let mut lookup_table = HashMap::new();

        for i in 0..=63 {
            let position: u64 = to_bitboard_position(i);

            let moves_bb = generate(0, 0, position);
            let blockers = MoveGenerator::get_blockers_bitboards(moves_bb);

            blockers.iter().for_each(|blocker_bb| {
                lookup_table.insert((i as u8, *blocker_bb), generate(*blocker_bb, 0, position));
            });
        }

        lookup_table
    }

    fn generate_queen_moves(
        enemy_positions: u64, friendly_positions: u64, initial_position: u64,
    ) -> u64 {
        MoveGenerator::generate_bishop_moves(enemy_positions, friendly_positions, initial_position)
            | MoveGenerator::generate_rook_moves(
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
                MoveGenerator::generate_sliding_moves(
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
                MoveGenerator::generate_sliding_moves(
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
        MoveGenerator::pre_compute_slider_moves(&MoveGenerator::generate_bishop_moves)
    }

    fn pre_compute_queen_moves() -> [u64; 64] {
        MoveGenerator::pre_compute_slider_moves(&MoveGenerator::generate_queen_moves)
    }

    fn pre_compute_rook_moves() -> [u64; 64] {
        MoveGenerator::pre_compute_slider_moves(&MoveGenerator::generate_rook_moves)
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

            moves_vec[square as usize] = MoveGenerator::generate_king_moves(0, position);
        }
    }

    /// This function was used once to generate knight moves
    /// It will be kept here for the sake of history.
    fn pre_compute_knight_moves(moves_vec: &mut [u64; 64]) {
        let start = 0;
        let end = 63;

        for square in start..=end {
            let position = to_bitboard_position(square);

            MoveGenerator::get_knight_moves(position, &mut moves_vec[square as usize], &north_one);

            MoveGenerator::get_knight_moves(position, &mut moves_vec[square as usize], &south_one);
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

fn print_board(color: Color, piece_square: u64, piece_type: PieceType, bb_position: u64) {
    println!("  a b c d e f g h");
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);

        for file in 0..8 {
            let square = 1 << (rank * 8 + file);

            if square == 1 << piece_square {
                let symbol = get_piece_symbol(color, piece_type);

                print!("{symbol} ");
            } else if bb_position & square != 0 {
                print!("1 ");
            } else {
                print!(". ");
            }
        }

        println!("{}", rank + 1);
    }

    println!("  a b c d e f g h");
    println!("{:#018X}", bb_position)
}

#[cfg(test)]
mod tests {

    use crate::game_bit_board::{
        enums::{Color, PieceType},
        move_generator::move_generator::print_board,
        positions::{A1, C1, D1, D5, E4},
        utils::memory_usage_in_kb,
    };

    use super::MoveGenerator;

    #[test]
    fn test_create_rook_lookup_table() {
        test_create_lookup_table(&MoveGenerator::generate_rook_moves, PieceType::Rook)
    }

    #[test]
    fn test_create_bishop_lookup_table() {
        test_create_lookup_table(&MoveGenerator::generate_bishop_moves, PieceType::Bishop)
    }

    fn test_create_lookup_table(generate: &dyn Fn(u64, u64, u64) -> u64, piece_type: PieceType) {
        let table = MoveGenerator::create_lookup_table(generate);

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
        let position = E4;

        let moves = MoveGenerator::generate_rook_moves(0, 0, position);

        print_board(Color::White, 28, PieceType::Rook, moves);

        let blockes = MoveGenerator::get_blockers_bitboards(moves);

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
        let moves = MoveGenerator::pre_compute_bishop_moves();

        print_board(Color::White, 2, PieceType::Bishop, moves[2]);

        println!("{:#018X?}", moves)
    }

    #[test]
    fn test_pre_compute_queen_moves() {
        let moves = MoveGenerator::pre_compute_queen_moves();

        print_board(Color::White, 3, PieceType::Queen, moves[3]);

        println!("{:#018X?}", moves)
    }

    #[test]
    fn test_pre_compute_rook_moves() {
        let moves = MoveGenerator::pre_compute_rook_moves();

        print_board(Color::White, 0, PieceType::Rook, moves[0]);

        println!("{:#018X?}", moves)
    }

    #[test]
    fn test_generate_queen_moves() {
        let position = D1;

        let mut moves = MoveGenerator::generate_queen_moves(0, 0, position);

        println!(
            "{:?}",
            print_board(Color::Black, 3, PieceType::Queen, moves)
        );

        assert_eq!(0x08080888492A1CF7, moves);

        moves = MoveGenerator::generate_queen_moves(0x1020000, 0x800, position);

        println!(
            "{:?}",
            print_board(Color::Black, 3, PieceType::Queen, moves)
        );

        assert_eq!(0x00000080402214F7, moves);
    }

    #[test]
    fn test_generate_bishop_moves() {
        let position = C1;
        let mut moves = MoveGenerator::generate_bishop_moves(0, 0, position);

        println!(
            "{:?}",
            print_board(Color::Black, 2, PieceType::Bishop, moves)
        );

        assert_eq!(0x0000804020110A00, moves);

        moves = MoveGenerator::generate_bishop_moves(0, 0x800, position);

        println!(
            "{:?}",
            print_board(Color::Black, 2, PieceType::Bishop, moves)
        );

        assert_eq!(0x0000000000010200, moves);

        moves = MoveGenerator::generate_bishop_moves(0x800, 0, position);

        println!(
            "{:?}",
            print_board(Color::Black, 2, PieceType::Bishop, moves)
        );

        assert_eq!(0x0000000000010A00, moves);
    }

    #[test]
    fn test_generate_rook_moves() {
        let mut position = A1;

        let mut moves = MoveGenerator::generate_rook_moves(0, 0, position);

        println!("{:?}", print_board(Color::Black, 0, PieceType::Rook, moves));

        assert_eq!(0x01010101010101FE, moves);

        position = D5;

        moves = MoveGenerator::generate_rook_moves(0, 0xFFFF000000000000, position);

        println!(
            "{:?}",
            print_board(Color::Black, 35, PieceType::Rook, moves)
        );

        assert_eq!(0x000008F708080808, moves);

        moves =
            MoveGenerator::generate_rook_moves(0x0000000000FFF9F6, 0xFFFF000000000000, position);

        assert_eq!(0x000008F708080000, moves);

        println!(
            "{:?}",
            print_board(Color::Black, 35, PieceType::Rook, moves)
        );
    }
}
