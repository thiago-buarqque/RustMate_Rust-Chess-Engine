from ai_engine import BoardWrapper

from game.dto.BoardPieceDTO import BoardPieceDTO
from game.dto.DTO import DTO


class BoardDTO(DTO):
    def __init__(self, board: BoardWrapper):
        pieces = board.get_available_moves()

        dict.__init__(self,
                      blackEnPassant=board.get_black_en_passant(),
                      blackCaptures=board.black_captures_to_fen(),
                      pieces=[BoardPieceDTO.from_pieceDTO(piece) for piece in pieces],
                      whiteCaptures=board.white_captures_to_fen(),
                      whiteEnPassant=board.get_white_en_passant(),
                      whiteMove=board.is_white_move(),
                      winner=board.get_winner_fen(),
                      zobrit=board.get_zobrist_hash()
                      )
