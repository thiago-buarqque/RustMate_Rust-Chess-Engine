export type AIResponse = {
  "depth": number,
  "duration": number,
  "evaluation": number,
  "aiMove": TMove
};

export type TBoard = {
  blackCaptures: string[];
  blackKingInCheck: boolean;
  enPassant: number;
  evaluation: number;
  fen: string;
  moves: TMove[];
  pieces: TPiece[];
  whiteCaptures: string[];
  whiteKingInCheck: boolean;
  siteToMove: TColor;
  winner: "-" | "b" | "w" | "d";
  zobrit: number;
};

export type TPiece = {
  color: TColor;
  fen: string;
  position: number;
  type: TPieceType;
}

export type TMove = {
  _move: number,
  bitboards: number[],
  black_king_moved: false,
  board_en_passant_bb_piece_square: number,
  board_en_passant_bb_position: number,
  castling_rights: number,
  color: TColor,
  en_passant_bb_piece_square: number,
  en_passant_bb_position: number,
  piece_type: TPieceType,
  white_king_moved: false,
  zobrist_hash: number
}

export enum TColor {
  Black = "Black",
  White = "White",
}

export enum TPieceType {
  Empty = "Empty",
  Bishop = "Bishop",
  King = "King",
  Knight = "Knight",
  Pawn = "Pawn",
  Queen = "Queen",
  Rook = "Rook",
}
