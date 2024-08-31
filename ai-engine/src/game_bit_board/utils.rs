use std::{collections::HashMap, mem::size_of};

use crate::game_bit_board::enums::Color;

use super::{enums::PieceType, positions::BBPositions};

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
