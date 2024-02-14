export type AIResponse = {
  "depth": number,
  "duration": number,
  "evaluation": number,
  "aiMove": TMove
};

export type TBoard = {
  blackCaptures: string[];
  blackKingInCheck: boolean;
  boardEvaluation: number;
  boardFen: string;
  pieces: TPiece[];
  whiteCaptures: string[];
  whiteKingInCheck: boolean;
  whiteMove: boolean;
  winner: "-" | "b" | "w" | "d";
  zobrit: number;
};

export type TPiece = {
  fen: string | null;
  moves: TMove[];
  position: number;
  value: number;
  white: boolean;
};

export type TMove = {
  capture: boolean;
  castle: boolean;
  fromPosition: number;
  enPassant: boolean;
  notation: string;
  moveWorth: number;
  piece_value: number;
  promotion: boolean;
  promotionType: number;
  toPosition: number;
};

export enum TPieceColor {
  Black = 8,
  White = 16,
}

export enum TPieceType {
  Empty = 0,
  Bishop = 1,
  King = 2,
  Knight = 3,
  Pawn = 4,
  Queen = 5,
  Rook = 6,
}
