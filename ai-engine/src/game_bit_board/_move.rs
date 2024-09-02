use core::fmt;

use crate::game_bit_board::positions::Squares;

use super::move_contants::*;

/// Encodes a move with source and destination squares and optional flags.
///
/// The move is encoded as follows:
/// - The lowest 6 bits (bits 0-5) store the destination square index (`to`).
/// - The next 6 bits (bits 6-11) store the source square index (`from`).
/// - The highest 4 bits (bits 12-15) store any special flags.
///
/// ```markdown
/// | Flag | Move Type              |
/// |------|------------------------|
/// | 0000 | Normal                 |
/// | 0001 | Double Pawn Push       |
/// | 0010 | King Castle            |
/// | 0011 | Queen Castle           |
/// | 0100 | Capture                |
/// | 0101 | En Passant             |
/// | 1000 | Knight Promotion       |
/// | 1001 | Bishop Promotion       |
/// | 1010 | Rook Promotion         |
/// | 1011 | Queen Promotion        |
/// | 1100 | Knight Promo Capture   |
/// | 1101 | Bishop Promo Capture   |
/// | 1110 | Rook Promo Capture     |
/// | 1111 | Queen Promo Capture    |
/// ```
#[derive(Debug, Clone)]
pub struct Move {
    _move: u16,
    en_passant_bb_position: u64,
    en_passant_bb_piece_square: u64,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self._move == other._move
            && self.en_passant_bb_position == other.en_passant_bb_position
            && self.en_passant_bb_piece_square == other.en_passant_bb_piece_square
    }

    fn ne(&self, other: &Self) -> bool {
        self._move != other._move
            || self.en_passant_bb_position != other.en_passant_bb_position
            || self.en_passant_bb_piece_square != other.en_passant_bb_piece_square
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flag = match self._move >> 12 {
            NORMAL => "NORMAL",
            DOUBLE_PAWN_PUSH => "DOUBLE_PAWN_PUSH",
            KING_CASTLE => "KING_CASTLE",
            QUEEN_CASTLE => "QUEEN_CASTLE",
            CAPTURE => "CAPTURE",
            EN_PASSANT => "EN_PASSANT",
            KNIGHT_PROMOTION => "KNIGHT_PROMOTION",
            BISHOP_PROMOTION => "BISHOP_PROMOTION",
            ROOK_PROMOTION => "ROOK_PROMOTION",
            QUEEN_PROMOTION => "QUEEN_PROMOTION",
            KNIGHT_PROMO_CAPTURE => "KNIGHT_PROMO_CAPTURE",
            BISHOP_PROMO_CAPTURE => "BISHOP_PROMO_CAPTURE",
            ROOK_PROMO_CAPTURE => "ROOK_PROMO_CAPTURE",
            QUEEN_PROMO_CAPTURE => "QUEEN_PROMO_CAPTURE",
            _ => "UNKNOWN FLAG",
        };

        write!(
            f,
            "{} -> {} | {} | ep_position: {} | ep_square: {}",
            Squares::to_string(self.get_from()),
            Squares::to_string(self.get_to()),
            flag,
            self.en_passant_bb_position,
            self.en_passant_bb_piece_square
        )
    }
}

impl Move {
    /// Encondes a normal move `from` square `to` square.
    /// # Parameters
    /// - `from`: The index of the square the piece is moving from (0-63).
    /// - `to`: The index of the square the piece is moving to (0-63).
    ///
    /// # Returns
    /// - A `Move` instance with the encoded move.
    pub fn from_to(from: usize, to: usize) -> Self { Move::with_flags(0, from, to) }

    /// Encondes a special move `from` square `to` square.
    /// # Parameters
    /// - `flags`: A 4-bit value used to encode special move types (e.g.,
    ///   promotion, en passant).
    /// - `from`: The index of the square the piece is moving from (0-63).
    /// - `to`: The index of the square the piece is moving to (0-63).
    ///
    /// # Returns
    /// - A `Move` instance with the encoded move.
    pub fn with_flags(flags: u16, from: usize, to: usize) -> Self {
        Self {
            _move: ((flags & 0xf) << 12) | ((from as u16 & 0x3f) << 6) | (to as u16 & 0x3f),
            en_passant_bb_position: 0,
            en_passant_bb_piece_square: 0,
        }
    }

    pub fn as_u64(&self) -> u64 { self._move as u64 }

    fn get_flags(&self) -> u16 { (self._move >> 12) & 0x0f }

    pub fn get_from(&self) -> usize { ((self._move >> 6) & 0x3f) as usize }

    pub fn get_to(&self) -> usize { (self._move & 0x3f) as usize }

    pub fn is_normal(&self) -> bool { (self._move >> 12) == NORMAL }

    pub fn is_double_pawn_push(&self) -> bool { (self._move >> 12) == DOUBLE_PAWN_PUSH }

