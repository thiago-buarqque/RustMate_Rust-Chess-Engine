use crate::game_bit_board::{
    board::Board,
    enums::{Color, PieceType},
    positions::{same_rank, BBPositions, Squares},
    utils::{
        bitwise_utils::{east_one, north_one, pop_lsb, south_one, to_bitboard_position, west_one},
        utils::get_piece_symbol,
    },
};

use super::{
    attack_data::AttackData,
    contants::{
        BLACK_KING_RELEVANT_SQUARES_RELATED_TO_ENEMY_PAWNS, BLACK_PAWN_ATTACKS,
        WHITE_KING_RELEVANT_SQUARES_RELATED_TO_ENEMY_PAWNS, WHITE_PAWN_ATTACKS,
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

pub fn get_king_relevant_squares_related_to_enemy_pawns(color: Color, square: usize) -> u64 {
    if color.is_black() {
        BLACK_KING_RELEVANT_SQUARES_RELATED_TO_ENEMY_PAWNS[square]
    } else {
        WHITE_KING_RELEVANT_SQUARES_RELATED_TO_ENEMY_PAWNS[square]
    }
}

pub fn generate_king_relevant_squares_related_to_enemy_pawns(color: Color, inital_pos: u64) -> u64 {
    let mut result = 0;

    let mut side_squares_bb = 0;

    // West

    let mut current_pos = west_one(inital_pos);

    side_squares_bb |= current_pos;

    current_pos = west_one(current_pos);

    side_squares_bb |= current_pos | west_one(current_pos);

    // East

    current_pos = east_one(inital_pos);

    side_squares_bb |= current_pos;

    current_pos = east_one(current_pos);

    // One more to make sure to handle pawns blocking en passant
    side_squares_bb |= current_pos | east_one(current_pos);

    // Temporally append initial position
    side_squares_bb |= inital_pos;

    result |= side_squares_bb;

    let step_fn = if color.is_black() {
        south_one
    } else {
        north_one
    };

    // Get the next two rows in front of the king.

    current_pos = step_fn(side_squares_bb);

    result |= current_pos | step_fn(current_pos);

    // Remove initial square since it's where the king is
    // and there can't be a pawn there.
    result &= !inital_pos;

    // The final position is a matrix with 3 rows and 5 columns,
    // excluding the king position.
    //
    // Example
    //
    //   a b c d e f g h
    // 8 . . . . . . . . 8
    // 7 . . . . . . . . 7
    // 6 . . . . . . . . 6
    // 5 . . . . . . . . 5
    // 4 . . . . . . . . 4
    // 3 . 1 1 1 1 1 1 1 3
    // 2 . 1 1 1 1 1 1 1 2
    // 1 . 1 1 1 ♔ 1 1 1 1
    //   a b c d e f g h
    // 0x0000000000FEFEEE
    //   a b c d e f g h
    // 8 . 1 1 1 ♚ 1 1 1 8
    // 7 . 1 1 1 1 1 1 1 7
    // 6 . 1 1 1 1 1 1 1 6
    // 5 . . . . . . . . 5
    // 4 . . . . . . . . 4
    // 3 . . . . . . . . 3
    // 2 . . . . . . . . 2
    // 1 . . . . . . . . 1
    //   a b c d e f g h
    // 0xEEFEFE0000000000

    result
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        enums::{Color, PieceType},
        positions::Squares,
    };

    use super::{get_king_relevant_squares_related_to_enemy_pawns, print_board};

    #[test]
    fn test_get_king_relevant_squares_related_to_enemy_pawns() {
        let mut king_square = Squares::E1;

        let mut bb_position =
            get_king_relevant_squares_related_to_enemy_pawns(Color::White, king_square);

        print_board(
            Color::White,
            king_square as u64,
            PieceType::King,
            bb_position,
        );

        assert_eq!(0x0000000000FEFEEE, bb_position);

        king_square = Squares::E8;

        bb_position = get_king_relevant_squares_related_to_enemy_pawns(Color::Black, king_square);

        print_board(
            Color::Black,
            king_square as u64,
            PieceType::King,
            bb_position,
        );

        assert_eq!(0xEEFEFE0000000000, bb_position);
    }
}
