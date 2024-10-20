import React, { useEffect, useState } from "react";

import { PIECE_ICONS } from "./BoardPiece";
import { AIResponse, TBoard, TMove, TColor, TPieceType, TPiece } from "./types";

//@ts-ignore
import captureAudio from "../assets/sound/capture.mp3";
//@ts-ignore
import moveAudio from "../assets/sound/move-self.mp3";

import http from "../http-common";

import "./board.scss";
import { EMPTY_FEN, EMPTY_MOVE, INITIAL_FEN, LINES } from "./constants";
import Logs, { getBoardEvaluationMessage } from "./Logs";
import Cell from "./Cell";

const get_empty_move = (position: number) => {
	const piece: TMove = JSON.parse(JSON.stringify(EMPTY_MOVE));
	// piece.position = position;

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

const INVALID_POSITION = -1;

const getPieceMove = (moves: TMove[], position: number) => {
	return moves.find((move) => move.to === position);
};

const Board = () => {
	const [selectedPiecePos, setSelectedPiecePos] = useState<number>(INVALID_POSITION);
	const [lastMovePos, setLastMovePos] = useState<number>(INVALID_POSITION);
	const [isWaitingForAI, setIsWaitingForAI] = useState(false);
	const [lastAIResponse, setLastAIResponse] = useState<AIResponse>();
	const [board, setBoard] = useState<TBoard>({
		blackCaptures: [],
		blackKingInCheck: false,
		enPassant: 0,
		evaluation: 0,
		fen: INITIAL_FEN,
		whiteCaptures: [],
		pieces: [],
		whiteKingInCheck: false,
		siteToMove: TColor.White,
		winner: "-",
		zobrit: 0,
	});

	const onPieceSelect = (piece: TPiece) => {
		if (isWaitingForAI) {
			return;
		}

		if (board.siteToMove !== piece.color) {
            console.log("LOG: It's not your turn");
			// Play invalid move sound
			return;
		}
		if (selectedPiecePos === piece.position) {
			setSelectedPiecePos(INVALID_POSITION);
		} else {
			if (selectedPiecePos !== INVALID_POSITION) {
				togglePieceAvailableMoves(selectedPiecePos);
			}

			setSelectedPiecePos(piece.position);
		}
		togglePieceAvailableMoves(piece.position);
	};

	const togglePieceAvailableMoves = (position: number) => {
		if (isWaitingForAI) {
			return;
		}

		board.pieces[position].moves.forEach((move) => {
			const to_position = move.to

			const capturedPiece = board.pieces[to_position];

			const className = capturedPiece.type !== TPieceType.Empty ? "capture-receptor" : "empty-receptor";

			const cell = document.querySelector(`.cell[data-pos='${to_position}']`) as HTMLDivElement;

			// cell.onclick = () => onCellClick(cell, move.row, move.column);
			cell.classList.toggle(className);

			const cellPiece = document.querySelector(
				`.cell[data-pos='${to_position}'] button.piece-button`
			) as HTMLDivElement;

			cellPiece?.classList.toggle("disabled");
		});
	};

	const onMovePiece = (cell: HTMLDivElement, cellPosition: number) => {
		// if (isWaitingForAI) {
		// 	return;
		// }

		if (selectedPiecePos !== INVALID_POSITION) {
			let move = getPieceMove(board.pieces[selectedPiecePos].moves, cellPosition);

			if (move === undefined) {
                console.log("Could not find move");
				return;
			}

			// TODO: Add the option to choose the promotion
			// if (move.promotion) {
			// 	move.promotionType = TColor.White | TPieceType.Queen;
			// }

			// const copy_board: TBoard = JSON.parse(JSON.stringify(board));

			// copy_board.moves[position] = get_empty_move(position);

			// selectedPiecePos.position = cellPosition;

			// copy_board.moves[cellPosition] = selectedPiecePos;

			// setBoard(copy_board);
			setSelectedPiecePos(INVALID_POSITION);

			const cellPiece = document.querySelector(
				`.cell[data-pos='${selectedPiecePos}'] button.piece-button.disabled`
			) as HTMLDivElement;

			cellPiece?.classList.remove("disabled");

			// playMoveAudio(move.capture);

			togglePieceAvailableMoves(selectedPiecePos);

			movePiece(move);
			setLastMovePos(move.from);
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

	// const getAiMove = () => {
	// 	setIsWaitingForAI(true);

	// 	http.post<AIResponse>("/ai/move")
	// 		.then((response) => response.data)
	// 		.then((aiResponse) => {
	// 			fetchBoard().then(() => {
	// 				playMoveAudio(aiResponse.aiMove.capture);
    //       setLastMove(aiResponse.aiMove)

	// 				setLastAIResponse(aiResponse);
	// 			});
	// 		})
	// 		.finally(() => {
	// 			setIsWaitingForAI(false);
	// 		});
	// };

	const movePiece = (move: TMove) => {
		// setIsWaitingForAI(true);

		http.post<TBoard>("/board/move/piece", move)
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);

				// getAiMove();
			})
			.catch((err) => {
				console.error(err);
				// setIsWaitingForAI(false);
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

				// if (!data.siteToMove) {
				// 	getAiMove();
				// }

				setLastAIResponse(undefined);
			});
	};

	// const setAITime = (e: React.FormEvent<HTMLFormElement>) => {
	// 	e.preventDefault();
	// 	let aiTimeInput = document.getElementById("ai_time");

	// 	if (!aiTimeInput) {
	// 		return;
	// 	}

	// 	let time = Number((aiTimeInput as HTMLInputElement).value);

	// 	http.post<{ moves: number; elapsedTime: number }>("/ai/time_to_think", {
	// 		time_to_think: time,
	// 	});
	// };

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
				{/* <form method="post" onSubmit={}>
					<input type="number" name="aiTime" id="ai_time" defaultValue={2} />
					<button type="submit" id="set_ai_time_btn">
						Set
					</button>
				</form> */}
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
				{[7, 6, 5, 4, 3, 2, 1, 0].map((i) => (
					<div key={i} className="row">
						{[0, 1, 2, 3, 4, 5, 6, 7].map((j) => {
              				const position = (i * 8) + (7 - j);

							const piece = board.pieces[position];

							return (
								<Cell
                  					key={position}
									board={board}
									column={j}
                  					// lastMove={lastMovePos}
									onClickPiece={onPieceSelect}
									onMovePiece={onMovePiece}
									piece={piece}
									row={i}
									selectedPiecePosition={selectedPiecePos}
								/>
							);
						}).reverse()}
					</div>
				))}
				{board.winner !== "-" && (
					<span id="winner-announcement">
						{getBoardEvaluationMessage(board.evaluation, board.winner)}
					</span>
				)}
			</div>

			<Logs aiResponse={lastAIResponse} board={board} isWaitingForAI={isWaitingForAI} />
		</>
	);
};

export default Board;
