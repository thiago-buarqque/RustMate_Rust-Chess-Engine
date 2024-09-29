use crate::game_bit_board::{
    _move::{_move::Move, move_contants::CAPTURE},
    board::Board,
    enums::{Color, PieceType},
    positions::{same_rank, BBPositions, Squares},
    utils::{
        bitwise_utils::{
            east_one, no_ea_one, no_we_one, north_one, pop_lsb, so_ea_one, so_we_one, south_one,
            to_bitboard_position, west_one,
        },
        utils::get_piece_symbol,
    },
};

use super::{
    attack_data::AttackData,
    contants::{BLACK_PAWN_ATTACKS, WHITE_PAWN_ATTACKS},
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

pub fn is_en_passant_discovered_check(
    color: Color, attack_data: &AttackData, square: usize, board: &Board,
) -> bool {
    if color != attack_data.side_to_move || !same_rank(square, attack_data.king_square) {
        return false;
    }

    let row = BBPositions::get_row_bb(attack_data.king_bb_position);

    let opponent = attack_data.side_to_move.opponent();

    let opponent_queens = board.get_piece_positions(opponent, PieceType::Queen) & row;

    let opponent_rooks = board.get_piece_positions(opponent, PieceType::Rook) & row;

    if opponent_rooks == 0 && opponent_queens == 0 {
        return false;
    }

    let friendly_pieces = board.get_player_pieces_positions(attack_data.side_to_move);
    let opponent_pieces = board.get_player_pieces_positions(opponent);
    let mut row_occupied_squares = (friendly_pieces | opponent_pieces) & row;

    // Remove friendly and enemy pawns involved in en passant
    row_occupied_squares &= !board.get_en_passant_piece_square_bb();
    row_occupied_squares &= !to_bitboard_position(square as u64);

    let mut squares_between_rook_and_king = 0;
    if opponent_rooks != 0 {
        let closest_rook_square = get_closest_square(attack_data.king_square, opponent_rooks);

        squares_between_rook_and_king =
            squares_between(attack_data.king_square, closest_rook_square);
    }

    let mut squares_between_queen_and_king = 0;
    if opponent_queens != 0 {
        let closest_queen_square = get_closest_square(attack_data.king_square, opponent_queens);

        if closest_queen_square != 0 {
            squares_between_queen_and_king =
                squares_between(attack_data.king_square, closest_queen_square);
        }
    }

    let squares_between_king = squares_between_rook_and_king | squares_between_queen_and_king;

    // After removing all those pieces, if the row is empty, it means there aren't
    // other pieces (either friendly or not) that would protect the king
    if squares_between_king & row_occupied_squares == 0 {
        return true;
    }

    false
}

pub fn squares_between(sq1: usize, sq2: usize) -> u64 {
    if sq1 < sq2 {
        ((sq1 + 1)..sq2).fold(0, |acc, sq| acc | (1 << sq))
    } else {
        ((sq2 + 1)..sq1).fold(0, |acc, sq| acc | (1 << sq))
    }
}

pub fn get_closest_square(base_square: usize, pieces: u64) -> usize {
    let mut closest_square = usize::MAX;

    let mut pieces = pieces.clone();

    while pieces != 0 {
        let square = pop_lsb(&mut pieces) as usize;

        if base_square.abs_diff(square) < closest_square {
            closest_square = square;
        }
    }

    closest_square
}

pub fn look_up_pawn_attacks(color: Color, square: usize) -> u64 {
    if color.is_white() {
        WHITE_PAWN_ATTACKS[square]
    } else {
        BLACK_PAWN_ATTACKS[square]
    }
}

pub fn is_promotion_square(color: Color, square: usize) -> bool {
    if square >= Squares::A8 && square <= Squares::H8 && color.is_white() {
        return true;
    }

    square >= Squares::A1 && square <= Squares::H1 && color.is_black()
}

// TODO: pre-calculate this for every square, as I did for sliding pieces
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

    // South one

    let _south_one = south_one(inital_pos);

    positions |= _south_one;

    // South west

    let _so_we_one = so_we_one(inital_pos);

    positions |= _so_we_one;

    let _so_we_we = west_one(_so_we_one);

    positions |= _so_we_we;

    // South east

    let _so_ea_one = so_ea_one(inital_pos);

    positions |= _so_ea_one;

    let _so_ea_ea = east_one(_so_ea_one);

    positions |= _so_ea_ea;

    // South two

    let _south_two = south_one(_south_one);

    positions |= _south_two;

    // South south west

    let _so_so_we = west_one(_south_two);

    positions |= _so_so_we;

    let _so_so_we_we = west_one(_so_so_we);

    positions |= _so_so_we_we;

    // South south east

    let _so_so_ea = east_one(_south_two);

    positions |= _so_so_ea;

    let _so_so_ea_ea = east_one(_so_so_ea);

    positions |= _so_so_ea_ea;

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
