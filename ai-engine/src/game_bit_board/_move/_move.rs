use core::fmt;

use crate::game_bit_board::enums::{Color, PieceType};

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
    color: Color,
    piece_type: PieceType,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self._move == other._move
            && self.en_passant_bb_position == other.en_passant_bb_position
            && self.en_passant_bb_piece_square == other.en_passant_bb_piece_square
            && self.color == other.color
            && self.piece_type == other.piece_type
    }

    fn ne(&self, other: &Self) -> bool {
        self._move != other._move
            || self.en_passant_bb_position != other.en_passant_bb_position
            || self.en_passant_bb_piece_square != other.en_passant_bb_piece_square
            || self.color != other.color
            || self.piece_type != other.piece_type
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic_notation())
    }
}

impl Move {
    // For test purposes
    pub fn dummy_from_to(from: usize, to: usize) -> Self {
        if cfg!(test) {
            Move::with_flags(0, from, to, Color::Black, PieceType::Empty)
        } else {
            panic!("Can only be used in test envs");
        }
    }

    // For test purposes
    pub fn dummy_with_flags(flags: u16, from: usize, to: usize) -> Self {
        if cfg!(test) {
            Move::with_flags(flags, from, to, Color::Black, PieceType::Empty)
        } else {
            panic!("Can only be used in test envs");
        }
    }

    /// Encondes a normal move `from` square `to` square.
    /// # Parameters
    /// - `from`: The index of the square the piece is moving from (0-63).
    /// - `to`: The index of the square the piece is moving to (0-63).
    ///
    /// # Returns
    /// - A `Move` instance with the encoded move.
    pub fn from_to(from: usize, to: usize, color: Color, piece_type: PieceType) -> Self {
        Move::with_flags(0, from, to, color, piece_type)
    }

    /// Encondes a special move `from` square `to` square.
    /// # Parameters
    /// - `flags`: A 4-bit value used to encode special move types (e.g.,
    ///   promotion, en passant).
    /// - `from`: The index of the square the piece is moving from (0-63).
    /// - `to`: The index of the square the piece is moving to (0-63).
    ///
    /// # Returns
    /// - A `Move` instance with the encoded move.
    pub fn with_flags(
        flags: u16, from: usize, to: usize, color: Color, piece_type: PieceType,
    ) -> Self {
        Self {
            _move: ((flags & 0xf) << 12) | ((from as u16 & 0x3f) << 6) | (to as u16 & 0x3f),
            en_passant_bb_position: 0,
            en_passant_bb_piece_square: 0,
            color,
            piece_type,
        }
    }

    pub fn get_color(&self) -> Color { self.color }

    pub fn get_piece_type(&self) -> PieceType { self.piece_type }

    pub fn as_u64(&self) -> u64 { self._move as u64 }

    pub fn get_flags(&self) -> u16 { (self._move >> 12) & 0x0f }

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

    pub fn is_knight_promo_capture(&self) -> bool { (self._move >> 12) == KNIGHT_PROMOTION_CAPTURE }

    pub fn is_bishop_promo_capture(&self) -> bool { (self._move >> 12) == BISHOP_PROMOTION_CAPTURE }

    pub fn is_rook_promo_capture(&self) -> bool { (self._move >> 12) == ROOK_PROMOTION_CAPTURE }

    pub fn is_queen_promo_capture(&self) -> bool { (self._move >> 12) == QUEEN_PROMOTION_CAPTURE }

    pub fn is_promo_capture(&self) -> bool { (self._move >> 12) >= KNIGHT_PROMOTION_CAPTURE }

    pub fn is_promotion(&self) -> bool { (self._move >> 12) >= KNIGHT_PROMOTION }

    pub fn set_en_passant_bb_position(&mut self, position: u64) {
        self.en_passant_bb_position = position
    }

    pub fn get_en_passant_bb_position(&self) -> u64 { self.en_passant_bb_position }

    pub fn set_en_passant_bb_piece_square(&mut self, square: u64) {
        self.en_passant_bb_piece_square = square
    }

    pub fn get_en_passant_bb_piece_square(&self) -> u64 { self.en_passant_bb_piece_square }

