use serde::{ser::{SerializeStruct, Serializer}, Serialize};

use super::{piece_move::PieceMove, piece_utils::piece_fen_from_value};

#[derive(Debug, Clone)]
pub struct BoardPiece {
    moves: Vec<PieceMove>,
    position: i8,
    value: i8,
    white: bool,
}

impl Serialize for BoardPiece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("BoardPiece", 4)?;
        state.serialize_field("moves", &self.moves)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("fen", &piece_fen_from_value(self.value))?;
        state.serialize_field("white", &self.white)?;
        state.end()
    }
}

impl BoardPiece {
    pub fn new(moves: Vec<PieceMove>, position: i8, value: i8, white: bool) -> Self {
        BoardPiece {
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

    pub fn get_value(&self) -> i8 {
        self.value
    }

    pub fn is_white(&self) -> bool {
        self.white
    }

    pub fn set_moves(&mut self, moves: Vec<PieceMove>) {
        self.moves = moves
    }
}
