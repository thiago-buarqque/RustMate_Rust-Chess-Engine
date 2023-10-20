use crate::{
    common::{
        board_piece::BoardPiece,
        contants::{BISHOP_WORTH, EMPTY_PIECE, KING_WORTH, PAWN_WORTH, QUEEN_WORTH, ROOK_WORTH},
        enums::PieceType,
        piece_move::PieceMove,
        piece_utils::{get_piece_type, get_piece_worth, get_promotion_options, is_white_piece},
    },
    game::{board::Board, board_state::BoardState, move_generator_helper::get_adjacent_position},
};

use super::constants::{
    BISHOP_SQUARE_TABLE, BLACK_KING_SQUARE_TABLE_END_GAME, BLACK_KING_SQUARE_TABLE_MIDDLE_GAME,
    BLACK_PAWN_SQUARE_TABLE, KNIGHT_SQUARE_TABLE, QUEEN_SQUARE_TABLE, ROOK_SQUARE_TABLE,
    WHITE_KING_SQUARE_TABLE_END_GAME, WHITE_KING_SQUARE_TABLE_MIDDLE_GAME, WHITE_PAWN_SQUARE_TABLE,
};

pub fn get_sorted_moves(board: &Board, max: bool, pieces: &[BoardPiece]) -> Vec<PieceMove> {
    let (mut moves, attacked_positions) = get_friendly_moves_and_attacked_positions(pieces, board);

    let board_state = board.get_state_reference();

    let end_game = is_end_game(pieces);

    for _move in moves.iter_mut() {
        let moving_piece = board_state.get_piece(_move.get_from_position());
        let target_piece = board_state.get_piece(_move.get_to_position());

        // Capturing move
        if _move.is_capture() {
            _move.set_move_worth(get_piece_worth(target_piece) - get_piece_worth(moving_piece));

            if get_piece_type(target_piece) == PieceType::King {
                _move.sum_to_move_worth(KING_WORTH as i32)
            }
        }

        if _move.is_promotion() {
            _move.sum_to_move_worth(_move.get_promotion_value() as i32);
        }

        // Penalize pieces from moving to a attacked position
        if attacked_positions.contains(&_move.get_to_position()) {
            _move.sum_to_move_worth(get_piece_worth(moving_piece))
        }

        _move.sum_to_move_worth(get_pst_value(
            _move.get_to_position(),
            _move.get_piece_value(),
            end_game,
            is_white_piece(_move.get_piece_value()),
        ) as i32);
    }

    // TODO order also based on the hashmap with previous generated states

    if max {
        moves.sort_by_key(|k| std::cmp::Reverse(k.get_move_worth()));
    } else {
        moves.sort_by_key(|k| k.get_move_worth());
    }

    moves
}

fn get_friendly_moves_and_attacked_positions(
    pieces: &[BoardPiece],
    board: &Board,
) -> (Vec<PieceMove>, Vec<i8>) {
    let mut moves: Vec<PieceMove> = pieces
        .iter()
        .filter(|piece| piece.is_white() == board.is_white_move())
        .flat_map(|piece| piece.get_moves_clone())
        .collect();

    let attacked_positions: Vec<i8> = pieces
        .iter()
        .filter(|piece| piece.is_white() != board.is_white_move())
        .flat_map(|piece| piece.get_moves_reference())
        .map(|_move| _move.get_to_position())
        .collect();

    let mut promotion_moves: Vec<PieceMove> = Vec::new();

    moves
        .iter()
        .filter(|_move| _move.is_promotion())
        .for_each(|_move| {
            for promotion_option in get_promotion_options(is_white_piece(_move.get_piece_value())) {
                let mut move_clone = _move.clone();

                move_clone.set_promotion_value(promotion_option);

                promotion_moves.push(move_clone)
            }
        });

    if !promotion_moves.is_empty() {
        moves.extend(promotion_moves);
        moves.retain(|_move| !_move.is_promotion());
    }

    (moves, attacked_positions)
}

fn get_pst_value(position: i8, piece_value: i8, end_game: bool, white_piece: bool) -> f32 {
    let piece_type = get_piece_type(piece_value);

    if piece_type == PieceType::Pawn {
        if white_piece {
            return WHITE_PAWN_SQUARE_TABLE[position as usize] as f32;
        }

        return BLACK_PAWN_SQUARE_TABLE[position as usize] as f32;
    } else if piece_type == PieceType::King {
        if end_game {
            return if white_piece {
                WHITE_KING_SQUARE_TABLE_END_GAME[position as usize] as f32
            } else {
                BLACK_KING_SQUARE_TABLE_END_GAME[position as usize] as f32
            };
        }

        return if white_piece {
            WHITE_KING_SQUARE_TABLE_MIDDLE_GAME[position as usize] as f32
        } else {
            BLACK_KING_SQUARE_TABLE_MIDDLE_GAME[position as usize] as f32
        };
    }

    (match piece_type {
        PieceType::Bishop => BISHOP_SQUARE_TABLE[position as usize],
        PieceType::Knight => KNIGHT_SQUARE_TABLE[position as usize],
        PieceType::Queen => QUEEN_SQUARE_TABLE[position as usize],
        PieceType::Rook => ROOK_SQUARE_TABLE[position as usize],
        _ => 0,
    }) as f32
}

fn is_end_game(pieces: &[BoardPiece]) -> bool {
    let mut black_pieces = 0;
    let mut white_pieces = 0;

    pieces.iter().for_each(|piece| {
        if piece.get_value() != EMPTY_PIECE {
            if piece.is_white() {
                white_pieces += 1
            } else {
                black_pieces += 1;
            }
        }
    });

    black_pieces <= 3 || white_pieces <= 3
}

