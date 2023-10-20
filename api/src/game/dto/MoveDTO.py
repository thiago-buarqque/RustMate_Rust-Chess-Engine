from ai_engine.ai_engine import PieceMoveDTO

from game.dto.DTO import DTO


class MoveDTO(DTO):
    def __init__(self,
                 from_position: int,
                 is_capture: bool,
                 is_en_passant: bool,
                 is_promotion: bool,
                 piece_value: int,
                 promotion_type: str,
                 to_position: int):
        dict.__init__(self,
                      fromPosition=from_position,
                      isCapture=is_capture,
                      isEnPassant=is_en_passant,
                      isPromotion=is_promotion,
                      pieceValue=piece_value,
                      promotionType=promotion_type,
                      toPosition=to_position)

    @staticmethod
    def from_piece_moveDTO(piece_move: PieceMoveDTO):
        return MoveDTO(
            piece_move.from_position,
            piece_move.is_capture,
            piece_move.is_en_passant,
            piece_move.is_promotion,
            piece_move.piece_value,
            piece_move.promotion_type,
            piece_move.to_position,
        )

    @staticmethod
    def from_dict_piece_moveDTO(piece_move: dict):
        return PieceMoveDTO(
            piece_move['fromPosition'],
            piece_move['isCapture'],
            piece_move['isEnPassant'],
            piece_move['isPromotion'],
            piece_move['pieceValue'],
            piece_move['promotionType'],
            piece_move['toPosition']
        )
