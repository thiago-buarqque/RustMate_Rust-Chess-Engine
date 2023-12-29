export type TBoard = {
  blackCaptures: string[];
  pieces: TPiece[];
  whiteCaptures: string[];
  whiteMove: boolean;
  winner: "-" | "b" | "w" | "d";
  zobrit: number;
};

export type TPiece = {
  fen: string | null;
  moves: TMove[];
  position: number;
  white: boolean;
};

export type TMove = {
  fromPosition: number;
  isEnPassant: boolean;
  piece_value: number;
  promotionType: number;
  isPromotion: boolean;
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
