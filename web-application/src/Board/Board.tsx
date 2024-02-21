import React, { useEffect, useState } from "react";

import { PIECE_ICONS } from "./BoardPiece";
import { AIResponse, TBoard, TMove, TPiece, TPieceColor, TPieceType } from "./types";

//@ts-ignore
import captureAudio from "../assets/sound/capture.mp3";
//@ts-ignore
import moveAudio from "../assets/sound/move-self.mp3";

import http from "../http-common";

import "./board.scss";
import { EMPTY_FEN, EMPTY_PIECE, INITIAL_FEN, LINES } from "./constants";
import Logs, { getBoardEvaluationMessage } from "./Logs";
import Cell from "./Cell";

const get_empty_piece = (position: number) => {
	const piece: TPiece = JSON.parse(JSON.stringify(EMPTY_PIECE));
	piece.position = position;

	return piece;
};

const playMoveAudio = (capture: boolean) => {
	let audio;

	if (capture) {
		audio = new Audio(captureAudio);
	} else {
		audio = new Audio(moveAudio);
	}
	audio.play();
};

const getPieceMove = (availableMoves: TMove[], position: number) => {
	return availableMoves.find((move) => move.toPosition === position);
};

const Board = () => {
	const [selectedPiece, setSelectedPiece] = useState<TPiece | null>(null);
	const [lastMove, setLastMove] = useState<TMove | null>(null);
	const [isWaitingForAI, setIsWaitingForAI] = useState(false);
	const [lastAIResponse, setLastAIResponse] = useState<AIResponse>();
	const [board, setBoard] = useState<TBoard>({
		blackCaptures: [],
		blackKingInCheck: false,
		boardEvaluation: 0,
		boardFen: INITIAL_FEN,
		whiteCaptures: [],
		pieces: [],
		whiteKingInCheck: false,
		whiteMove: true,
		winner: "-",
		zobrit: 0,
	});

	const onPieceSelect = (piece: TPiece) => {
		if (isWaitingForAI) {
			return;
		}

		if (board.whiteMove !== piece.white) {
			// Play invalid move sound
			return;
		}
		if (selectedPiece === piece) {
			setSelectedPiece(null);
		} else {
			if (selectedPiece) {
				togglePieceAvailableMoves(selectedPiece);
			}

			setSelectedPiece(piece);
		}
		togglePieceAvailableMoves(piece);
	};

	const togglePieceAvailableMoves = (piece: TPiece) => {
		if (isWaitingForAI) {
			return;
		}

		piece.moves.forEach((move) => {
			const className = board.pieces[move.toPosition].fen !== EMPTY_FEN ? "capture-receptor" : "empty-receptor";

			const cell = document.querySelector(`.cell[data-pos='${move.toPosition}']`) as HTMLDivElement;

			// cell.onclick = () => onCellClick(cell, move.row, move.column);
			cell.classList.toggle(className);

			const cellPiece = document.querySelector(
				`.cell[data-pos='${move.toPosition}'] button.piece-button`
			) as HTMLDivElement;

			cellPiece?.classList.toggle("disabled");
		});
	};

	const onMovePiece = (cell: HTMLDivElement, cellPosition: number) => {
		if (isWaitingForAI) {
			return;
		}

		if (selectedPiece) {
			const { position, moves } = selectedPiece;

			let pieceMove = getPieceMove(moves, cellPosition);

			if (pieceMove === undefined) {
				return;
			}

			// TODO: Add the option to choose the promotion
			if (pieceMove.promotion) {
				pieceMove.promotionType = TPieceColor.White | TPieceType.Queen;
			}

			const copy_board: TBoard = JSON.parse(JSON.stringify(board));

			copy_board.pieces[position] = get_empty_piece(position);

			selectedPiece.position = cellPosition;

			copy_board.pieces[cellPosition] = selectedPiece;

			setSelectedPiece(null);
			setBoard(copy_board);

			const cellPiece = document.querySelector(
				`.cell[data-pos='${position}'] button.piece-button.disabled`
			) as HTMLDivElement;

			cellPiece?.classList.remove("disabled");

			playMoveAudio(pieceMove.capture);

			togglePieceAvailableMoves(selectedPiece);

			movePiece(pieceMove);
			setLastMove(pieceMove);
		}
	};

	const fetchBoard = async () => {
		return http
			.get<TBoard>("/board")
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);
			});
	};

	const getAiMove = () => {
		setIsWaitingForAI(true);

		http.post<AIResponse>("/ai/move")
			.then((response) => response.data)
			.then((aiResponse) => {
				fetchBoard().then(() => {
					playMoveAudio(aiResponse.aiMove.capture);
          setLastMove(aiResponse.aiMove)

					setLastAIResponse(aiResponse);
				});
			})
			.finally(() => {
				setIsWaitingForAI(false);
			});
	};

	const movePiece = (pieceMove: TMove) => {
		setIsWaitingForAI(true);

		http.post<TBoard>("/board/move/piece", pieceMove)
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);

				getAiMove();
			})
			.catch((err) => {
				console.error(err);
				setIsWaitingForAI(false);
			});
	};

	const loadFEN = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();

		const inputFen: HTMLInputElement | null = document.getElementById("input-fen") as HTMLInputElement;

		if (!inputFen) return;

		let fen = INITIAL_FEN;
		if (inputFen.value.trim() !== "") {
			fen = inputFen.value.trim();
		}

		http.post<TBoard>("/board/load/fen", {
			fen,
		})
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);

				if (!data.whiteMove) {
					getAiMove();
				}

				setLastAIResponse(undefined);
			});
	};

	const setAITime = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();
		let aiTimeInput = document.getElementById("ai_time");

		if (!aiTimeInput) {
			return;
		}

		let time = Number((aiTimeInput as HTMLInputElement).value);

		http.post<{ moves: number; elapsedTime: number }>("/ai/time_to_think", {
			time_to_think: time,
		});
	};

	useEffect(() => {
		fetchBoard();
	}, []);

	return (
		<>
			<div id="floating-forms">
				<form method="post" onSubmit={loadFEN}>
					<input type="text" name="fen" id="input-fen" />
					<button type="submit" id="reset-btn">
						Load FEN
					</button>
				</form>
				{/* <form method="post" onSubmit={fetchCountMoves}>
          <input type="number" name="rawDepth" id="raw_search_depth" />
          <button type="submit" id="count_moves_btn">
            Count
          </button>
        </form>*/}
				<form method="post" onSubmit={setAITime}>
					<input type="number" name="aiTime" id="ai_time" defaultValue={2} />
					<button type="submit" id="set_ai_time_btn">
						Set
					</button>
				</form>
			</div>

			<div id="board">
				<div id="white-captures" className="captures">
					{board.whiteCaptures.map((piece_fen, i) => (
						<img key={i} className="captured_piece" src={PIECE_ICONS[piece_fen]} alt={piece_fen} />
					))}
				</div>
				<div id="black-captures" className="captures">
					{board.blackCaptures.map((piece_fen, i) => (
						<img key={i} className="captured_piece" src={PIECE_ICONS[piece_fen]} alt={piece_fen} />
					))}
				</div>
				{LINES.map((i) => (
					<div key={i} className="row">
						{LINES.map((j) => {
              const position = i * 8 + j;

							const piece = board.pieces[position];

							return (
								<Cell
                  key={position}
									board={board}
									column={j}
                  lastMove={lastMove}
									onClickPiece={onPieceSelect}
									onMovePiece={onMovePiece}
									piece={piece}
									row={i}
									selectedPiecePosition={selectedPiece?.position}
								/>
							);
						})}
					</div>
				))}
				{board.winner !== "-" && (
					<span id="winner-announcement">
						{getBoardEvaluationMessage(board.boardEvaluation, board.winner)}
					</span>
				)}
			</div>

			<Logs aiResponse={lastAIResponse} board={board} isWaitingForAI={isWaitingForAI} />
		</>
	);
};

export default Board;
