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
        positions::{A1, C1, D1, D5},
    };

    use super::MoveGenerator;

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
