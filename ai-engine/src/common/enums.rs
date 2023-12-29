#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Empty = 0,
    Bishop = 1,
    King = 2,
    Knight = 3,
    Pawn = 4,
    Queen = 5,
    Rook = 6,
}

impl PieceType {
    pub fn value(&self) -> i8 {
        *self as i8
    }
}

#[derive(Clone, Copy)]
pub enum PieceColor {
    Black = 8,
    White = 16,
}

impl PieceColor {
    pub fn value(&self) -> i8 {
        *self as i8
    }
}
