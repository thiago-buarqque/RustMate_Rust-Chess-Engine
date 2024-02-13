use crate::{
    common::{
        piece::Piece,
        board_utils::{get_position_column_number, get_position_line_number},
        contants::{BISHOP_WORTH, EMPTY_PIECE, KING_WORTH, PAWN_WORTH, QUEEN_WORTH, ROOK_WORTH},
        enums::PieceType,
        piece_move::PieceMove,
        piece_utils::{get_piece_type, get_piece_worth, get_promotion_options, is_white_piece},
    },
    game::{board::Board, board_state::BoardState, move_generator_helper::get_adjacent_position},
};

use super::constants::{
    BLACK_BISHOP_SQUARE_TABLE, BLACK_KING_SQUARE_TABLE_END_GAME,
    BLACK_KING_SQUARE_TABLE_MIDDLE_GAME, BLACK_KNIGHT_SQUARE_TABLE, BLACK_PAWN_SQUARE_TABLE,
    BLACK_ROOK_SQUARE_TABLE, END_GAME_PIECES_THRESHOLD, QUEEN_SQUARE_TABLE,
    WHITE_BISHOP_SQUARE_TABLE, WHITE_KING_SQUARE_TABLE_END_GAME,
    WHITE_KING_SQUARE_TABLE_MIDDLE_GAME, WHITE_KNIGHT_SQUARE_TABLE, WHITE_PAWN_SQUARE_TABLE,
    WHITE_ROOK_SQUARE_TABLE,
};

pub fn get_sorted_moves(
    best_move: &Option<PieceMove>,
    board: &Board,
    max: bool,
    pieces: &[Piece],
) -> Vec<PieceMove> {
    let (mut moves, attacked_positions) = get_friendly_moves_and_attacked_positions(pieces, board);

    let board_state = board.get_state_reference();

    let end_game = is_end_game(pieces);

    moves.iter_mut().for_each(|_move| {
        let moving_piece = _move.get_piece_value();

        if _move.is_capture() {
            let target_piece = board_state.get_piece(_move.get_to_position());

            _move.sum_to_move_worth(
                (5 * get_piece_worth(target_piece)) - get_piece_worth(moving_piece),
            );
        }

        if _move.is_promotion() {
            _move.sum_to_move_worth(_move.get_promotion_value() as i32);
        }

        if attacked_positions.contains(&_move.get_to_position()) {
            _move.sum_to_move_worth(-get_piece_worth(moving_piece))
        }

        _move.sum_to_move_worth(get_position_value(
            _move.get_to_position(),
            _move.get_piece_value(),
            end_game,
            is_white_piece(_move.get_piece_value()),
        ) as i32);

        if end_game && get_piece_type(moving_piece) == PieceType::King {
            _move.sum_to_move_worth(get_end_game_move_worth(board.clone(), max, _move));
        }
    });

    // TODO order also based on the hashmap with previous generated states

    if max {
        moves.sort_by_key(|k| std::cmp::Reverse(k.get_move_worth()));
    } else {
        moves.sort_by_key(|k| k.get_move_worth());
    }

    if best_move.is_some() {
        let best_move = best_move.clone().unwrap();

        for (i, _move) in moves.iter().enumerate() {
            if _move.eq(&best_move) {
                moves.remove(i);
                moves.insert(0, best_move);

                break;
            }
        }
    }

    moves
}

fn get_end_game_move_worth(mut board: Board, max: bool, piece_move: &PieceMove) -> i32 {
    let _ = board.move_piece(piece_move);

    let state_reference = &board.get_state_reference();

    let (friendly_king_position, opponent_king_position) =
        if is_white_piece(piece_move.get_piece_value()) {
            (
                state_reference.get_white_king_position(),
                state_reference.get_black_king_position(),
            )
        } else {
            (
                state_reference.get_black_king_position(),
                state_reference.get_white_king_position(),
            )
        };

    let mut evaluation = 0.0_f32;

    let opponent_king_rank = get_position_line_number(opponent_king_position) as f32;
    let opponent_king_file = get_position_column_number(opponent_king_position) as f32;

    let opponent_king_dst_to_center_file = (3.0 - opponent_king_file).max(opponent_king_file - 4.0);
    let opponent_king_dst_to_center_rank = (3.0 - opponent_king_rank).max(opponent_king_rank - 4.0);

    let opponent_king_dst_from_center =
        opponent_king_dst_to_center_file + opponent_king_dst_to_center_rank;

    evaluation += opponent_king_dst_from_center as f32;

    let friendly_king_rank = get_position_line_number(friendly_king_position) as f32;
    let friendly_king_file = get_position_column_number(friendly_king_position) as f32;

    let dst_between_kings_file = (friendly_king_file - opponent_king_file).abs();
    let dst_between_kings_rank = (friendly_king_rank - opponent_king_rank).abs();

    let dst_between_kings = dst_between_kings_file + dst_between_kings_rank;

    evaluation += 14.0 - (dst_between_kings as f32);

    return (evaluation
        * 10.0
        * calculate_end_game_weight(&board.get_pieces())
        * if max { 1.0 } else { -1.0 }) as i32;
}

