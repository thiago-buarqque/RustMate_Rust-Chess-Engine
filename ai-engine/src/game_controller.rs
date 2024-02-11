use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    ai::ai_utils::get_board_value,
    common::{
        board_utils::get_move_notation,
        contants::{EMPTY_PIECE, INVALID_BOARD_POSITION},
        piece_move::PieceMove,
        piece_utils::get_promotion_options,
    },
    dto::dtos::{AIDepthDTO, FenDTO, MovesCountDTO},
    game::board::Board,
    global_state::GlobalState,
};

#[get("/board")]
pub async fn get_board(global_state: web::Data<Mutex<GlobalState>>) -> impl Responder {
    get_board_response(global_state)
}

use std::{sync::Mutex, time::Instant};

#[post("/board/moves/count")]
pub async fn get_move_generation_count(
    piece_move: web::Json<MovesCountDTO>,
    global_state: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let mut board = &mut global_state.lock().unwrap().board;

    let start = Instant::now();

    let nodes_searched = move_generation_count(&mut board, piece_move.depth, false);

    HttpResponse::Ok().json(json!({
        "moves": nodes_searched,
        "elapsedTime": start.elapsed().as_millis(),
    }))
}

#[post("/board/move/piece")]
pub async fn move_piece(
    piece_move: web::Json<PieceMove>,
    global_state: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    // println!("Req: {:?}", req);
    let mut _global_state = global_state.lock().unwrap();

    let time_to_think = _global_state.time_to_think;

    let board = &mut _global_state.board;

    let _ = board.move_piece(&piece_move);

    let mut board_clone = board.clone();

    let ai = &mut _global_state.ai;

    let (_, ai_move) = ai.get_move(&mut board_clone, time_to_think);

    let board = &mut _global_state.board;

    if ai_move.get_from_position() != INVALID_BOARD_POSITION
        && ai_move.get_to_position() != INVALID_BOARD_POSITION
    {
        let _ = board.move_piece(&ai_move);
    }

    drop(_global_state);

    get_board_response(global_state)
}

#[post("/board/load/fen")]
pub async fn load_fen(
    fen_dto: web::Json<FenDTO>,
    global_state: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let mut _global_state = global_state.lock().unwrap();
    let board = &mut _global_state.board;

    board.load_position(&fen_dto.fen);

    drop(_global_state);

    get_board_response(global_state)
}

#[post("/ai/time_to_think")]
pub async fn set_ai_depth(
    depth: web::Json<AIDepthDTO>,
    global_state: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    global_state.lock().unwrap().time_to_think = depth.time_to_think;

    HttpResponse::Ok()
}

pub fn get_board_response(global_state: web::Data<Mutex<GlobalState>>) -> impl Responder {
    let mut _global_state = global_state.lock().unwrap();
    let board = &mut _global_state.board;

    let board_state = board.get_state_reference();

    let black_en_passant = board_state.get_black_en_passant();
    let white_en_passant = board_state.get_white_en_passant();
    let board_fen = board_state.get_fen();

    let pieces = board.get_pieces();

    HttpResponse::Ok().json(json!({
        "blackCaptures": board.black_captures_to_fen(),
        "blackEnPassant": black_en_passant,
        "boardFen": board_fen,
        "whiteCaptures": board.white_captures_to_fen(),
        "whiteMove": board.is_white_move(),
        "winner": board.get_winner_fen(),
        "zobrit": board.get_zobrist_hash(),
        "pieces": pieces,
        "whiteEnPassant": white_en_passant,
        "boardEvaluation": get_board_value(board, board.is_white_move() && !board.is_game_finished(), &pieces)
    }))
}

fn move_generation_count(board: &mut Board, depth: usize, track_moves: bool) -> u64 {
    if depth == 0 || board.is_game_finished() {
        return 1;
    }

    let pieces = board.get_pieces();

    let mut num_positions: u64 = 0;

    for piece in pieces.iter() {
        if (piece.get_value() == EMPTY_PIECE) || (piece.is_white() != board.is_white_move()) {
            continue;
        }

        for piece_move in piece.get_moves_clone().iter() {
            let mut promotion_char_options = vec![piece_move.get_promotion_value()];

            if piece_move.is_promotion() {
                promotion_char_options = get_promotion_options(piece.is_white());
            }

            let mut piece_move = piece_move.clone();

            for promotion_option in promotion_char_options {
                piece_move.set_promotion_value(promotion_option);

                let _ = board.move_piece(&piece_move);

                let moves_count = move_generation_count(board, depth - 1, false);
                num_positions += moves_count;

                if track_moves {
                    if piece_move.is_promotion() {
                        println!(
                            "{}{}: {}",
                            get_move_notation(&piece_move),
                            promotion_option.clone(),
                            moves_count
                        )
                    } else {
                        println!("{}: {}", get_move_notation(&piece_move), moves_count)
                    }
                }

                board.undo_last_move();
            }
        }
    }

    num_positions
}

// #[cfg(test)]
// mod tests {
//     use super::BoardWrapper;

//     #[test]
//     fn test_move_generation_count() {
//         let mut board_wrapper = BoardWrapper::default();

//         // Positions for initial FEN

//         assert_eq!(board_wrapper.get_move_generation_count(1), 20);
//         assert_eq!(board_wrapper.get_move_generation_count(2), 400);
//         assert_eq!(board_wrapper.get_move_generation_count(3), 8_902);
//         assert_eq!(board_wrapper.get_move_generation_count(4), 197_281);

//         board_wrapper
//             .load_position("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");

//         assert_eq!(board_wrapper.get_move_generation_count(1), 48);
//         assert_eq!(board_wrapper.get_move_generation_count(2), 2_039);
//         assert_eq!(board_wrapper.get_move_generation_count(3), 97_862);
//         assert_eq!(board_wrapper.get_move_generation_count(4), 4_085_603);
//     }
// }
