use serde::{Deserialize, Serialize};

use crate::game_bit_board::{_move::_move::Move, enums::{Color, PieceType}};

#[derive(Debug, Clone, Deserialize)]
pub struct FenDTO {
    pub fen: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MovesCountDTO {
    pub depth: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AIDepthDTO {
    pub time_to_think: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PieceDTO {
    color: Color,
    fen: String,
    position: u64,
    r#type: PieceType,
    moves: Vec<MoveDTO>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MoveDTO {
    pub flags: u16,
    pub from: usize,
    pub to: usize,
    pub promotion_flag: u16
}

impl MoveDTO {
    pub fn from_move(_move: &Move) -> Self {
        Self {
            flags: _move.get_flags(),
            from: _move.get_from(),
            to: _move.get_to(),
            promotion_flag: 0
        }
    }
}