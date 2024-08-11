use crate::game_bit_board::enums::Color;

use super::{enums::PieceType, positions::{ROW_2, ROW_7}};

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
      _ => "."
  }).to_string()
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
      _ => "."
  }).to_string()
}

pub fn is_pawn_in_initial_position(position: u64, white: bool) -> bool {
  (ROW_2.contains(&position) && white) || (ROW_7.contains(&position) && !white)
}