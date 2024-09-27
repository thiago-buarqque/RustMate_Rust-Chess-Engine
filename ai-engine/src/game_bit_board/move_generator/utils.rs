use crate::game_bit_board::{enums::{Color, PieceType}, utils::{bitwise_utils::{east_one, no_ea_one, no_we_one, north_one, so_ea_one, so_we_one, south_one, west_one}, utils::get_piece_symbol}};

pub fn print_board(color: Color, piece_square: u64, piece_type: PieceType, bb_position: u64) {
    println!("  a b c d e f g h");
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);

        for file in 0..8 {
            let square = rank * 8 + file;
            let position = 1 << square;

            if square == piece_square {
                let symbol = get_piece_symbol(color, piece_type);

                print!("{symbol} ");
            } else if bb_position & position != 0 {
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

pub fn get_king_attacker_positions(bb_position: u64) -> u64 {
    let mut positions = 0_u64;

    [north_one, south_one, west_one, east_one, 
    no_ea_one, no_we_one, so_ea_one, so_we_one].iter().for_each(|f| {
        let mut pos = f(bb_position);

        while pos != 0 {
            positions |= pos;

            pos = f(pos);
        }
    });

    positions
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{enums::{Color, PieceType}, positions::Squares, utils::bitwise_utils::to_bitboard_position};

    use super::{get_king_attacker_positions, print_board};


    #[test]
    pub fn test_get_king_possible_attackers_positions() {
        let positions = get_king_attacker_positions(to_bitboard_position(Squares::D5 as u64));

        print_board(Color::White, Squares::D5 as u64, PieceType::King, positions);

        assert_eq!(0x492A1CF71C2A4988, positions)
    }

}