use crate::common::piece_move::PieceMove;

use super::{dto_utils::piece_move_dto_from_piece_move, piece_move_dto::PieceMoveDTO};

#[derive(Debug, Clone)]
pub struct PieceDTO {
    pub fen: char,
    pub moves: Vec<PieceMoveDTO>,
    pub position: i8,
    pub white: bool,
}

impl PieceDTO {
    pub fn new(fen: char, moves: Vec<PieceMove>, position: i8, white: bool) -> Self {
        PieceDTO {
            fen,
            moves: moves.iter().map(piece_move_dto_from_piece_move).collect(),
            position,
            white,
        }
    }
}
