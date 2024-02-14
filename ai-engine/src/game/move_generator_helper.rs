use crate::common::{
    board_utils::{get_position_column, get_position_rank}, contants::{EMPTY_PIECE, INVALID_BOARD_POSITION}, piece::Piece, piece_move::PieceMove
};

use super::{board_state::BoardState, move_generator::SquareOffset};

pub fn is_pawn_first_move(white_piece: bool, piece_position: i8) -> bool {
    if white_piece && (48..=55).contains(&piece_position) {
        return true;
    }

    if !white_piece && (8..=15).contains(&piece_position) {
        return true;
    }

    false
}

pub fn position_is_not_attacked(position: i8, opponent_moves: &[PieceMove]) -> bool {
    !opponent_moves
        .iter()
        .any(|_mut| _mut.get_to_position() == position)
}

pub fn is_king_in_check(pieces: &[Piece], king_position: i8, white_move: bool) -> bool {
    for board_piece in pieces.iter() {
        if board_piece.get_value() == EMPTY_PIECE {
            continue;
        }

        if board_piece.is_white() != white_move
            && board_piece
                .get_moves_reference()
                .iter()
                .any(|m| m.get_to_position() == king_position)
        {
            return true;
        }
    }

    false
}

pub fn is_the_same_diagonal(position1: i8, position2: i8) -> bool {
    let piece1_row = get_position_rank(position1);
    let piece1_col = get_position_column(position1);

    let piece2_row = get_position_rank(position2);
    let piece2_col = get_position_column(position2);

    (piece1_row as i8 - piece2_row as i8).abs() == (piece1_col as i8 - piece2_col as i8).abs()
}

pub fn is_path_clear(board_state: &BoardState, start: i8, end: i8, step: i8) -> bool {
    let mut i = start;

    while i != end {
        if board_state.get_piece(i) != EMPTY_PIECE {
            return false;
        }
        i += step;
    }

    true
}

pub fn is_king_straight_attacked_by_sliding_piece(board_state: &BoardState, king_position: i8, piece_position: i8) -> bool {
    let piece_rank = get_position_rank(piece_position);
    let king_rank = get_position_rank(king_position);

    if piece_rank == king_rank {
        return if piece_position < king_position {
            is_path_clear(board_state, piece_position, king_position, SquareOffset::Right as i8)
        } else {
            is_path_clear(board_state, piece_position, king_position, SquareOffset::Left as i8)
        }
    } 

    let piece_column = get_position_column(piece_position);
    let king_column = get_position_column(king_position);

    if piece_column == king_column {
        return if piece_rank > king_rank {
            is_path_clear(board_state, piece_position, king_position, SquareOffset::LineBelow as i8)
        } else {
            is_path_clear(board_state, piece_position, king_position, SquareOffset::LineAbove as i8)
        };
    }

    false
}

pub fn is_diagonal_offset(offset: SquareOffset) -> bool {
    (offset == SquareOffset::TopLeft) || 
    (offset == SquareOffset::TopRight) || 
    (offset == SquareOffset::BottomLeft) || 
    (offset == SquareOffset::BottomRight)
}

pub fn get_adjacent_position(current_position: i8, new_position: i8) -> i8 {
    if !(0..=63).contains(&new_position) {
        return INVALID_BOARD_POSITION;
    }

    // Is on the left side of the board
    if current_position % 8 == 0
        && (new_position == current_position - 1 // left
            || new_position == current_position - 9 // top left
            || new_position == current_position + 7)
    // bottom left
    {
        return INVALID_BOARD_POSITION;
    }

    // Is on the right side of the board
    if (current_position + 1) % 8 == 0
        && (new_position == current_position + 1 // right
            || new_position == current_position - 7 // top right
            || new_position == current_position + 9)
    // bottom right
    {
        return INVALID_BOARD_POSITION;
    }

    new_position
}

pub fn get_knight_move(lines_apart: i8, new_position: i8, current_position: i8) -> i8 {
    if get_line_distance_between_positions(current_position, new_position) == lines_apart {
        return new_position;
    }

    INVALID_BOARD_POSITION
}

pub fn get_line_distance_between_positions(position1: i8, position2: i8) -> i8 {
    let line_start1 = position1 - (position1 % 8);
    let line_start2 = position2 - (position2 % 8);

    if line_start1 > line_start2 {
        return (line_start1 - line_start2) / 8;
    }

    (line_start2 - line_start1) / 8
}