    pub fn to_algebraic_notation(&self) -> String {
        let from_square = self.decode_square((self._move >> 6) & 0x3F);
        let to_square = self.decode_square(self._move & 0x3F);
        let move_flags = (self._move >> 12) & 0xF;

        // Handle castling
        // if move_flags == KING_CASTLE {
        //     return "O-O".to_string();
        // } else if move_flags == QUEEN_CASTLE {
        //     return "O-O-O".to_string();
        // }

        // Piece type
        // let piece = match self.piece_type {
        //     PieceType::Pawn => "".to_string(),
        //     _ => get_piece_symbol(self.color, self.piece_type),
        // };

        // Captures
        // let capture_symbol = if move_flags == CAPTURE || move_flags >=
        // KNIGHT_PROMOTION_CAPTURE {     "x"
        // } else {
        //     ""
        // };

        // Promotion
        let promotion = if move_flags >= KNIGHT_PROMOTION {
            format!("{}", self.promotion_piece(move_flags))
        } else {
            "".to_string()
        };

        // Handle pawn capture notation
        // let pawn_file = if self.piece_type == PieceType::Pawn && capture_symbol ==
        // "x" {     self.file_of(from_square.clone())
        // } else {
        //     "".to_string()
        // };

        format!("{}{}{}", from_square, to_square, promotion)

        // format!(
        //     "{}{}{}{}{}",
        //     pawn_file, from_square, capture_symbol, to_square, promotion
        // )
    }

    fn decode_square(&self, square_index: u16) -> String {
        let file = ((square_index % 8) as u8 + b'a') as char;
        let rank = ((square_index / 8) + 1).to_string();
        format!("{}{}", file, rank)
    }

    fn promotion_piece(&self, flags: u16) -> String {
        match flags {
            KNIGHT_PROMOTION | KNIGHT_PROMOTION_CAPTURE => "n".to_string(),
            BISHOP_PROMOTION | BISHOP_PROMOTION_CAPTURE => "b".to_string(),
            ROOK_PROMOTION | ROOK_PROMOTION_CAPTURE => "r".to_string(),
            QUEEN_PROMOTION | QUEEN_PROMOTION_CAPTURE => "q".to_string(),
            _ => "".to_string(),
        }
    }

    fn file_of(&self, square: String) -> String {
        square.chars().next().unwrap().to_string() // Extracts file (first
                                                   // character)
    }
}

#[cfg(test)]
mod tests {
    // use crate::game_bit_board::positions::Squares;

    use super::*;

    // #[test]
    // fn test_algebraic_notation() {
    //     // Test normal knight move from g1 to f3
    //     let _move = Move::with_flags(
    //         NORMAL,
    //         Squares::G1,
    //         Squares::F3,
    //         Color::White,
    //         PieceType::Knight,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "♘f3");

    //     // Test double pawn push from e2 to e4
    //     let _move = Move::with_flags(
    //         DOUBLE_PAWN_PUSH,
    //         Squares::E2,
    //         Squares::E4,
    //         Color::White,
    //         PieceType::Pawn,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "e4");

    //     // Test pawn capturing from e4 to d5
    //     let _move = Move::with_flags(
    //         CAPTURE,
    //         Squares::E4,
    //         Squares::D5,
    //         Color::White,
    //         PieceType::Pawn,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "exd5");

    //     // Test king-side castle
    //     let _move = Move::with_flags(
    //         KING_CASTLE,
    //         Squares::E8,
    //         Squares::G8,
    //         Color::Black,
    //         PieceType::King,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "O-O");

    //     // Test queen-side castle
    //     let _move = Move::with_flags(
    //         QUEEN_CASTLE,
    //         Squares::E8,
    //         Squares::C8,
    //         Color::Black,
    //         PieceType::King,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "O-O-O");

    //     // Test knight capturing from g5 to f3
    //     let _move = Move::with_flags(
    //         CAPTURE,
    //         Squares::G5,
    //         Squares::F3,
    //         Color::White,
    //         PieceType::Knight,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "♘xf3");

