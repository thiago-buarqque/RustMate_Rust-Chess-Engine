#![allow(dead_code)]

use std::{io, num::ParseIntError};

use game_bit_board::{
    _move::{_move::Move, move_utils::get_promotion_flag_from_symbol, moves_counter::count_moves},
    board::Board,
    move_generator::move_generator::MoveGenerator,
    utils::utils::algebraic_to_square,
};
// mod ai;
// mod common;
// mod dto;
// mod game;
mod game_bit_board;
// mod game_controller;
// mod global_state;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

fn to_usize(value: &str) -> Result<usize, ParseIntError> { value.parse() }

fn get_piece_attacks(from: usize, moves: &Vec<Move>) -> Vec<usize> {
    let mut attacks: Vec<usize> = Vec::new();

    for _move in moves {
        if _move.get_from() == from {
            attacks.push(_move.get_to());
        }
    }

    attacks
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();

    // "8/2p5/3p4/KP5r/1R2Pp1k/8/6P1/8 b - e3"
    // r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
    let mut board = Board::new();
    let move_generator = MoveGenerator::new();

    loop {
        let moves = move_generator.get_moves(&mut board);

        // for (i, _move) in moves.iter().enumerate() {
        //     println!("{}: {}", i+1, _move.to_algebraic_notation());
        // }

        println!();

        board.display();

        let input: String = get_input("\nSearch depth: ");

        let depth = to_usize(&input);

        match depth {
            Ok(depth) => {
                count_moves(&mut board, depth, true, &move_generator);
            }
            Err(_) => {
                println!("Invalid option")
            }
        }

        let input: String = get_input("\nPiece square you want to move (e.g. a1): ");

        let from = algebraic_to_square(&input);

        let _move = moves.iter().find(|_move| _move.get_from() == from);

        board.display_with_attacks(get_piece_attacks(from, &moves));

        let input: String = get_input("\nSquare you want to move to (e.g. a1): ");

        let mut _move;

        if input.len() == 3 {
            // It's a promotion move
            println!("Actual move {}", &input[..2]);
            let to = algebraic_to_square(&input[..2]);

            let promotion_char = input.chars().nth(2).unwrap();

            let (promotion_flag, promotion_capture_flag) =
                get_promotion_flag_from_symbol(promotion_char);

            _move = moves.iter().find(|_move| {
                _move.get_from() == from
                    && _move.get_to() == to
                    && (_move.get_flags() == promotion_flag
                        || _move.get_flags() == promotion_capture_flag)
            });
        } else {
            let to = algebraic_to_square(&input);

            _move = moves
                .iter()
                .find(|_move| _move.get_from() == from && _move.get_to() == to);
        }

        if _move.is_none() {
            println!("Invalid move!");
        } else {
            let mut _move = _move.unwrap().clone();

            board.move_piece(&mut _move);
        }
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     rayon::ThreadPoolBuilder::new()
//         .num_threads(24)
//         .build_global()
//         .unwrap();
//     println!("Server started successfully 🚀!");

//     let state = web::Data::new(Mutex::new(GlobalState::new()));
//     HttpServer::new(move || {
//         let cors = Cors::default()
//             .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//             .allowed_origin("http://localhost:3000")
//             .allowed_headers(vec![
//                 header::CONTENT_TYPE,
//                 header::AUTHORIZATION,
//                 header::CONTENT_ENCODING,
//                 header::ACCEPT,
//             ])
//             .supports_credentials();

//         App::new()
//             .app_data(web::Data::clone(&state))
//             .service(game_controller::get_board)
//             .service(game_controller::get_move_generation_count)
//             .service(game_controller::load_fen)
//             .service(game_controller::move_piece)
//             .service(game_controller::set_ai_depth)
//             .service(game_controller::ai_move)
//             // .configure(config)
//             .wrap(cors)
//             .wrap(Logger::default())
//     })
//     .bind(("127.0.0.1", 8000))?
//     .run()
//     .await
// }
