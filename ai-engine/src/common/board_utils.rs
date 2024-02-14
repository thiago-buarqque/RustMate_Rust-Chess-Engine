use super::{piece_move::PieceMove, piece_utils::{get_piece_symbol, is_piece_of_type}};

#[inline]
pub fn get_position_rank(position: i8) -> usize {
    (8 - ((position - (position % 8)) / 8)) as usize
}

#[inline]
pub fn get_position_column(position: i8) -> usize {
    (position - (position - (position % 8))) as usize
}

#[inline]
pub fn get_position_file(position: i8) -> char {
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][get_position_column(position)]
}

pub fn get_position_notation(position: i8) -> String {
    format!("{}{}", get_position_file(position), get_position_rank(position))
}

pub fn get_move_notation(piece_move: &PieceMove) -> String {
    let mut result = String::new();

    result.push(get_piece_symbol(piece_move.get_piece_value()));

    if !is_piece_of_type(piece_move.get_piece_value(), super::enums::PieceType::Pawn) {
        let from_position_str = get_position_notation(piece_move.get_from_position());
    
        result.push_str(&from_position_str);
    }

    if piece_move.is_capture() {
        if is_piece_of_type(piece_move.get_piece_value(), super::enums::PieceType::Pawn) {
            result.push(get_position_file(piece_move.get_from_position()));
        }

        result.push_str("x");
    }

    let to_position_str = get_position_notation(piece_move.get_to_position());

    result.push_str(&to_position_str);

    if piece_move.is_en_passant() {
        result.push_str(" e.p");
    }

    if piece_move.is_promotion() {
        result.push(get_piece_symbol(piece_move.get_promotion_value()));
    }

    if piece_move.puts_king_in_check() {
        // println!("Colocando a cruz {}->{}", piece_move.get_from_position(), piece_move.get_to_position());
        result.push('+');
    }

    result
}
