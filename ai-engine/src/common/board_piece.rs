use super::piece_move::PieceMove;

#[derive(Debug, Clone)]
pub struct BoardPiece {
    moves: Vec<PieceMove>,
    position: i8,
    value: i8,
    white: bool,
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
