use pyo3::prelude::*;

use crate::common::piece_move::PieceMove;

use super::{dto_utils::piece_move_dto_from_piece_move, piece_move_dto::PieceMoveDTO};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PieceDTO {
    #[pyo3(get, set)]
    pub fen: char,
    #[pyo3(get, set)]
    pub moves: Vec<PieceMoveDTO>,
    #[pyo3(get, set)]
    pub position: i8,
    #[pyo3(get, set)]
    pub white: bool,
}

impl PieceDTO {
    pub fn new(fen: char, moves: Vec<PieceMove>, position: i8, white: bool) -> Self {
        PieceDTO {
            fen,
            moves: moves
                .iter()
                .map(piece_move_dto_from_piece_move)
                .collect(),
            position,
            white,
        }
    }
}
