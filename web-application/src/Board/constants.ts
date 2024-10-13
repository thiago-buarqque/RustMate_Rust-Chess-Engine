import { TColor, TMove, TPieceType } from "./types";

export const INITIAL_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

export const EMPTY_FEN = "."

export const LINES = [7, 6, 5, 4, 3, 2, 1, 0];
export const COLUMNS: { [key: number]: string } = {
  0: "A",
  1: "B",
  2: "C",
  3: "D",
  4: "E",
  5: "F",
  6: "G",
  7: "H",
};

export const EMPTY_MOVE: TMove = {
  _move: 0,
  bitboards: [],
  black_king_moved: false,
  board_en_passant_bb_piece_square: 0,
  board_en_passant_bb_position: 0,
  castling_rights: 0,
  color: TColor.Black,
  en_passant_bb_piece_square: 0,
  en_passant_bb_position: 0,
  piece_type: TPieceType.Empty,
  white_king_moved: false,
  zobrist_hash: 0
};