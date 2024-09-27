use crate::game_bit_board::{
    _move::{_move::Move, move_contants::CAPTURE},
    enums::{Color, PieceType},
    utils::{
        bitwise_utils::{
            east_one, no_ea_one, no_we_one, north_one, pop_lsb,
            to_bitboard_position, west_one,
        },
        utils::get_piece_symbol,
    },
};

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

pub fn create_moves(
    mut attacks: u64, opponent_pieces_bb: u64, moves: &mut Vec<Move>, square: usize, color: Color,
    piece_type: PieceType,
) {
    while attacks != 0 {
        let target_square = pop_lsb(&mut attacks);

        let mut flags: u16 = 0;
        if to_bitboard_position(target_square as u64) & opponent_pieces_bb != 0 {
            flags = CAPTURE;
        }

        moves.push(Move::with_flags(
            flags,
            square,
            target_square as usize,
            color,
            piece_type,
        ));
    }
}

pub fn get_king_relevant_squares_related_to_enemy_pawns(inital_pos: u64) -> u64 {
    let mut positions = 0;

    let _west_one = west_one(inital_pos);

    positions |= _west_one;

    let west_two = west_one(_west_one);

    positions |= west_two;

    // East

    let _east_one = east_one(inital_pos);

    positions |= _east_one;

    let east_two = east_one(_east_one);

    positions |= east_two;

    // North one

    let _north_one = north_one(inital_pos);

    positions |= _north_one;

    // North west

    let _no_we_one = no_we_one(inital_pos);

    positions |= _no_we_one;

    let _no_we_we = west_one(_no_we_one);

    positions |= _no_we_we;

    // North east

    let _no_ea_one = no_ea_one(inital_pos);

    positions |= _no_ea_one;

    let _no_ea_ea = east_one(_no_ea_one);

    positions |= _no_ea_ea;

    // North two

    let _north_two = north_one(_north_one);

    positions |= _north_two;

    // North north west

    let _no_no_we = west_one(_north_two);

    positions |= _no_no_we;

    let _no_no_we_we = west_one(_no_no_we);

    positions |= _no_no_we_we;

    // North north east

    let _no_no_ea = east_one(_north_two);

    positions |= _no_no_ea;

    let _no_no_ea_ea = east_one(_no_no_ea);

    positions |= _no_no_ea_ea;

    positions
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        enums::{Color, PieceType},
        positions::{BBPositions, Squares},
    };

    use super::{get_king_relevant_squares_related_to_enemy_pawns, print_board};

    #[test]
    fn test_get_king_relevant_squares_related_to_enemy_pawns() {
        print_board(
            Color::White,
            Squares::E1 as u64,
            PieceType::King,
            get_king_relevant_squares_related_to_enemy_pawns(BBPositions::E1),
        );
    }
}
