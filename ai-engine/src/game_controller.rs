use std::{sync::Mutex, time::Instant};

use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::json;

use crate::{
    ai::ai_utils::get_board_value,
    common::{
        contants::INVALID_BOARD_POSITION,
        piece_move::PieceMove,
    },
    dto::dtos::{AIDepthDTO, FenDTO, MovesCountDTO},
    game::moves_counter::count_moves,
    global_state::GlobalState,
};

#[get("/board")]
pub async fn get_board(global_state: web::Data<Mutex<GlobalState>>) -> impl Responder {
    get_board_response(global_state)
}

#[post("/board/moves/count")]
pub async fn get_move_generation_count(
    piece_move: web::Json<MovesCountDTO>,
    global_state: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    let mut board = &mut global_state.lock().unwrap().board;

    let start = Instant::now();

    let nodes_searched = count_moves(&mut board, piece_move.depth, false);

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

    let black_king_in_check = board_state.is_black_king_in_check();
    let white_king_in_check = board_state.is_white_king_in_check();

    let black_en_passant = board_state.get_black_en_passant();
    let white_en_passant = board_state.get_white_en_passant();

    let board_fen = board_state.get_fen();

    let pieces = board.get_pieces();

    HttpResponse::Ok().json(json!({
        "blackCaptures": board.black_captures_to_fen(),
        "blackEnPassant": black_en_passant,
        "blackKingInCheck": black_king_in_check,
        "boardFen": board_fen,
        "whiteCaptures": board.white_captures_to_fen(),
        "whiteMove": board.is_white_move(),
        "winner": board.get_winner_fen(),
        "zobrit": board.get_zobrist_hash(),
        "pieces": pieces,
        "whiteEnPassant": white_en_passant,
        "whiteKingInCheck": white_king_in_check,
        "boardEvaluation": get_board_value(board, board.is_white_move() && !board.is_game_finished(), &pieces)
    }))
}
