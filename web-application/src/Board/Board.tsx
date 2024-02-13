import React, { MouseEventHandler, useEffect, useRef, useState } from "react";

import BoardPiece, { PIECE_ICONS } from "./BoardPiece";
import { TBoard, TMove, TPiece, TPieceColor, TPieceType } from "./types";

//@ts-ignore
import captureAudio from "../assets/sound/capture.mp3";
//@ts-ignore
import moveAudio from "../assets/sound/move-self.mp3";

import http from "../http-common";

import "./board.scss";
import { EMPTY_FEN, INITIAL_FEN } from "./constants";

const LINES = [0, 1, 2, 3, 4, 5, 6, 7];
const COLUMNS: { [key: number]: string } = {
  0: "A",
  1: "B",
  2: "C",
  3: "D",
  4: "E",
  5: "F",
  6: "G",
  7: "H",
};

const EMPTY_PIECE: TPiece = {
  moves: [],
  position: -1,
  fen: null,
  white: false,
};

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

const notAnAvailableMove = (availableMoves: TMove[], position: number) => {
  return availableMoves.find((move) => move.toPosition === position);
};

const Board = () => {
  const [selectedPiece, setSelectedPiece] = useState<TPiece | null>(null);
  const [isAlreadyMoving, setIsAlreadyMoving] = useState(false);
  const [board, setBoard] = useState<TBoard>({
    blackCaptures: [],
    boardEvaluation: 0,
    boardFen: INITIAL_FEN,
    whiteCaptures: [],
    pieces: [],
    whiteMove: true,
    winner: "-",
    zobrit: 0
  });

  const onPieceSelect = (piece: TPiece) => {
    if(isAlreadyMoving) {
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
    if(isAlreadyMoving) {
      return;
    }

    piece.moves.forEach((move) => {
      const className =
        board.pieces[move.toPosition].fen !== EMPTY_FEN ? "capture-receptor" : "empty-receptor";

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
    if(isAlreadyMoving) {
      return;
    }

    if (selectedPiece) {
      const { position, moves } = selectedPiece;

      let pieceMove = notAnAvailableMove(moves, cellPosition);

      if (pieceMove === undefined) {
        return;
      }

      if (pieceMove.isPromotion) {
        console.log("This is a promoting pawn!");
      }

      const copy_board: TBoard = JSON.parse(JSON.stringify(board));

      let capture = false;
      if (copy_board.pieces[cellPosition].fen !== null) {
        capture = true;
      }

      copy_board.pieces[position] = get_empty_piece(position);

      selectedPiece.position = cellPosition;

      copy_board.pieces[cellPosition] = selectedPiece;

      setSelectedPiece(null);
      setBoard(copy_board);

      const cellPiece = document.querySelector(
        `.cell[data-pos='${position}'] button.piece-button.disabled`
      ) as HTMLDivElement;

      cellPiece?.classList.remove("disabled");

      playMoveAudio(capture);

      // console.log(`Capture`, capture);
      // if (capture) {
      //   console.log(cell);
      //   cell.classList.remove("capture-receptor");
      // }

      togglePieceAvailableMoves(selectedPiece);
      // add loading before sending request
      movePiece(pieceMove);
      // currentTarget.onclick = null;
      // send request to server and update the state with the result
    }
  };

  const movePiece = (pieceMove: TMove) => {
    setIsAlreadyMoving(true);
    pieceMove.promotionType = TPieceColor.White | 5 // Queen
    
    http
      .post<TBoard>("/board/move/piece", pieceMove)
      .then((response) => response.data)
      .then((data) => {
        setBoard(data);
        playMoveAudio(false);
      })
      .finally(() => {
        setIsAlreadyMoving(false);
      })
  };

  const resetBoard = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const inputFen: HTMLInputElement | null = document.getElementById(
      "input-fen"
    ) as HTMLInputElement;

    if (!inputFen) return;

    let fen = INITIAL_FEN;
    if (inputFen.value.trim() !== "") {
      fen = inputFen.value.trim();
    }

    http
      .post<TBoard>("/board/load/fen", {
        fen,
      })
      .then((response) => response.data)
      .then((data) => {
        setBoard(data);
      });
  };

  const fetchCountMoves = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    let depthInput = document.getElementById("raw_search_depth");

    if (!depthInput) {
      return;
    }

    let depth = Number((depthInput as HTMLInputElement).value);

    http
      .post<{ moves: number, elapsedTime: number }>("/board/moves/count", {
        depth,
      })
      .then((response) => response.data)
      .then((data) => {
        console.log(`Evaluated ${data.moves} in ${data.elapsedTime}ms for a depth of ${depth}`);
      });
  };

  const setAITime = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    let aiTimeInput = document.getElementById("ai_time");

    if (!aiTimeInput) {
      return;
    }

    let time = Number((aiTimeInput as HTMLInputElement).value);

    http
      .post<{ moves: number, elapsedTime: number }>("/ai/time_to_think", {
        time_to_think: time,
      });
  };

  useEffect(() => {
    http
      .get<TBoard>("/board")
      .then((response) => response.data)
      .then((data) => {        
        setBoard(data);
      });
  }, []);

  useEffect(() => {
    if (board.winner !== "-") {
      if (board.winner === "d") {
        alert("Draw");
      } else {
        alert(board.winner === "w" ? `Humano venceu!` : `IA venceu!`);
      }
    }
  }, [board]);

  return (
    <>
      <div id="floating-forms">
        <form method="post" onSubmit={resetBoard}>
          <input type="text" name="fen" id="input-fen" />
          <button type="submit" id="reset-btn">
            Load FEN
          </button>
        </form>
        <form method="post" onSubmit={fetchCountMoves}>
          <input type="number" name="rawDepth" id="raw_search_depth" />
          <button type="submit" id="count_moves_btn">
            Count
          </button>
        </form>
        <form method="post" onSubmit={setAITime}>
          <input type="number" name="aiTime" id="ai_time" defaultValue={5}/>
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
            {LINES.map((j) => (
              <div
                key={j}
                className={`cell${selectedPiece?.position === (i * 8 + j) ? " selected" : ""}`}
                data-pos={i * 8 + j}
                onClick={(e) => onMovePiece(e.currentTarget, i * 8 + j)}
              >
                {/* <span className="cell-index">{i * 8 + j}</span> */}
                {j === 0 && (
                  <span className={`row-index ${(i + 1) % 2 === 0 ? "white" : ""}`}>{8 - i}</span>
                )}
                {i === 7 && (
                  <span className={`column-index ${(j + 1) % 2 !== 0 ? "white" : ""}`}>
                    {COLUMNS[j]}
                  </span>
                )}
                {board.pieces[i * 8 + j] && board.pieces[i * 8 + j].fen !== EMPTY_FEN ? (
                  <BoardPiece boardPiece={board.pieces[i * 8 + j]} onClick={onPieceSelect} />
                ) : (
                  <div className="move-dot"></div>
                )}
              </div>
            ))}
          </div>
        ))}
        {/* <span id="board-fen">{board.boardFen}</span> */}
        <span id="winner-announcement">{getBoardEvaluationMessage(board.boardEvaluation) + ` (${board.boardEvaluation.toFixed(0)})`}</span>
        {/* <span id="zobrit">{getZobritBinary(board.zobrit)}</span> */}
      </div>
    </>
  );
};

const getBoardEvaluationMessage = (boardEvaluation: number) => {
  if(boardEvaluation === 0) {
    return "No one is winning";
  } else if(boardEvaluation < 0 && boardEvaluation >= -100) {
    return "Black is slightly better"
  } else if(boardEvaluation < -100) {
    return "Black is winning"
  } else if(boardEvaluation > 0 && boardEvaluation <= 100) {
    return "White is slightly better"
  } else {
    return "White is winning"
  }
}

const getZobritBinary = (zobrit: number) => {
  let binary = zobrit.toString(2)

  while(binary.length < 64) {
    binary = "0" + binary;
  }

  return binary.slice(0, 16) + " " + binary.slice(16, 32) + "\n" + binary.slice(32, 48) + " " + binary.slice(48, 64);
}

export default Board;
