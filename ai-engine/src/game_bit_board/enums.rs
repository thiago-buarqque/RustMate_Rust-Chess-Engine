use core::fmt;

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
    Empty,
}

// Borrowed from https://stackoverflow.com/a/32712140/14209524
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