pub fn get_board_value(board: &mut Board, max: bool, pieces: &[BoardPiece]) -> f32 {
    if board.is_game_finished() && board.get_winner_fen() == 'd' {
        // Draw
        return 0.0;
    }

    // The evaluation
    // f(p) = 200(K-K')
    //         + 9(Q-Q')
    //         + 5(R-R')
    //         + 3(B-B' + N-N')
    //         + 1(P-P')
    //         - 0.5(D-D' + S-S' + I-I')
    //         + 0.1(M-M') + ...
    //
    // ' means the opponent score
    // KQRBNP = number of kings, queens, rooks, bishops, knights and pawns
    // D,S,I = doubled, blocked and isolated pawns
    // M = Mobility (the number of legal moves)

    let mut k: f32 = 0.0;
    let mut q: f32 = 0.0;
    let mut r: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut n: f32 = 0.0;
    let mut p: f32 = 0.0;

    let mut d: f32 = 0.0;
    let mut s: f32 = 0.0;
    let mut i: f32 = 0.0;
    let mut m: f32 = 0.0;

    let board_state = board.get_state_reference();

    let mut pst_score: f32 = 0.0;

    let end_game = is_end_game(pieces);

    for piece in pieces.iter() {
        if piece.get_value() == EMPTY_PIECE {
            continue;
        }

        let pst_value = get_pst_value(
            piece.get_position(),
            piece.get_value(),
            end_game, // TODO determine endgame
            piece.is_white(),
        );

        if piece.is_white() == board.is_white_move() {
            pst_score += pst_value;
        } else {
            pst_score -= pst_value;
        }

        let factor: f32 = if piece.is_white() == board.is_white_move() {
            1.0
        } else {
            -1.0
        };

        let piece_type = get_piece_type(piece.get_value());

        match piece_type {
            PieceType::King => k += factor,
            PieceType::Queen => q += factor,
            PieceType::Rook => r += factor,
            PieceType::Bishop => b += factor,
            PieceType::Knight => n += factor,
            PieceType::Pawn => p += factor,
            // Additional cases for D, S, I, and M are handled below
            _ => (),
        }

        if piece_type == PieceType::Pawn {
            if is_doubled_pawn(board_state, piece.get_position(), piece.is_white()) {
                d += factor;
            }

            if is_blocked_pawn(board_state, piece.get_position(), piece.is_white()) {
                s += factor;
            }

            if is_isolated_pawn(board_state, piece.get_position(), piece.is_white()) {
                i += factor;
            }
        }

        for _move in piece.get_moves_reference().iter() {
            m += factor;
        }
    }

    let score = (KING_WORTH * k)
        + (QUEEN_WORTH * q)
        + (ROOK_WORTH * r)
        + (BISHOP_WORTH * (b + n))
        + (PAWN_WORTH * p)
        - ((d + s + i) / 2.0)
        + (m / 10.0)
        + pst_score;

    score * if max { 1.0 } else { -1.0 }
}

pub fn is_isolated_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
    let positions = [
        get_adjacent_position(position, position - 1),
        get_adjacent_position(position, position + 1),
        get_adjacent_position(position, position - 9),
        get_adjacent_position(position, position - 8),
        get_adjacent_position(position, position - 7),
        get_adjacent_position(position, position + 7),
        get_adjacent_position(position, position + 8),
        get_adjacent_position(position, position + 9),
    ];

    for adjacent_position in positions {
        if !board_state.is_valid_position(adjacent_position) {
            continue;
        }

        let piece = board_state.get_piece(adjacent_position);

        if piece == EMPTY_PIECE {
            continue;
        }

        if get_piece_type(piece) == PieceType::Pawn && is_white_piece(piece) == white_piece {
            return false;
        }
    }

    true
}

pub fn is_blocked_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
    let offset: i8 = if white_piece { -8 } else { 8 };

    let frontal_piece = board_state.get_piece(position + offset);

    if get_piece_type(frontal_piece) != PieceType::Empty
    // && white_piece != is_white_piece(frontal_piece)
    {
        let mut diagonal_left = 0;
        let mut diagonal_right = 0;

        if position % 8 != 0 {
            let diagonal_offset = if white_piece { -1 } else { 1 };

            diagonal_left = board_state.get_piece(position + offset + diagonal_offset);
        }

        if (position + 1) % 8 != 0 {
            let diagonal_offset = if white_piece { 1 } else { -1 };

            diagonal_right = board_state.get_piece(position + offset + diagonal_offset);
        }

        let diagonal_left_color = is_white_piece(diagonal_left);
        let diagonal_right_color = is_white_piece(diagonal_right);

        if diagonal_left == 0 && diagonal_right == 0 {
            return true;
        } else if diagonal_left != 0 && diagonal_right == 0 {
            return diagonal_left_color == white_piece;
        } else if diagonal_right != 0 && diagonal_left == 0 {
            return diagonal_right_color == white_piece;
        }

        return diagonal_left_color == white_piece && diagonal_right_color == white_piece;
    }

    false
}

pub fn is_doubled_pawn(board_state: &BoardState, mut position: i8, white_piece: bool) -> bool {
    let offset: i8 = if white_piece { -8 } else { 8 };

    position += offset;
    while board_state.is_valid_position(position) {
        let frontal_piece = board_state.get_piece(position);

        position += offset;

        if frontal_piece == EMPTY_PIECE {
            continue;
        }

        let piece_type = get_piece_type(frontal_piece);

        if piece_type != PieceType::Pawn {
            return false;
        } else if white_piece == is_white_piece(frontal_piece) {
            return true;
        }
    }

    false
}
