use crate::dto::piece_move_dto::PieceMoveDTO;

use super::{contants::EMPTY_PIECE, piece_utils::piece_value_from_fen};

#[derive(Debug, Clone, PartialEq)]
pub struct PieceMove {
    from_position: i8,
    is_capture: bool,
    is_en_passant: bool,
    is_promotion: bool,
    move_worth: i32,
    piece_value: i8,
    promotion_type: i8,
    to_position: i8,
}

impl PieceMove {
    pub fn new(from: i8, piece_value: i8, to: i8) -> Self {
        Self {
            from_position: from,
            is_capture: false,
            is_en_passant: false,
            is_promotion: false,
            move_worth: 0,
            piece_value,
            promotion_type: EMPTY_PIECE,
            to_position: to,
        }
    }

    pub fn from_dto(piece_move_dto: PieceMoveDTO) -> Self {
        Self {
            from_position: piece_move_dto.from_position,
            is_capture: piece_move_dto.is_capture,
            is_en_passant: piece_move_dto.is_en_passant,
            is_promotion: piece_move_dto.is_promotion,
            move_worth: 0,
            piece_value: piece_move_dto.piece_value,
            promotion_type: piece_value_from_fen(&piece_move_dto.promotion_type),
            to_position: piece_move_dto.to_position,
        }
    }

    pub fn clone(&self) -> PieceMove {
        PieceMove {
            from_position: self.from_position,
            is_capture: self.is_capture,
            is_en_passant: self.is_en_passant,
            is_promotion: self.is_promotion,
            move_worth: self.move_worth,
            piece_value: self.piece_value,
            promotion_type: self.promotion_type,
            to_position: self.to_position,
        }
    }

    pub fn eq(&self, piece_move: &Self) -> bool {
        (self.from_position == piece_move.from_position)
            && (self.is_capture == piece_move.is_capture)
            && (self.is_en_passant == piece_move.is_en_passant)
            && (self.is_promotion == piece_move.is_promotion)
            && (self.move_worth == piece_move.move_worth)
            && (self.piece_value == piece_move.piece_value)
            && (self.promotion_type == piece_move.promotion_type)
            && (self.to_position == piece_move.to_position)
    }

    pub fn get_from_position(&self) -> i8 {
        self.from_position
    }

    pub fn get_move_worth(&self) -> i32 {
        self.move_worth
    }

    pub fn get_piece_value(&self) -> i8 {
        self.piece_value
    }

    pub fn get_promotion_value(&self) -> i8 {
        self.promotion_type
    }

    pub fn get_to_position(&self) -> i8 {
        self.to_position
    }

    pub fn is_capture(&self) -> bool {
        self.is_capture
    }

    pub fn is_en_passant(&self) -> bool {
        self.is_en_passant
    }

    pub fn is_promotion(&self) -> bool {
        self.is_promotion
    }

    pub fn set_from_position(&mut self, from_position: i8) {
        self.from_position = from_position;
    }

    pub fn set_is_capture(&mut self, is_capture: bool) {
        self.is_capture = is_capture;
    }

    pub fn set_is_en_passant(&mut self, is_en_passant: bool) {
        self.is_en_passant = is_en_passant;
    }

    pub fn set_is_promotion(&mut self, is_promotion: bool) {
        self.is_promotion = is_promotion;
    }

    pub fn set_move_worth(&mut self, move_worth: i32) {
        self.move_worth = move_worth;
    }

    pub fn set_piece_value(&mut self, piece_value: i8) {
        self.piece_value = piece_value;
    }

    pub fn set_promotion_value(&mut self, promotion_type: i8) {
        self.promotion_type = promotion_type;
    }

    pub fn set_to_position(&mut self, to_position: i8) {
        self.to_position = to_position;
    }

    pub fn sum_to_move_worth(&mut self, value: i32) {
        self.move_worth += value
    }
}
