import React from "react";
import { TBoard, TMove, TPiece, TPieceColor, TPieceType } from "./types";
import { COLUMNS, EMPTY_FEN } from "./constants";
import BoardPiece from "./BoardPiece";

interface IProps {
    board: TBoard;
    column: number;
	lastMove: TMove | null;
    onClickPiece: (piece: TPiece) => void;
	onMovePiece: (cell: HTMLDivElement, cellPosition: number) => void;
    piece: TPiece | undefined
    row: number;
    selectedPiecePosition: number | undefined;
}

const Cell: React.FC<IProps> = ({
    board,
    column,
	lastMove,
    onClickPiece,
	onMovePiece,
    piece,
    row,
    selectedPiecePosition
}) => {
	const cellPosition = row * 8 + column;

	const getCellClasses = () => {
		let classes = "cell";
	
		if (selectedPiecePosition === cellPosition) {
			classes += " selected"
		} 
		
		classes += " " + getInCheckClass(
			board.blackKingInCheck,
			piece?.value || 0,
			board.whiteKingInCheck
		)

		if (lastMove?.fromPosition === cellPosition) {
			classes += " from-position"
		} else if (lastMove?.toPosition === cellPosition) {
			classes += " to-position"
		}
	
		return classes;
	}

	return (
		<div
			key={column}
			className={getCellClasses()}
			data-pos={cellPosition}
			onClick={(e) => onMovePiece(e.currentTarget, cellPosition)}
		>
			{/* <span className="cell-index">{i * 8 + column}</span> */}
			{column === 0 && <span className={`row-index ${(row + 1) % 2 === 0 ? "white" : ""}`}>{8 - row}</span>}
			{row === 7 && <span className={`column-index ${(column + 1) % 2 !== 0 ? "white" : ""}`}>{COLUMNS[column]}</span>}
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

const getInCheckClass = (blackKingInCheck: boolean, piece: number, whiteKingInCheck: boolean) => {
    if(piece === (TPieceColor.Black | TPieceType.King) && blackKingInCheck) {
      return 'in-check';
    }
  
    if(piece === (TPieceColor.White | TPieceType.King) && whiteKingInCheck) {
      return 'in-check';
    }
  
    return '';
  }

export default Cell;