fn get_friendly_moves_and_attacked_positions(
    pieces: &[Piece],
    board: &Board,
) -> (Vec<PieceMove>, Vec<i8>) {
    let mut moves: Vec<PieceMove> = pieces
        .iter()
        .filter(|piece| piece.is_white() == board.is_white_move())
        .flat_map(|piece| piece.get_moves_clone())
        .collect();

    let attacked_positions: Vec<i8> = pieces
        .iter()
        .filter(|piece| {
            piece.is_white() != board.is_white_move()
                && get_piece_type(piece.get_value()) != PieceType::Pawn
        })
        .flat_map(|piece| piece.get_moves_reference())
        .map(|_move| _move.get_to_position())
        .collect();

    let mut promotion_moves: Vec<PieceMove> = Vec::with_capacity(4);

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
        moves.retain(|_move| !_move.is_promotion());
        moves.extend(promotion_moves);
    }

    (moves, attacked_positions)
}

fn get_position_value(position: i8, piece_value: u8, end_game: bool, white_piece: bool) -> f32 {
    let piece_type = get_piece_type(piece_value);

    if piece_type == PieceType::Pawn {
        if white_piece {
            return WHITE_PAWN_SQUARE_TABLE[position as usize];
        }

        return BLACK_PAWN_SQUARE_TABLE[position as usize];
    } else if piece_type == PieceType::King {
        if end_game {
            return if white_piece {
                WHITE_KING_SQUARE_TABLE_END_GAME[position as usize]
            } else {
                BLACK_KING_SQUARE_TABLE_END_GAME[position as usize]
            };
        }

        return if white_piece {
            WHITE_KING_SQUARE_TABLE_MIDDLE_GAME[position as usize]
        } else {
            BLACK_KING_SQUARE_TABLE_MIDDLE_GAME[position as usize]
        };
    }

    if white_piece {
        match piece_type {
            PieceType::Bishop => WHITE_BISHOP_SQUARE_TABLE[position as usize],
            PieceType::Knight => WHITE_KNIGHT_SQUARE_TABLE[position as usize],
            PieceType::Queen => QUEEN_SQUARE_TABLE[position as usize],
            PieceType::Rook => WHITE_ROOK_SQUARE_TABLE[position as usize],
            _ => 0.0,
        }
    } else {
        match piece_type {
            PieceType::Bishop => BLACK_BISHOP_SQUARE_TABLE[position as usize],
            PieceType::Knight => BLACK_KNIGHT_SQUARE_TABLE[position as usize],
            PieceType::Queen => QUEEN_SQUARE_TABLE[position as usize],
            PieceType::Rook => BLACK_ROOK_SQUARE_TABLE[position as usize],
            _ => 0.0,
        }
    }
}

fn is_end_game(pieces: &[Piece]) -> bool {
    calculate_end_game_weight(pieces) >= 1.0
}

fn calculate_end_game_weight(pieces: &[Piece]) -> f32 {
    let mut black_pieces: f32 = 0.0;
    let mut white_pieces: f32 = 0.0;

    pieces.iter().for_each(|piece| {
        if piece.get_value() != EMPTY_PIECE {
            if piece.is_white() {
                white_pieces += 1.0
            } else {
                black_pieces += 1.0;
            }
        }
    });

    (END_GAME_PIECES_THRESHOLD / black_pieces)
        .max(END_GAME_PIECES_THRESHOLD / white_pieces)
}

pub fn get_board_value(board: &mut Board, max: bool, pieces: &[Piece]) -> f32 {
    if board.is_game_finished() && board.get_winner_fen() == 'd' {
        // Draw
        return 0.0;
    } 
    
    if board.is_game_finished() {
        return KING_WORTH * 10.0 * if max {
            1.0
        } else {
            -1.0
        };
    }

    // The evaluation
    // f(p) = 200(K-K') -> always 0 since the two kings are always present
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

    // let mut k: f32 = 0.0;
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

        let pst_value = get_position_value(
            piece.get_position(),
            piece.get_value(),
            end_game,
            piece.is_white(),
        );

        let factor: f32 = if piece.is_white() == board.is_white_move() {
            1.0
        } else {
            -1.0
        };

        pst_score = pst_value * factor;

        let piece_type = get_piece_type(piece.get_value());

        match piece_type {
            PieceType::Queen => q += factor,
            PieceType::Rook => r += factor,
            PieceType::Bishop => b += factor,
            PieceType::Knight => n += factor,
            PieceType::Pawn => {
                p += factor;

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
            // Additional cases for D, S, I, and M are handled below
            _ => (),
        }

        m += piece.get_moves_reference().len() as f32 * factor;
    }

    let score = (QUEEN_WORTH * q) + (ROOK_WORTH * r) + (BISHOP_WORTH * (b + n)) + (PAWN_WORTH * p)
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

pub fn is_doubled_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
    let offset: i8 = if white_piece { -8 } else { 8 };

    let mut position: i8 = position + offset;
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
