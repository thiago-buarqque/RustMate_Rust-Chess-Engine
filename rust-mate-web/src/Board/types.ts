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
  moves: TMove[];
  position: number;
  type: TPieceType;
}

export type TMove = {
    flags: number,
    from: number,
    to: number
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
