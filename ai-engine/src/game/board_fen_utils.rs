use crate::common::{
    board_utils::get_position_notation, contants::INVALID_BOARD_POSITION, enums::PieceType, fen_utils::get_piece_fen,
};

use super::board_state::BoardState;

pub fn get_position_fen(board_state: &BoardState) -> String {
    let mut fen = String::new();

    get_pieces_fen(board_state, &mut fen);

    fen.push(' ');

    get_color_fen(board_state, &mut fen);

    fen.push(' ');

    fen.push_str(&get_castle_fen(board_state));

    fen.push(' ');

    fen.push_str(&get_en_passant_fen(board_state));

    fen.push(' ');

    fen.push_str(&board_state.get_half_moves().to_string());

    fen.push(' ');

    fen.push_str(&board_state.get_full_moves().to_string());

    fen
}

fn append_empty_squares(fen: &mut String, empty_count: &mut usize) {
    if *empty_count > 0 {
        fen.push_str(&empty_count.to_string());
        *empty_count = 0;
    }
}

fn get_pieces_fen(board_state: &BoardState, fen: &mut String) {
    let mut empty_squares = 0;

    for (position, &piece) in board_state.get_squares().iter().enumerate() {
        let is_new_rank = position % 8 == 0 && position != 0;
        let is_empty_square = piece == PieceType::Empty.value();

        if is_new_rank {
            append_empty_squares(fen, &mut empty_squares);

            fen.push('/');
        }

        if is_empty_square {
            empty_squares += 1;

            continue;
        }

        append_empty_squares(fen, &mut empty_squares);

        fen.push(get_piece_fen(piece));
    }
}

fn get_color_fen(board_state: &BoardState, fen: &mut String) {
    if board_state.is_white_move() {
        fen.push('w');
    } else {
        fen.push('b');
    }
}

fn get_castle_fen(board_state: &BoardState) -> String {
    let mut castle_fen = String::new();

    if board_state.is_white_able_to_king_side_castle() {
        castle_fen.push('K');
    }

    if board_state.is_white_able_to_queen_side_castle() {
        castle_fen.push('Q');
    }

    if board_state.is_black_able_to_king_side_castle() {
        castle_fen.push('k');
    }

    if board_state.is_black_able_to_queen_side_castle() {
        castle_fen.push('q');
    }

    if castle_fen.is_empty() {
        castle_fen = String::from("-");
    }

    castle_fen
}

fn get_en_passant_fen(board_state: &BoardState) -> String {
    let black_en_passant = board_state.get_black_en_passant();
    let white_en_passant = board_state.get_white_en_passant();

    if black_en_passant != INVALID_BOARD_POSITION {
        String::from(get_position_notation(black_en_passant).as_str())
    } else if white_en_passant != INVALID_BOARD_POSITION {
        String::from(get_position_notation(white_en_passant).as_str())
    } else {
        String::from("-")
    }
}