import os
import time
import sys

from flask import request

from flask_app import app, to_json_response

from ai_engine.ai_engine import BoardWrapper, PieceMoveDTO

from game.dto.BoardDTO import BoardDTO
from game.dto.MoveDTO import MoveDTO

module_path = os.path.abspath(os.path.join("../../"))

if module_path not in sys.path:
    sys.path.append(module_path)

board = BoardWrapper()


@app.route('/board/', methods=['GET'])
def get_board():
    return to_json_response(BoardDTO(board))


@app.route('/board/moves/count', methods=['POST'])
def get_move_count():
    depth = request.json["depth"]

    start = time.time()
    states = board.get_move_generation_count(int(depth))
    end = time.time()
    print(f"Elapsed time: {(end - start) * 1000}")

    return to_json_response({
        "moves": states
    })


@app.route('/board/move/piece', methods=['POST'])
def move_piece():
    board.move_piece(MoveDTO.from_dict_piece_moveDTO(request.json))

    # Evaluated 55593 states
    # Time elapsed is: 1.940869201s
    # Elapsed time: 1.9409072399139404
    # -72.0, (1, 18)

    # Evaluated 84521 states
    # Time elapsed is: 3.140024914s
    # Elapsed time: 3.1400742530822754
    # -101.80000305175781, (12, 20)

    # Ai move
    if board.get_winner_fen() == "-":
        start = time.time()
        move_value, move = \
            board.get_ai_move(5)

        end = time.time()
        print(f"Elapsed time: {(end - start)}")
        print(f"{move_value}, "
              f"({move.from_position}, {move.to_position})")

        if move.from_position != -1 and move.to_position != -1:
            board.move_piece(move)

    return get_board()


@app.route('/board/load/fen', methods=['POST'])
def reset_board():
    board.load_position(request.json["fen"])

    return get_board()


if __name__ == '__main__':
    app.run(debug=True, use_debugger=True, use_reloader=False, host='0.0.0.0',
            port=8000)
