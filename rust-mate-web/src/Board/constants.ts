import { TColor, TMove, TPieceType } from "./types";

export const INITIAL_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

export const EMPTY_FEN = "."

export const LINES = [0, 1, 2, 3, 4, 5, 6, 7];
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
    flags: -1,
    from: -1,
    to: -1
};

export const NORMAL: number = 0x0;
export const DOUBLE_PAWN_PUSH: number = 0x1;
export const KING_CASTLE: number = 0x2;
export const QUEEN_CASTLE: number = 0x3;
export const CAPTURE: number = 0x4;
export const EN_PASSANT: number = 0x5;
export const KNIGHT_PROMOTION: number = 0x8;
export const BISHOP_PROMOTION: number = 0x9;
export const ROOK_PROMOTION: number = 0xA;
export const QUEEN_PROMOTION: number = 0xB;
export const KNIGHT_PROMOTION_CAPTURE: number = 0xC;
export const BISHOP_PROMOTION_CAPTURE: number = 0xD;
export const ROOK_PROMOTION_CAPTURE: number = 0xE;
export const QUEEN_PROMOTION_CAPTURE: number = 0xF;
