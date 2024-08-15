use crate::game_bit_board::{bitwise_utils::{east_one, west_one}, utils::{get_piece_symbol, is_pawn_in_initial_position}};

use super::super::{bitwise_utils::{no_ea_one, no_we_one, north_one, so_ea_one, so_we_one, south_one, to_bitboard_position}, enums::{Color, PieceType}};

struct MoveGenerator { }

impl MoveGenerator {
    pub fn new() -> Self {
        MoveGenerator { }
    }

    /// This function was used once to generate knight moves
    /// It will be kept here for the sake of history.
    fn pre_compute_knight_moves(
        moves_vec: &mut [u64; 64]
    ) {
        let start = 0;
        let end = 63;

        for square in start..=end {
            let position = to_bitboard_position(square);

            MoveGenerator::get_knight_moves(
                position,
                &mut moves_vec[square as usize],
                &north_one
            );

            MoveGenerator::get_knight_moves(
                position,
                &mut moves_vec[square as usize],
                &south_one
            );
        }
    }

    /// This function was used once to generate knight moves
    /// It will be kept here for the sake of history.
    fn get_knight_moves(initial_position: u64, moves: &mut u64, north_or_south_one: &dyn Fn(u64) -> u64) {
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

    /// This function was used once to generate the pawn moves
    /// It will be kept here for the sake of history.
    fn pre_compute_pawn_moves(
        moves_vec: &mut [u64; 64],
        ea_one_fn: &dyn Fn(u64) -> u64,
        step_one_fn: &dyn Fn(u64) -> u64,
        we_one_fn: &dyn Fn(u64) -> u64,
        white: bool
    ) {
        let start = 0;
        let end = 63;

        for square in start..=end {
            let position = to_bitboard_position(square);

            let mut moves: u64 = step_one_fn(position);

            if is_pawn_in_initial_position(position, white) {
                let two_squares_move = step_one_fn(moves);

                moves = two_squares_move | moves;
            }

            let no_we_move = we_one_fn(position);
            let no_ea_move = ea_one_fn(position);

            moves = moves | no_we_move | no_ea_move;
        
            moves_vec[square as usize] = moves;
        }
    }
}

fn print_board(color: Color, piece_index: u64, piece_type: PieceType, bb_position: u64) {
    println!("  a b c d e f g h");
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = 1 << (rank * 8 + file);
            if square == 1 << piece_index {
                let symbol = get_piece_symbol(color, piece_type);
                print!("{symbol} ");
            }
            else if bb_position & square != 0 {
                print!("1 ");
            } else {
                print!(". ");
            }
        }
        println!("{}", rank + 1);
    }
    println!("  a b c d e f g h");
}


#[cfg(test)]
mod tests {

    use crate::game_bit_board::{enums::{Color, PieceType}, move_generator::move_generator::print_board};

    use super::MoveGenerator;

    #[test]
    fn test_pre_compute_pawn_moves() {
        let mut moves = [0; 64];

        MoveGenerator::pre_compute_knight_moves(&mut moves);

        println!("{:#018X?}", moves);

        for i in 0..64 {
            let _moves = moves[i];

            println!(
            "{:?}",
            print_board(Color::Black, i as u64, PieceType::Knight, _moves));
        }
    }
}