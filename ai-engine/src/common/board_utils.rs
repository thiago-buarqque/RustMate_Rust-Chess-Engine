use super::piece_move::PieceMove;

#[inline]
pub fn get_position_line_number(position: i8) -> usize {
    (8 - ((position - (position % 8)) / 8)) as usize
}

#[inline]
pub fn get_position_column_number(position: i8) -> usize {
    (position - (position - (position % 8))) as usize
}

pub fn get_position_notation(position: i8) -> String {
    let columns = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let line = get_position_line_number(position);
    let column = get_position_column_number(position);

    format!("{}{}", columns[column], line)
}

pub fn get_move_notation(piece_move: &PieceMove) -> String {
    // Unfinished

    let from_position_str = get_position_notation(piece_move.get_from_position());
    let to_position_str = get_position_notation(piece_move.get_to_position());

    format!("{}{}", from_position_str, to_position_str)
}
