use std::{sync::Mutex, time::Instant};

use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::{json, Value};

use crate::{
    dto::dtos::{AIDepthDTO, FenDTO, MovesCountDTO}, game_bit_board::{_move::{_move::Move, moves_counter::count_moves}, board::Board, enums::Color, utils::{bitwise_utils::pop_lsb, utils::get_piece_letter}}, global_state::GlobalState
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
    let mut move_generator = &mut global_state.lock().unwrap().move_generator;

    let start = Instant::now();

    let nodes_searched = count_moves(&mut board, piece_move.depth, false, &mut move_generator);

    HttpResponse::Ok().json(json!({
        "moves": nodes_searched,
        "elapsedTime": start.elapsed().as_millis(),
    }))
}

#[post("/board/move/piece")]
pub async fn move_piece(
    _move: web::Json<Move>,
    global_state: web::Data<Mutex<GlobalState>>,
) -> impl Responder {
    // println!("Req: {:?}", req);
    let mut _global_state = global_state.lock().unwrap();

    let board = &mut _global_state.board;

    let mut _move = _move.clone();

    let _ = board.move_piece(&mut _move);

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
    global_state.lock().unwrap().seconds_to_think = depth.time_to_think;

    HttpResponse::Ok()
}

// #[post("/ai/move")]
// pub async fn ai_move(global_state: web::Data<Mutex<GlobalState>>) -> impl Responder {
//     let mut _global_state = global_state.lock().unwrap();

//     let time_to_think = _global_state.seconds_to_think;

//     if _global_state.board.is_game_finished() {
//         return HttpResponse::Ok().json(json!({
//             "depth": 0,
//             "duration": 0,
//             "evaluation": 0,
//             "aiMove": None::<Move>
//         }));
//     }

//     let mut board = _global_state.board.to_owned();
    
//     let ai_player = &mut _global_state.ai;

//     let (duration, depth, evaluation, ai_move) = ai_player.get_move(&mut board, time_to_think);

//     let board = &mut _global_state.board;

//     let _ = board.move_piece(&ai_move);

//     HttpResponse::Ok().json(json!({
//         "depth": depth,
//         "duration": duration,
//         "evaluation": evaluation,
//         "aiMove": ai_move
//     }))
// }

pub fn get_board_response(global_state: web::Data<Mutex<GlobalState>>) -> impl Responder {
    let mut _global_state = global_state.lock().unwrap();

    let board = &mut _global_state.board;

    let fen = board.to_fen();
    let en_passant = board.get_en_passant_bb_position();
    let side_to_move = board.get_side_to_move();
    let zobrist = board.get_zobrist_hash();
    let pieces: Vec<Value> = get_pieces(board);

    let mut board_clone = board.clone();

    let move_generator = &mut _global_state.move_generator;

    let moves: Vec<Move> = move_generator.get_moves(&mut board_clone);

    HttpResponse::Ok().json(json!({
        "blackCaptures": Vec::<String>::new(),
        "enPassant": en_passant,
        "blackKingInCheck": false,
        "fen": fen,
        "whiteCaptures": Vec::<String>::new(),
        "siteToMove": side_to_move,
        "winner": "-",
        "zobrit": zobrist,
        "moves": moves,
        "pieces": pieces,
        "whiteKingInCheck": false,
        // "boardEvaluation": get_board_value(board, board.is_white_move() && !board.is_game_finished(), &pieces)
    }))
}

fn get_pieces(board: &mut Board) -> Vec<Value> {
    let mut pieces: Vec<Value> = Vec::with_capacity(64);

    for square in 0..64 {
        let piece_type = board.get_piece_type(square);
        let piece_color = board.get_piece_color(square);
        let fen = get_piece_letter(piece_color, piece_type);

        pieces.push(json!({
            "color": piece_color,
            "fen": fen,
            "position": square,
            "type": piece_type,
        }));
    }

    pieces
}