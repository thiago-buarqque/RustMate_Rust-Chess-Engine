mod ai;
mod board_wrapper;
mod common;
mod dto;
mod game;

use std::env;

use board_wrapper::BoardWrapper;
use dto::{piece_dto::PieceDTO, piece_move_dto::PieceMoveDTO};
use pyo3::prelude::*;

#[pymodule]
fn ai_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    env::set_var("RUST_BACKTRACE", "1");
    m.add_class::<PieceDTO>()?;
    m.add_class::<PieceMoveDTO>()?;
    m.add_class::<BoardWrapper>()?;

    Ok(())
}
