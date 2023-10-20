use super::enums::{PieceColor, PieceType};

pub const EMPTY_PIECE: i8 = PieceType::Empty as i8;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_LOWER_BOUND: i8 = PieceColor::White as i8 | PieceType::Bishop as i8;
pub const WHITE_UPPER_BOUND: i8 = PieceColor::White as i8 | PieceType::Rook as i8;

pub const BISHOP_WORTH: f32 = 300.0;
pub const KING_WORTH: f32 = 20000.0;
pub const KNIGHT_WORTH: f32 = 300.0;
pub const PAWN_WORTH: f32 = 100.0;
pub const QUEEN_WORTH: f32 = 900.0;
pub const ROOK_WORTH: f32 = 500.0;
