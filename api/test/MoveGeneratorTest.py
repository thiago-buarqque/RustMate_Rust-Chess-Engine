import unittest

from Board import Board
from MoveGenerator import MoveGenerator
from Piece import PieceColor, PieceType
from utils import EMPTY_BOARD_FEN
from test.BaseTest import BaseTest


class MoveGeneratorTest(BaseTest):

    def setUp(self) -> None:
        self.board = Board()

    def test_generate_pawn_moves_with_blocking_piece(self):
        self.board.place_piece(36, PieceColor.Black | PieceType.Pawn)
        self.board.place_piece(52, PieceColor.White | PieceType.Pawn)

        move_generator = MoveGenerator(self.board)

        white_moves = move_generator.generate_pawn_moves(52)

        self.assertEqual(1, len(white_moves))
        self.assertLists([44], white_moves)

        self.board.move_piece(52, 44)

        white_moves = move_generator.generate_pawn_moves(44)

        self.assertEqual(0, len(white_moves))

        black_moves = move_generator.generate_pawn_moves(36)

        self.assertEqual(0, len(black_moves))

    def test_diagonal_capture_using_pawn(self):
        self.board.place_piece(16, PieceColor.Black | PieceType.Pawn)
        self.board.place_piece(9, PieceColor.Black | PieceType.Pawn)
        self.board.place_piece(18, PieceColor.White | PieceType.Pawn)

        move_generator = self.board.move_generator

        moves = move_generator.generate_pawn_moves(9)

        self.assertEqual(3, len(moves))
        self.assertIn(17, moves)
        self.assertIn(25, moves)
        self.assertIn(18, moves)

    def test_generate_pawn_first_move(self):
        move_generator = self.board.move_generator

        white_moves = move_generator.generate_pawn_moves(8)
        self.assertEqual(2, len(white_moves))
        self.assertIn(16, white_moves)
        self.assertIn(24, white_moves)

        black_moves = move_generator.generate_pawn_moves(48)
        self.assertEqual(2, len(black_moves))
        self.assertIn(40, black_moves)
        self.assertIn(32, black_moves)

    def test_generate_rook_moves(self):
        self.board.load_position(EMPTY_BOARD_FEN)

        self.board.place_piece(0, PieceColor.Black | PieceType.Rook)

        move_generator = self.board.move_generator

        moves = move_generator.generate_rook_moves(0)

        self.assertLists([1, 2, 3, 4, 5, 6, 7, 8, 16, 24, 32, 40, 48, 56],
                         moves)

        self.board.place_piece(16, PieceColor.Black | PieceType.Pawn)
        self.board.place_piece(4, PieceColor.Black | PieceType.King)

        moves = move_generator.generate_rook_moves(0)

        self.assertLists([1, 2, 3, 8], moves)

        self.board.place_piece(16, PieceColor.White | PieceType.Queen)

        moves = move_generator.generate_rook_moves(0)

        self.assertLists([1, 2, 3, 8, 16], moves)

    def test_generate_bishop_moves(self):
        self.board.load_position(EMPTY_BOARD_FEN)
        self.board.place_piece(2, PieceColor.Black | PieceType.Bishop)

        move_generator = self.board.move_generator

        moves = move_generator.generate_bishop_moves(2)

        self.assertLists([9, 16, 11, 20, 29, 38, 47], moves)

        self.board.place_piece(5, PieceColor.Black | PieceType.Bishop)
        moves = move_generator.generate_bishop_moves(5)

        self.assertLists([12, 19, 26, 33, 40, 14, 23], moves)

        self.board.place_piece(58, PieceColor.White | PieceType.Bishop)
        moves = move_generator.generate_bishop_moves(58)

        self.assertLists([49, 40, 51, 44, 37, 30, 23], moves)

        self.board.place_piece(61, PieceColor.White | PieceType.Bishop)
        moves = move_generator.generate_bishop_moves(61)

        self.assertLists([52, 43, 34, 25, 16, 54, 47], moves)

    def test_generate_queen_moves(self):
        self.board.load_position(EMPTY_BOARD_FEN)

        self.board.place_piece(0, PieceColor.White | PieceType.Queen)

        move_generator = self.board.move_generator

        moves = move_generator.generate_queen_moves(0)

        self.assertLists(
            [9, 18, 27, 36, 45, 54, 63, 1, 2, 3, 4, 5, 6, 7, 8, 16, 24, 32, 40,
             48, 56], moves)

        self.board.move_piece(0, 36)

        moves = move_generator.generate_queen_moves(36)

        self.assertLists(
            [27, 18, 9, 0, 29, 22, 15, 43, 50, 57, 45, 54, 63, 28, 20,
             12, 4, 35, 34, 33, 32, 37, 38, 39, 44, 52, 60], moves)

    def test_generate_king_moves(self):
        self.board.load_position(EMPTY_BOARD_FEN)

        self.board.place_piece(36, PieceColor.Black | PieceType.King)
        self.board.place_piece(5, PieceColor.White | PieceType.Queen)

        move_generator = self.board.move_generator

        opponent_moves = move_generator.generate_queen_moves(5)

        king_moves = move_generator.generate_king_moves(
            opponent_moves,
            36
        )

        self.assertLists([27, 28, 35, 43, 44], king_moves)

    # For each test assert that moves that would reveal king don't exist
    # maybe in board#getLegalMoves?


if __name__ == '__main__':
    unittest.main()
