use std::{
    ops::Add,
    sync::{Arc, Mutex, atomic::{AtomicI32, Ordering}},
    time::{Duration, Instant}
};

use rayon::{prelude::{IntoParallelRefMutIterator, ParallelIterator}, iter::IntoParallelRefIterator};

use crate::{
    common::{board_piece::BoardPiece, piece_move::PieceMove},
    game::board::Board,
};

use super::{ai_utils::{get_board_value, get_sorted_moves}, transposition_table::{TranspositionTable, TranspositionTableEntry}};

pub struct AIPlayer {}

impl AIPlayer {
    pub fn new() -> Self {
        AIPlayer {}
    }

    pub fn get_move(&mut self, board: &mut Board, time_to_think: u64) -> (f32, PieceMove) {
        let value = Arc::new(Mutex::new(f32::MIN));
        
        let mut depth = 1;
        let best_move = Arc::new(Mutex::new(PieceMove::new(-1, 0, -1)));
        let state_count = Arc::new(AtomicI32::new(0));
        let start_time = Instant::now();
        let transposition_table: Arc<Mutex<TranspositionTable>> = Arc::new(Mutex::new(TranspositionTable::new()));

        let mut max = true;

        while Instant::now().duration_since(start_time) < Duration::new(time_to_think, 0) {
            let current_best_move = best_move.clone();
            let alpha = Arc::new(Mutex::new(f32::MIN));

            let pieces: Vec<BoardPiece> = board.get_pieces();

            let best_move_guard = current_best_move.lock().unwrap();

            let moves: Vec<PieceMove> = get_sorted_moves(&Some(best_move_guard.to_owned()), board, true, &pieces);

            // print!("Sorted moves for: {}", board.get_state_reference().get);

            drop(best_move_guard);

            moves.par_iter().for_each(|_move| {
                let mut new_board = board.clone();
                
                let _ = new_board.make_move(&_move);
                
                let score = -self.negamax(&mut new_board, -f32::MAX, -*alpha.lock().unwrap(), max, depth, &state_count, &transposition_table);
                // println!("Original move worth: {} ({}->{}) | score: {score}", _move.get_move_worth(), _move.get_from_position(), _move.get_to_position());
                
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

            let _transposition_table =  transposition_table.lock().unwrap();

            println!("Hash table size on depth {}: {}kb", depth,_transposition_table.estimated_memory_usage_kb());

            drop(_transposition_table)
        }

        let _transposition_table =  transposition_table.lock().unwrap();

        
        let locked_value = value.lock().unwrap();
        
        let locked_best_move = best_move.lock().unwrap();

        let best_move = locked_best_move.to_owned();

        println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}", state_count.load(Ordering::SeqCst), start_time.elapsed().as_millis(), depth, _transposition_table.get_hits(), best_move.get_move_worth());

        (locked_value.to_owned(), best_move)
    }

    // fn search_parallel(
    //     &self,
    //     _move: &PieceMove,
    //     board: &mut Board,
    //     depth: u8,
    //     value: &Arc<Mutex<f32>>,
    //     moves_count: &Arc<Mutex<u64>>,
    //     best_move: &Arc<Mutex<PieceMove>>,
    // ) {
    //     let _ = board.make_move(_move);
    //     let node_results = self.search(board, f32::MIN, f32::MAX, false, depth - 1);
    //     let mut locked_moves_count = moves_count.lock().unwrap();
    //     *locked_moves_count = locked_moves_count.add(node_results.1);
    //     drop(locked_moves_count);
    //     let mut locked_value = value.lock().unwrap();
    //     if -node_results.0 > *locked_value {
    //         *locked_value = -node_results.0;
    //         let mut locked_best_move = best_move.lock().unwrap();
    //         *locked_best_move = _move.clone();
    //         drop(locked_best_move)
    //     }
    //     drop(locked_value);
    //     board.undo_last_move();
    // }

    fn negamax(
        &self,
        board: &mut Board,
        alpha: f32,
        beta: f32,
        max: bool,
        depth: u8,
        state_count: &Arc<AtomicI32>,
        transposition_table: &Arc<Mutex<TranspositionTable>>
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

        let pieces: Vec<BoardPiece> = board.get_pieces();

        if depth == 0 || board.is_game_finished() {
            state_count.fetch_add(1, Ordering::SeqCst);

            let value: f32 = get_board_value(board, max, &pieces);

            let mut _transposition_table = transposition_table.lock().unwrap();

            _transposition_table.store(board.get_zobrist_hash(), TranspositionTableEntry {
                depth,
                value,
                best_move: None,
            });

            drop(_transposition_table);

            return if board.is_game_finished() && depth > 1 {value * depth as f32} else {value};
        }

        let mut moves: Vec<PieceMove> = get_sorted_moves(&best_move, board, max, &pieces);

        let mut alpha = alpha;
        for _move in moves.iter_mut() {
            let _ = board.make_move(_move);

            let score = -self.negamax(board, -beta, -alpha, !max, depth - 1, &state_count, transposition_table);

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

        _transposition_table.store(board.get_zobrist_hash(), TranspositionTableEntry {
            depth,
            value: alpha,
            best_move,
        });

        drop(_transposition_table);

        alpha
    }
}
