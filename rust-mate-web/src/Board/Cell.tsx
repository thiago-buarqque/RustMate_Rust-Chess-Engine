import React from "react";
import { TBoard, TPiece } from "./types";
import { COLUMNS, EMPTY_FEN } from "./constants";
import BoardPiece from "./BoardPiece";

interface IProps {
    board: TBoard;
    column: number;
    onClickPiece: (piece: TPiece) => void;
	  onMovePiece: (cell: HTMLDivElement, position: number) => void;
    piece: TPiece | undefined
    position: number;
    row: number;
    selectedPiecePosition: number | undefined;
}

const Cell: React.FC<IProps> = ({
    board,
    column,
    onClickPiece,
	  onMovePiece,
    piece,
    position,
    row,
    selectedPiecePosition
}) => {
	const getCellClasses = () => {
		let classes = "cell";
	
		if (selectedPiecePosition === position) {
			classes += " selected"
		} 

		return classes;
	}

	return (
		<div
			key={column}
			className={getCellClasses()}
			data-pos={position}
			onClick={(e) => onMovePiece(e.currentTarget, position)}
		>
			{/* <span className="cell-index">{position}</span> */}
			{column === 7 && <span className={`row-index ${(row) % 2 === 0 ? "white" : ""}`}>{row + 1}</span>}
			{row === 0 && <span className={`column-index ${(column ) % 2 !== 0 ? "white" : ""}`}>{COLUMNS[7 - column]}</span>}
			{piece && piece.fen !== EMPTY_FEN ? (
				<BoardPiece
					blackKingInCheck={board.blackKingInCheck}
					boardPiece={piece}
					onClick={onClickPiece}
					whiteKingInCheck={board.whiteKingInCheck}
				/>
			) : (
				<div className="move-dot"></div>
			)}
		</div>
	);
};

// const getInCheckClass = (blackKingInCheck: boolean, piece: number, whiteKingInCheck: boolean) => {
//     if(piece === (TColor.Black | TPieceType.King) && blackKingInCheck) {
//       return 'in-check';
//     }
  
//     if(piece === (TColor.White | TPieceType.King) && whiteKingInCheck) {
//       return 'in-check';
//     }
  
//     return '';
//   }

export default Cell;
