use pyo3::{exceptions, prelude::*};
use std::time::Instant;

use crate::{
    ai::ai_player::AIPlayer,
    common::{
        contants::{EMPTY_PIECE, INITIAL_FEN},
        piece_move::PieceMove,
        piece_utils::{get_promotion_options, piece_fen_from_value},
    },
    dto::{
        dto_utils::piece_move_dto_from_piece_move, piece_dto::PieceDTO,
        piece_move_dto::PieceMoveDTO,
    },
    game::board::Board,
};

#[pyclass]
pub struct BoardWrapper {
    board: Board,
    nega_max: AIPlayer,
}

// is this a facade?
#[pymethods]
impl BoardWrapper {
    #[new]
    fn default() -> BoardWrapper {
        let mut board: Board = Board::new();

        board.load_position(INITIAL_FEN);

        BoardWrapper {
            board,
            nega_max: AIPlayer::new(),
        }
    }

    pub fn get_ai_move(&mut self, depth: u8) -> (f32, PieceMoveDTO) {
        let start = Instant::now();

        let result = self.nega_max.get_move(&mut self.board, depth);

        let duration = start.elapsed();

        println!("Time elapsed is: {:?}", duration);

        (result.0, piece_move_dto_from_piece_move(&result.1))
    }

    pub fn get_move_generation_count(&mut self, depth: usize) -> u64 {
        let nodes_searched = move_generation_count(&mut self.board, depth, true);
        println!("\nNodes searched: {}", nodes_searched);

        nodes_searched
    }

    pub fn black_captures_to_fen(&self) -> Vec<char> {
        self.board.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<char> {
        self.board.white_captures_to_fen()
    }

    pub fn get_available_moves(&mut self) -> Vec<PieceDTO> {
        let mut pieces: Vec<PieceDTO> = Vec::new();

        for piece in self.board.get_pieces().iter() {
            pieces.push(PieceDTO::new(
                piece_fen_from_value(piece.get_value()),
                piece.get_moves_clone(),
                piece.get_position(),
                piece.is_white(),
            ));
        }

        pieces
    }

    pub fn get_zobrist_hash(&self) -> u64 {
        self.board.get_zobrist_hash()
    }

    pub fn get_black_en_passant(&self) -> i8 {
        self.board.get_state_reference().get_black_en_passant()
    }

    pub fn get_white_en_passant(&self) -> i8 {
        self.board.get_state_reference().get_white_en_passant()
    }

    pub fn get_winner_fen(&self) -> char {
        self.board.get_winner_fen()
    }

    pub fn is_white_move(&self) -> bool {
        self.board.is_white_move()
    }

    pub fn load_position(&mut self, fen: &str) {
        self.board.load_position(fen)
    }

    pub fn move_piece(&mut self, piece_move: PieceMoveDTO) -> PyResult<()> {
        let _move = PieceMove::from_dto(piece_move);

        match self.board.make_move(&_move) {
            Ok(()) => Ok(()),
            Err(error) => Err(exceptions::PyValueError::new_err(error)),
        }
    }
}

fn move_generation_count(board: &mut Board, depth: usize, track_moves: bool) -> u64 {
    if depth == 0 || board.is_game_finished() {
        return 1;
    }

    let pieces = board.get_pieces();

    let mut num_positions: u64 = 0;

    for piece in pieces.iter() {
        if (piece.get_value() == EMPTY_PIECE) || (piece.is_white() != board.is_white_move()) {
            continue;
        }

        for piece_move in piece.get_moves_clone().iter() {
            let mut promotion_char_options = vec![piece_move.get_promotion_value()];

            if piece_move.is_promotion() {
                promotion_char_options = get_promotion_options(piece.is_white());
            }

            let mut piece_move = piece_move.clone();

            for promotion_option in promotion_char_options {
                piece_move.set_promotion_value(promotion_option);

                let _ = board.make_move(&piece_move);

                let moves_count = move_generation_count(board, depth - 1, false);
                num_positions += moves_count;

                if track_moves {
                    if piece_move.is_promotion() {
                        println!(
                            "{}{}: {}",
                            get_move_char(&piece_move),
                            promotion_option.clone(),
                            moves_count
                        )
                    } else {
                        println!("{}: {}", get_move_char(&piece_move), moves_count)
                    }
                }

                board.undo_last_move();
            }
        }
    }

    num_positions
}

#[inline]
fn get_position_line_number(position: i8) -> usize {
    (8 - ((position - (position % 8)) / 8)) as usize
}

#[inline]
fn get_position_column_number(position: i8) -> usize {
    (position - (position - (position % 8))) as usize
}

fn get_position_string(position: i8, columns: &[char]) -> String {
    let line = get_position_line_number(position);
    let column = get_position_column_number(position);

    format!("{}{}", columns[column], line)
}

fn get_move_char(piece_move: &PieceMove) -> String {
    let columns = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let from_position_str = get_position_string(piece_move.get_from_position(), &columns);
    let to_position_str = get_position_string(piece_move.get_to_position(), &columns);

    format!("{}{}", from_position_str, to_position_str)
}

#[cfg(test)]
mod tests {
    use super::BoardWrapper;

    #[test]
    fn test_move_generation_count() {
        let mut board_wrapper = BoardWrapper::default();

        // Positions for initial FEN

        assert_eq!(board_wrapper.get_move_generation_count(1), 20);
        assert_eq!(board_wrapper.get_move_generation_count(2), 400);
        assert_eq!(board_wrapper.get_move_generation_count(3), 8_902);
        assert_eq!(board_wrapper.get_move_generation_count(4), 197_281);

        board_wrapper
            .load_position("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");

        assert_eq!(board_wrapper.get_move_generation_count(1), 48);
        assert_eq!(board_wrapper.get_move_generation_count(2), 2_039);
        assert_eq!(board_wrapper.get_move_generation_count(3), 97_862);
        assert_eq!(board_wrapper.get_move_generation_count(4), 4_085_603);
    }
}
