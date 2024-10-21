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