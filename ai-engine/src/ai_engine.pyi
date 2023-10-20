from typing import Union

class PieceMoveDTO:
    from_position: int
    to_position: int
    promotion_type: int
    is_promotion: bool
    is_en_passant: bool

class PieceDTO:
    fen: str
    moves: list[PieceMoveDTO]
    position: int
    white: bool

    def __init__(self, fen: str,
                 moves: list[PieceMoveDTO],
                 position: int,
                 white: bool) -> None:
        pass

class BoardWrapper:
    def get_move_generation_count(self) -> int:
        pass

    def get_ai_move(self) -> (int, PieceMoveDTO):
        pass

    def black_captures_to_fen(self) -> list[str]:
        pass

    def white_captures_to_fen(self) -> list[str]:
        pass

    def get_available_moves(self) -> list[PieceDTO]:
        pass

    def get_winner_fen(self) -> str:
        pass

    def is_white_move(self) -> bool:
        pass

    def load_position(self):
        pass

    def move_piece(self, from_index: int, to_index: int):
        pass