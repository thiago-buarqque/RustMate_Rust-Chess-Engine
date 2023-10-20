use crate::common::{piece_move::PieceMove, piece_utils::piece_fen_from_value};

use super::piece_move_dto::PieceMoveDTO;

pub fn piece_move_dto_from_piece_move(piece_move: &PieceMove) -> PieceMoveDTO {
    PieceMoveDTO {
        from_position: piece_move.get_from_position(),
        is_capture: piece_move.is_capture(),
        to_position: piece_move.get_to_position(),
        piece_value: piece_move.get_piece_value(),
        promotion_type: piece_fen_from_value(piece_move.get_promotion_value()),
        is_promotion: piece_move.is_promotion(),
        is_en_passant: piece_move.is_en_passant(),
    }
}
