#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty
}
