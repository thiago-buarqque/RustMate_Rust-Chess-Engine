use std::{collections::HashMap, mem, mem::size_of};

use crate::game_bit_board::{
    enums::{Color, PieceType},
    positions::BBPositions,
};

pub fn algebraic_to_square(algebraic: &str) -> usize {
    let file = algebraic.chars().next().unwrap() as u16 - b'a' as u16;
    let rank = algebraic.chars().nth(1).unwrap().to_digit(10).unwrap() as u16 - 1;
    (rank * 8 + file) as usize
}

pub fn square_to_algebraic(square: usize) -> String {
    let file = (square % 8) as u8; // Calculate file index (0 = a, 7 = h)
    let rank = (square / 8) as u8; // Calculate rank index (0 = 1, 7 = 8)

    let algebraic_file = (b'a' + file) as char; // Convert file index to letter
    let algebraic_rank = (b'1' + rank) as char; // Convert rank index to number

    // Concatenate to form the algebraic notation
    format!("{}{}", algebraic_file, algebraic_rank)
}

pub fn get_piece_symbol(color: Color, piece_type: PieceType) -> String {
    (match (color, piece_type) {
        (Color::Black, PieceType::Bishop) => "♝",
        (Color::Black, PieceType::King) => "♚",
        (Color::Black, PieceType::Knight) => "♞",
        (Color::Black, PieceType::Pawn) => "♟",
        (Color::Black, PieceType::Queen) => "♛",
        (Color::Black, PieceType::Rook) => "♜",
        (Color::White, PieceType::Bishop) => "♗",
        (Color::White, PieceType::King) => "♔",
        (Color::White, PieceType::Knight) => "♘",
        (Color::White, PieceType::Pawn) => "♙",
        (Color::White, PieceType::Queen) => "♕",
        (Color::White, PieceType::Rook) => "♖",
        _ => ".",
    })
    .to_string()
}

pub fn get_piece_letter(color: Color, piece_type: PieceType) -> String {
    (match (color, piece_type) {
        (Color::Black, PieceType::Bishop) => "b",
        (Color::Black, PieceType::King) => "k",
        (Color::Black, PieceType::Knight) => "n",
        (Color::Black, PieceType::Pawn) => "p",
        (Color::Black, PieceType::Queen) => "q",
        (Color::Black, PieceType::Rook) => "r",
        (Color::White, PieceType::Bishop) => "B",
        (Color::White, PieceType::King) => "K",
        (Color::White, PieceType::Knight) => "N",
        (Color::White, PieceType::Pawn) => "P",
        (Color::White, PieceType::Queen) => "Q",
        (Color::White, PieceType::Rook) => "R",
        _ => ".",
    })
    .to_string()
}

pub fn get_piece_color_and_type_from_symbol(symbol: char) -> (Color, PieceType) {
    match symbol {
        'b' => (Color::Black, PieceType::Bishop),
        'k' => (Color::Black, PieceType::King),
        'n' => (Color::Black, PieceType::Knight),
        'p' => (Color::Black, PieceType::Pawn),
        'q' => (Color::Black, PieceType::Queen),
        'r' => (Color::Black, PieceType::Rook),
        'B' => (Color::White, PieceType::Bishop),
        'K' => (Color::White, PieceType::King),
        'N' => (Color::White, PieceType::Knight),
        'P' => (Color::White, PieceType::Pawn),
        'Q' => (Color::White, PieceType::Queen),
        'R' => (Color::White, PieceType::Rook),
        _ => (Color::Black, PieceType::Empty),
    }
}

pub fn is_pawn_in_initial_position(position: u64, white: bool) -> bool {
    (BBPositions::ROW_2.contains(&position) && white)
        || (BBPositions::ROW_7.contains(&position) && !white)
}

pub fn memory_usage_in_kb(map: &HashMap<(u8, u64), u64>) -> usize {
    // Calculate the size of one entry
    let entry_size = size_of::<(u8, u64)>() + size_of::<u64>();

    // Number of elements
    let num_elements = map.len();

    // Estimate the overhead (this is a rough estimate, depending on the capacity)
    // Assume the overhead is proportional to the capacity
    let overhead = map.capacity() * size_of::<(u8, u64)>() / 2;

    // Calculate the total memory usage
    let total_memory = num_elements * entry_size + overhead;

    // Convert to kilobytes
    total_memory / 1024
}

pub fn estimate_memory_usage_in_bytes<T>(vec_capacity: usize) -> u64 {
    // Metadata size (24 bytes on a 64-bit system)
    let metadata_size = mem::size_of::<Vec<T>>();

    // Size of elements in the vector
    let data_size = vec_capacity * mem::size_of::<T>();

    // Total size in bytes
    let total_size_bytes = metadata_size + data_size;

    total_size_bytes as u64
}
