use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use rayon::{iter::IntoParallelRefIterator, prelude::ParallelIterator};

use crate::{
    common::{piece::Piece, contants::INVALID_BOARD_POSITION, piece_move::PieceMove},
    game::board::Board,
};

use super::{
    ai_utils::{get_board_value, get_sorted_moves},
    transposition_table::{TranspositionTable, TranspositionTableEntry},
};

pub struct AIPlayer {}

impl AIPlayer {
    pub fn new() -> Self {
        AIPlayer {}
    }

    pub fn get_move(&mut self, board: &mut Board, time_to_think: u64) -> (f32, PieceMove) {
        let value = Arc::new(Mutex::new(f32::MIN));

        let mut depth = 1;
        let best_move = Arc::new(Mutex::new(PieceMove::new(
            INVALID_BOARD_POSITION,
            0,
            INVALID_BOARD_POSITION,
        )));
        let state_count = Arc::new(AtomicI32::new(0));
        let start_time = Instant::now();
        let transposition_table: Arc<Mutex<TranspositionTable>> =
            Arc::new(Mutex::new(TranspositionTable::new()));

        let mut max = true;

        let alpha = Arc::new(Mutex::new(f32::MIN));
        while Instant::now().duration_since(start_time) < Duration::new(time_to_think, 0) {
            let current_best_move = best_move.clone();

            let pieces: Vec<Piece> = board.get_pieces();

            let best_move_guard = current_best_move.lock().unwrap();

            let moves: Vec<PieceMove> =
                get_sorted_moves(&Some(best_move_guard.to_owned()), board, true, &pieces);

            drop(best_move_guard);

            // print!("Sorted moves for: {}", board.get_state_reference().get_fen());

            // for _move in moves.clone() {
            //     print!(" {}->{}: {}", _move.get_from_position(), _move.get_to_position(), _move.get_move_worth());
            // }

            // println!();

            moves.par_iter().for_each(|_move| {
                let mut new_board = board.clone();

                let _ = new_board.move_piece(&_move);

                let score = -self.negamax(
                    &mut new_board,
                    -f32::MAX,
                    -*alpha.lock().unwrap(),
                    !max,
                    depth,
                    &state_count,
                    &transposition_table,
                );

                let mut alpha_guard = alpha.lock().unwrap();

                if score > *alpha_guard {
                    *alpha_guard = score;
                    
                    let mut best_move_guard = current_best_move.lock().unwrap();
                    
                    *best_move_guard = _move.clone();
                }

                drop(alpha_guard);
            });

            depth += 1;
            max = !max;

            let _transposition_table = transposition_table.lock().unwrap();

            let current_best_move = current_best_move.lock().unwrap();

            // println!("Iteration best move: {}->{}: {}", 
            //     current_best_move.get_from_position(), current_best_move.get_to_position(), current_best_move.get_move_worth());

            // println!();

            drop(current_best_move);

            println!(
                "Hash table size on depth {}: {}kb",
                depth,
                _transposition_table.estimated_memory_usage_kb()
            );

            // println!();
            // println!();

            drop(_transposition_table)
        }

        let _transposition_table = transposition_table.lock().unwrap();

        let locked_value = value.lock().unwrap();

        let locked_best_move = best_move.lock().unwrap();

        let best_move = locked_best_move.to_owned();

        println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}", state_count.load(Ordering::SeqCst), start_time.elapsed().as_millis(), depth, _transposition_table.get_hits(), best_move.get_move_worth());

        (locked_value.to_owned(), best_move)
    }

    fn negamax(
        &self,
        board: &mut Board,
        alpha: f32,
        beta: f32,
        max: bool,
        depth: u8,
        state_count: &Arc<AtomicI32>,
        transposition_table: &Arc<Mutex<TranspositionTable>>,
    ) -> f32 {
        // Check if the position is already in the table
        let mut _transposition_table = transposition_table.lock().unwrap();

        let mut best_move = None;

        if let Some(entry) = _transposition_table.retrieve(board.get_zobrist_hash()) {
            best_move = entry.best_move.clone();

            if entry.depth >= depth {
                return entry.value;
            }
        }
        drop(_transposition_table);

        let pieces: Vec<Piece> = board.get_pieces();

        if depth == 0 || board.is_game_finished() {
            state_count.fetch_add(1, Ordering::SeqCst);

            let value: f32 = get_board_value(board, max, &pieces);

            let mut _transposition_table = transposition_table.lock().unwrap();

            _transposition_table.store(
                board.get_zobrist_hash(),
                TranspositionTableEntry {
                    depth,
                    value,
                    best_move: None,
                },
            );

            drop(_transposition_table);

            return if board.is_game_finished() && depth > 1 {
                value * depth as f32
            } else {
                value
            };
        }

        let mut moves: Vec<PieceMove> = get_sorted_moves(&best_move, board, max, &pieces);

        let mut alpha = alpha;
        for (i, _move) in moves.iter_mut().enumerate() {
            let _ = board.move_piece(_move);

            let mut new_depth = depth - 1;

            // Considering the move sorting is good: we could decrease
            // the depth from the 6th move forward.
            if i > 4 && !_move.is_capture() && depth >= 2 {
                new_depth = 1;
            }

            let score = -self.negamax(
                board,
                -beta,
                -alpha,
                !max,
                new_depth,
                &state_count,
                transposition_table,
            );

            board.undo_last_move();

            if score > alpha {
                alpha = score;
                best_move = Some(_move.clone());
                if alpha >= beta {
                    break;
                }
            }
        }

        let mut _transposition_table = transposition_table.lock().unwrap();

        _transposition_table.store(
            board.get_zobrist_hash(),
            TranspositionTableEntry {
                depth,
                value: alpha,
                best_move,
            },
        );

        drop(_transposition_table);

        alpha
    }
}
