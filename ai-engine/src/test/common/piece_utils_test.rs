#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_value_from_fen() {
        assert_eq!(piece_value_from_fen(&'B'), 17);
        assert_eq!(piece_value_from_fen(&'K'), 18);
        assert_eq!(piece_value_from_fen(&'k'), 10);
        assert_eq!(piece_value_from_fen(&'p'), 12);
        assert_eq!(piece_value_from_fen(&'a'), 0); // Test for invalid input
    }

    #[test]
    fn test_piece_fen_from_value() {
        assert_eq!(piece_fen_from_value(17), "B");
        assert_eq!(piece_fen_from_value(18), "K");
        assert_eq!(piece_fen_from_value(10), "k");
        assert_eq!(piece_fen_from_value(12), "p");
        assert_eq!(piece_fen_from_value(99), ""); // Test for invalid input
    }

    #[test]
    fn test_is_piece_of_type() {
        assert_eq!(is_piece_of_type(17, PieceType::Bishop), true);
        assert_eq!(is_piece_of_type(18, PieceType::King), true);
        assert_eq!(is_piece_of_type(18, PieceType::Knight), false);
        assert_eq!(is_piece_of_type(99, PieceType::Pawn), false); // Test for invalid input
    }

    #[test]
    fn test_pieces_to_fen() {
        assert_eq!(pieces_to_fen(vec![17, 18, 10, 12]), vec!["B", "K", "k", "p"]);
        assert_eq!(pieces_to_fen(vec![99, 50]), vec!["", ""]); // Test for invalid input
    }

    #[test]
    fn test_get_piece_type() {
        assert_eq!(get_piece_type(17), PieceType::Bishop as u8);
        assert_eq!(get_piece_type(18), PieceType::King as u8);
        assert_eq!(get_piece_type(10), PieceType::King as u8);
        assert_eq!(get_piece_type(99), PieceType::Empty as u8); // Test for invalid input
    }
}
