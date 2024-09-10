#![allow(dead_code)]

use std::io;

use game_bit_board::{_move::Move, board::Board, move_generator::move_generator::MoveGenerator};
// mod ai;
// mod common;
// mod dto;
// mod game;
mod game_bit_board;
// mod game_controller;
// mod global_state;

pub fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

use std::num::ParseIntError;

fn to_usize(value: &str) -> Result<usize, ParseIntError> {
    value.parse()
}

fn encode_square(algebraic: &str) -> u16 {
    let file = algebraic.chars().next().unwrap() as u16 - b'a' as u16;
    let rank = algebraic.chars().nth(1).unwrap().to_digit(10).unwrap() as u16 - 1;
    rank * 8 + file
}

fn get_piece_attacks(from: usize, moves: &Vec<Move>) -> Vec<usize>{
    let mut attacks: Vec<usize> = Vec::new();

    for _move in moves {
        if _move.get_from() == from {
            attacks.push(_move.get_to());
        }
    }

    attacks
}

fn main() {
    let mut board = Board::new();
    let move_generator = MoveGenerator::new();

    loop {
        let moves = move_generator.get_moves(&mut board);

        // for (i, _move) in moves.iter().enumerate() {
        //     println!("{}: {}", i+1, _move.to_algebraic_notation());
        // }

        println!();
        
        board.display();

        let input: String = get_input("\nPiece square you want to move (e.g. a1): ");

        let from = encode_square(&input);

        let _move = moves.iter().find(|_move| _move.get_from() == from.into());

        board.display_with_attacks(get_piece_attacks(from.into(), &moves));

        let input: String = get_input("\nSquare you want to move to (e.g. a1): ");

        let to = encode_square(&input);

        let _move = moves.iter().find(|_move| _move.get_from() == from.into() && _move.get_to() == to.into());

        if _move.is_none() {
            println!("Invalid move!");
        } else {
            board.move_piece(_move.unwrap().clone());
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
