import React from 'react'
import { AIResponse, TBoard } from './types'

import './logs.scss';

interface IProps {
    aiResponse: AIResponse | undefined,
    board: TBoard,
    isWaitingForAI: boolean
}

const Logs: React.FC<IProps> = ({aiResponse, board, isWaitingForAI}) => {
  return (
    <div id='logs'>
        <div id="board-info">
                <h1>Board:</h1>
                <span className='log'><strong>Evaluation:</strong> {board.boardEvaluation.toFixed(1)} ({getBoardEvaluationMessage(board.boardEvaluation, board.winner)})</span>
                <span className='log'><strong>Position hash:</strong> {getZobritBinary(board.zobrit)}</span>
                <span className='log'><strong>Black in check:</strong> {board.blackKingInCheck.toString()}</span>
                <span className='log'><strong>White in check:</strong> {board.whiteKingInCheck.toString()}</span>
            </div>
        <hr></hr>
        {
            (aiResponse && !isWaitingForAI) &&
            <div id="ai-move-info">
                <h1>AI move:</h1>
                <span className='log'><strong>Depth:</strong> {aiResponse.depth}</span>
                <span className='log'><strong>Duration:</strong> {aiResponse.duration}ms</span>
                <span className='log'><strong>Evaluation:</strong> {aiResponse.evaluation.toFixed(1)}</span>
                <span className='log'><strong>Best move:</strong> {aiResponse.aiMove.notation}</span>
            </div>
        }

        {
            isWaitingForAI && <h1>AI is thinking...</h1>
        }
    </div>
  )
}

const getZobritBinary = (zobrit: number) => {
    let binary = zobrit.toString(2)
  
    while(binary.length < 64) {
      binary = "0" + binary;
    }
  
    return binary.slice(0, 16) + " " + binary.slice(16, 32) + "\n" + binary.slice(32, 48) + " " + binary.slice(48, 64);
  }

export const getBoardEvaluationMessage = (boardEvaluation: number, winner: "-" | "b" | "w" | "d") => {
    if(winner === 'd') {
        return "Draw";
    } else if(winner === 'b') {
        return "Black won!"
    } else if (winner === 'w') {
        return "White won!"
    }

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

export default Logs