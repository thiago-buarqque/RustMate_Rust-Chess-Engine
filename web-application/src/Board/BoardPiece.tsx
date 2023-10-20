import React from "react";

import BishopBlack from "../assets/bishop-black.svg";
import BishopWhite from "../assets/bishop-white.svg";
import KingBlack from "../assets/king-black.svg";
import KingWhite from "../assets/king-white.svg";
import KnightBlack from "../assets/knight-black.svg";
import KnightWhite from "../assets/knight-white.svg";
import PawnBlack from "../assets/pawn-black.svg";
import PawnWhite from "../assets/pawn-white.svg";
import QueenBlack from "../assets/queen-black.svg";
import QueenWhite from "../assets/queen-white.svg";
import RookBlack from "../assets/rook-black.svg";
import RookWhite from "../assets/rook-white.svg";

import { TPiece} from "./types";

export const PIECE_ICONS: {
  [key: string]: string;
} = {
  "b": BishopBlack,
  "B": BishopWhite,
  "k": KingBlack,
  "K": KingWhite,
  "n": KnightBlack,
  "N": KnightWhite,
  "p": PawnBlack,
  "P": PawnWhite,
  "q": QueenBlack,
  "Q": QueenWhite,
  "r": RookBlack,
  "R": RookWhite,
};

interface IProps {
  boardPiece: TPiece;
  onClick: (position: TPiece) => void;
}

const BoardPiece: React.FC<IProps> = ({ boardPiece, onClick }) => {
  const { fen: type } = boardPiece;

  return (
    <button
      className="piece-button"
      onClick={(e) => {
        if (!e.currentTarget.classList.contains("disabled")) {
          onClick(boardPiece);
        }
      }}
    >
      {type && <img className="piece" src={PIECE_ICONS[type]} alt={type} />}
    </button>
  );
};

export default BoardPiece;
