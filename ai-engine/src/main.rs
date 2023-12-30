mod ai;
mod common;
mod dto;
mod game;
mod game_controller;
mod global_state;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use global_state::GlobalState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started successfully 🚀!");

    let state = web::Data::new(Mutex::new(GlobalState::new()));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::clone(&state))
            .service(game_controller::get_board)
            .service(game_controller::move_piece)
            .service(game_controller::load_fen)
            .service(game_controller::get_move_generation_count)
            // .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
