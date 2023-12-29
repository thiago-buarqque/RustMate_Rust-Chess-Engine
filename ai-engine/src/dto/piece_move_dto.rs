#[derive(Debug, Clone)]
pub struct PieceMoveDTO {
    pub from_position: i8,
    pub is_capture: bool,
    pub is_en_passant: bool,
    pub is_promotion: bool,
    pub piece_value: i8,
    pub promotion_type: char,
    pub to_position: i8,
}

impl PieceMoveDTO {
    pub fn new(
        from_position: i8,
        is_capture: bool,
        is_en_passant: bool,
        is_promotion: bool,
        piece_value: i8,
        promotion_type: char,
        to_position: i8,
    ) -> Self {
        Self {
            from_position,
            is_capture,
            is_en_passant,
            is_promotion,
            piece_value,
            promotion_type,
            to_position,
        }
    }
}
