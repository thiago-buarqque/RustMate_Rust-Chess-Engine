use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use super::{fen_utils::get_piece_fen, piece_move::PieceMove};

#[derive(Debug)]
pub struct Piece {
    moves: Vec<PieceMove>,
    position: i8,
    value: u8,
    white: bool,
}

impl Serialize for Piece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BoardPiece", 5)?;

        state.serialize_field("fen", &get_piece_fen(self.value))?;
        state.serialize_field("moves", &self.moves)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("white", &self.white)?;

        state.end()
    }
}

impl Piece {
    pub fn new(moves: Vec<PieceMove>, position: i8, value: u8, white: bool) -> Self {
        Piece {
            moves,
            position,
            value,
            white,
        }
    }

    pub fn get_moves_clone(&self) -> Vec<PieceMove> {
        self.moves.clone()
    }

    pub fn get_moves_reference(&self) -> &Vec<PieceMove> {
        &self.moves
    }

    pub fn get_position(&self) -> i8 {
        self.position
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn is_white(&self) -> bool {
        self.white
    }

    pub fn set_moves(&mut self, moves: Vec<PieceMove>) {
        self.moves = moves
    }
}