    //     // Test pawn promotion from e7 to e8 to queen
    //     let _move = Move::with_flags(
    //         QUEEN_PROMOTION,
    //         Squares::E7,
    //         Squares::E8,
    //         Color::Black,
    //         PieceType::Pawn,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "e8=Q");

    //     // Test pawn promotion with capture from e7 to d8 to queen
    //     let _move = Move::with_flags(
    //         QUEEN_PROMOTION_CAPTURE,
    //         Squares::E7,
    //         Squares::D8,
    //         Color::Black,
    //         PieceType::Pawn,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "exd8=Q");

    //     // Test bishop move from c1 to g5
    //     let _move = Move::with_flags(
    //         NORMAL,
    //         Squares::B1,
    //         Squares::G5,
    //         Color::White,
    //         PieceType::Bishop,
    //     );
    //     assert_eq!(_move.to_algebraic_notation(), "♗g5");
    // }

    #[test]
    fn test_from_to() {
        let m = Move::dummy_from_to(12, 28);
        assert_eq!(m.get_from(), 12);
        assert_eq!(m.get_to(), 28);
        assert_eq!(m.get_flags(), 0);
    }

    #[test]
    fn test_with_flags() {
        let m = Move::dummy_with_flags(KING_CASTLE, 4, 6);
        assert_eq!(m.get_from(), 4);
        assert_eq!(m.get_to(), 6);
        assert_eq!(m.get_flags(), KING_CASTLE);
    }

    #[test]
    fn test_is_normal() {
        let m = Move::dummy_from_to(12, 28);
        assert!(m.is_normal());
    }

    #[test]
    fn test_is_double_pawn_push() {
        let m = Move::dummy_with_flags(DOUBLE_PAWN_PUSH, 12, 20);
        assert!(m.is_double_pawn_push());
    }

    #[test]
    fn test_is_king_castle() {
        let m = Move::dummy_with_flags(KING_CASTLE, 4, 6);
        assert!(m.is_king_castle());
    }

    #[test]
    fn test_is_queen_castle() {
        let m = Move::dummy_with_flags(QUEEN_CASTLE, 4, 6);
        assert!(m.is_queen_castle());
    }

    #[test]
    fn test_is_capture() {
        let m = Move::dummy_with_flags(CAPTURE, 12, 28);
        assert!(m.is_capture());
    }

    #[test]
    fn test_is_en_passant() {
        let m = Move::dummy_with_flags(EN_PASSANT, 12, 28);
        assert!(m.is_en_passant());
    }

    #[test]
    fn test_is_knight_promotion() {
        let m = Move::dummy_with_flags(KNIGHT_PROMOTION, 12, 28);
        assert!(m.is_knight_promotion());
    }

    #[test]
    fn test_is_bishop_promotion() {
        let m = Move::dummy_with_flags(BISHOP_PROMOTION, 12, 28);
        assert!(m.is_bishop_promotion());
    }

    #[test]
    fn test_is_rook_promotion() {
        let m = Move::dummy_with_flags(ROOK_PROMOTION, 12, 28);
        assert!(m.is_rook_promotion());
    }

    #[test]
    fn test_is_queen_promotion() {
        let m = Move::dummy_with_flags(QUEEN_PROMOTION, 12, 28);
        assert!(m.is_queen_promotion());
    }

    #[test]
    fn test_is_knight_promo_capture() {
        let m = Move::dummy_with_flags(KNIGHT_PROMOTION_CAPTURE, 12, 28);
        assert!(m.is_knight_promo_capture());
    }

    #[test]
    fn test_is_bishop_promo_capture() {
        let m = Move::dummy_with_flags(BISHOP_PROMOTION_CAPTURE, 12, 28);
        assert!(m.is_bishop_promo_capture());
    }

    #[test]
    fn test_is_rook_promo_capture() {
        let m = Move::dummy_with_flags(ROOK_PROMOTION_CAPTURE, 12, 28);
        assert!(m.is_rook_promo_capture());
    }

    #[test]
    fn test_is_queen_promo_capture() {
        let m = Move::dummy_with_flags(QUEEN_PROMOTION_CAPTURE, 12, 28);
        assert!(m.is_queen_promo_capture());
    }
}
