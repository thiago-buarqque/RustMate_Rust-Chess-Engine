use std::{sync::{Mutex, MutexGuard}, time::Instant};

use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::{json, Value};

use crate::{
    dto::dtos::{FenDTO, MoveDTO, MovesCountDTO}, game_bit_board::{_move::moves_counter::count_moves, board::Board, move_generator::move_generator::MoveGenerator, utils::utils::get_piece_letter}
};

#[get("/board")]
pub async fn get_board(board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>) -> impl Responder {
    get_board_response(&mut board.lock().unwrap(), &mut move_generator.lock().unwrap())
}

#[post("/board/moves/count")]
pub async fn get_move_generation_count(
    piece_move: web::Json<MovesCountDTO>,
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>
) -> impl Responder {
    let mut board = &mut board.lock().unwrap();
    let mut move_generator = &mut move_generator.lock().unwrap();

    let start = Instant::now();

    let nodes_searched = count_moves(&mut board, piece_move.depth, false, &mut move_generator);

    HttpResponse::Ok().json(json!({
        "moves": nodes_searched,
        "elapsedTime": start.elapsed().as_millis(),
    }))
}

#[post("/board/move/piece")]
pub async fn move_piece(
    _move1: web::Json<MoveDTO>,
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>
) -> impl Responder {
    let board = &mut board.lock().unwrap();

    let mut move_generator= move_generator.lock().unwrap();

    let moves = move_generator.get_moves(board);

    for _move2 in moves {
        if _move2.get_from() != _move1.from || _move2.get_to() != _move1.to {
            continue;
        }

        board.move_piece(&mut _move2.clone());

        return get_board_response(board, &mut move_generator);
    }

    HttpResponse::NotFound().json(json!({
        "message": "Move not found"
    }))
}

#[post("/board/load/fen")]
pub async fn load_fen(
    fen_dto: web::Json<FenDTO>,
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>
) -> impl Responder {
    let board = &mut board.lock().unwrap();

    board.load_position(&fen_dto.fen);

    get_board_response(board, &mut move_generator.lock().unwrap())
}

// #[post("/ai/time_to_think")]
// pub async fn set_ai_depth(
//     depth: web::Json<AIDepthDTO>,
//     global_state: web::Data<Mutex<GlobalState>>,
// ) -> impl Responder {
//     global_state.lock().unwrap().seconds_to_think = depth.time_to_think;

//     HttpResponse::Ok()
// }

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

pub fn get_board_response(board: &mut MutexGuard<'_, Board>, move_generator: &mut MutexGuard<'_, MoveGenerator>) -> HttpResponse {
    let fen = board.to_fen();
    let en_passant = board.get_en_passant_bb_position();
    let side_to_move = board.get_side_to_move();
    let zobrist = board.get_zobrist_hash();
    
    let pieces: Vec<Value> = get_pieces(board, move_generator);

    HttpResponse::Ok().json(json!({
        "blackCaptures": Vec::<String>::new(),
        "enPassant": en_passant,
        "blackKingInCheck": false,
        "fen": fen,
        "whiteCaptures": Vec::<String>::new(),
        "siteToMove": side_to_move,
        "winner": board.get_winner(),
        "zobrit": zobrist,
        "pieces": pieces,
        "whiteKingInCheck": false
        // "boardEvaluation": get_board_value(board, board.is_white_move() && !board.is_game_finished(), &pieces)
    }))
}

fn get_pieces(board: &mut Board, move_generator: &mut MoveGenerator) -> Vec<Value> {
    let moves: Vec<MoveDTO> = move_generator.get_moves(board).iter().map(|_move| MoveDTO::from_move(_move)).collect();
    
    let mut pieces: Vec<Value> = Vec::with_capacity(64);

    for square in 0..64 {
        let piece_type = board.get_piece_type(square);
        let piece_color = board.get_piece_color(square);
        let fen = get_piece_letter(piece_color, piece_type);

        pieces.push(json!({
            "color": piece_color,
            "fen": fen,
            "moves": moves.iter().filter(|_move| _move.from == square).collect::<Vec<_>>(),
            "position": square,
            "type": piece_type,
        }));
    }

    pieces
}