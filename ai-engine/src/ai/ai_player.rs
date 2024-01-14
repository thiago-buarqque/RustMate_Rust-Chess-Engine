use std::{
    ops::Add,
    sync::{Arc, Mutex},
    time::{Duration, Instant}
};

use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    common::{board_piece::BoardPiece, piece_move::PieceMove},
    game::board::Board,
};

use super::ai_utils::{get_board_value, get_sorted_moves};

pub struct AIPlayer {}

impl AIPlayer {
    pub fn new() -> Self {
        AIPlayer {}
    }

    pub fn get_move(&mut self, board: &mut Board, depth: u8) -> (f32, PieceMove) {
        let now = Instant::now();

        let best_move = Arc::new(Mutex::new(PieceMove::new(-1, 0, -1)));
        let moves_count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        let value = Arc::new(Mutex::new(f32::MIN));

        let pieces: Vec<BoardPiece> = board.get_pieces();

        let mut moves: Vec<PieceMove> = get_sorted_moves(board, true, &pieces);

        moves.par_iter_mut().for_each(|_move| {
            let mut board_copy = board.clone();

            self.search_parallel(
                _move,
                &mut board_copy,
                depth - 1,
                &value,
                &moves_count,
                &best_move,
            );
        });

        println!("Evaluated {} state in {}ms", moves_count.lock().unwrap(), now.elapsed().as_millis());

        let locked_value = value.lock().unwrap();

        let locked_best_move = best_move.lock().unwrap();

        (locked_value.to_owned(), locked_best_move.to_owned())
    }

    fn search_parallel(
        &self,
        _move: &PieceMove,
        board: &mut Board,
        depth: u8,
        value: &Arc<Mutex<f32>>,
        moves_count: &Arc<Mutex<u64>>,
        best_move: &Arc<Mutex<PieceMove>>,
    ) {
        let _ = board.make_move(_move);

        let node_results = self.search(board, f32::MIN, f32::MAX, false, depth - 1);

        let mut locked_moves_count = moves_count.lock().unwrap();

        *locked_moves_count = locked_moves_count.add(node_results.1);

        drop(locked_moves_count);

        let mut locked_value = value.lock().unwrap();

        if -node_results.0 > *locked_value {
            *locked_value = -node_results.0;

            let mut locked_best_move = best_move.lock().unwrap();

            *locked_best_move = _move.clone();

            drop(locked_best_move)
        }

        drop(locked_value);

        board.undo_last_move();
    }

    fn search(
        &self,
        board: &mut Board,
        mut alpha: f32,
        beta: f32,
        max: bool,
        depth: u8,
    ) -> (f32, u64) {
        let pieces: Vec<BoardPiece> = board.get_pieces();

        if depth == 0 || board.is_game_finished() {
            return (get_board_value(board, max, &pieces), 1);
        }

        let mut moves_count = 0;

        let mut moves: Vec<PieceMove> = get_sorted_moves(board, max, &pieces);

        let mut value = f32::MIN;
        'piece_move_loop: for _move in moves.iter_mut() {
            let _ = board.make_move(_move);

            let node_results = self.search(board, -beta, -alpha, !max, depth - 1);

            board.undo_last_move();

            moves_count += node_results.1;

            value = value.max(-node_results.0);

            alpha = alpha.max(value);

            if alpha >= beta {
                break 'piece_move_loop;
            }
        }

        (alpha, moves_count)
    }
}