    pub fn is_king_castle(&self) -> bool { (self._move >> 12) == KING_CASTLE }

    pub fn is_queen_castle(&self) -> bool { (self._move >> 12) == QUEEN_CASTLE }

    pub fn is_capture(&self) -> bool { (self._move >> 12) == CAPTURE }

    pub fn is_en_passant(&self) -> bool { (self._move >> 12) == EN_PASSANT }

    pub fn is_knight_promotion(&self) -> bool { (self._move >> 12) == KNIGHT_PROMOTION }

    pub fn is_bishop_promotion(&self) -> bool { (self._move >> 12) == BISHOP_PROMOTION }

    pub fn is_rook_promotion(&self) -> bool { (self._move >> 12) == ROOK_PROMOTION }

    pub fn is_queen_promotion(&self) -> bool { (self._move >> 12) == QUEEN_PROMOTION }

    pub fn is_knight_promo_capture(&self) -> bool { (self._move >> 12) == KNIGHT_PROMO_CAPTURE }

    pub fn is_bishop_promo_capture(&self) -> bool { (self._move >> 12) == BISHOP_PROMO_CAPTURE }

    pub fn is_rook_promo_capture(&self) -> bool { (self._move >> 12) == ROOK_PROMO_CAPTURE }

    pub fn is_queen_promo_capture(&self) -> bool { (self._move >> 12) == QUEEN_PROMO_CAPTURE }

    pub fn set_en_passant_bb_position(&mut self, position: u64) {
        self.en_passant_bb_position = position
    }

    pub fn get_en_passant_bb_position(&self) -> u64 { self.en_passant_bb_position }

    pub fn set_en_passant_bb_piece_square(&mut self, square: u64) {
        self.en_passant_bb_piece_square = square
    }

    pub fn get_en_passant_bb_piece_square(&self) -> u64 { self.en_passant_bb_piece_square }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_to() {
        let m = Move::from_to(12, 28);
        assert_eq!(m.get_from(), 12);
        assert_eq!(m.get_to(), 28);
        assert_eq!(m.get_flags(), 0);
    }

    #[test]
    fn test_with_flags() {
        let m = Move::with_flags(KING_CASTLE, 4, 6);
        assert_eq!(m.get_from(), 4);
        assert_eq!(m.get_to(), 6);
        assert_eq!(m.get_flags(), KING_CASTLE);
    }

    #[test]
    fn test_is_normal() {
        let m = Move::from_to(12, 28);
        assert!(m.is_normal());
    }

    #[test]
    fn test_is_double_pawn_push() {
        let m = Move::with_flags(DOUBLE_PAWN_PUSH, 12, 20);
        assert!(m.is_double_pawn_push());
    }

    #[test]
    fn test_is_king_castle() {
        let m = Move::with_flags(KING_CASTLE, 4, 6);
        assert!(m.is_king_castle());
    }

    #[test]
    fn test_is_queen_castle() {
        let m = Move::with_flags(QUEEN_CASTLE, 4, 6);
        assert!(m.is_queen_castle());
    }

    #[test]
    fn test_is_capture() {
        let m = Move::with_flags(CAPTURE, 12, 28);
        assert!(m.is_capture());
    }

    #[test]
    fn test_is_en_passant() {
        let m = Move::with_flags(EN_PASSANT, 12, 28);
        assert!(m.is_en_passant());
    }

    #[test]
    fn test_is_knight_promotion() {
        let m = Move::with_flags(KNIGHT_PROMOTION, 12, 28);
        assert!(m.is_knight_promotion());
    }

    #[test]
    fn test_is_bishop_promotion() {
        let m = Move::with_flags(BISHOP_PROMOTION, 12, 28);
        assert!(m.is_bishop_promotion());
    }

    #[test]
    fn test_is_rook_promotion() {
        let m = Move::with_flags(ROOK_PROMOTION, 12, 28);
        assert!(m.is_rook_promotion());
    }

    #[test]
    fn test_is_queen_promotion() {
        let m = Move::with_flags(QUEEN_PROMOTION, 12, 28);
        assert!(m.is_queen_promotion());
    }

    #[test]
    fn test_is_knight_promo_capture() {
        let m = Move::with_flags(KNIGHT_PROMO_CAPTURE, 12, 28);
        assert!(m.is_knight_promo_capture());
    }

    #[test]
    fn test_is_bishop_promo_capture() {
        let m = Move::with_flags(BISHOP_PROMO_CAPTURE, 12, 28);
        assert!(m.is_bishop_promo_capture());
    }

    #[test]
    fn test_is_rook_promo_capture() {
        let m = Move::with_flags(ROOK_PROMO_CAPTURE, 12, 28);
        assert!(m.is_rook_promo_capture());
    }

    #[test]
    fn test_is_queen_promo_capture() {
        let m = Move::with_flags(QUEEN_PROMO_CAPTURE, 12, 28);
        assert!(m.is_queen_promo_capture());
    }
}
