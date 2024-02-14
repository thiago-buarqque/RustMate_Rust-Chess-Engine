use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use rayon::{iter::IntoParallelRefIterator, prelude::ParallelIterator};

use crate::{
    common::{contants::INVALID_BOARD_POSITION, enums::PieceColor, piece::Piece, piece_move::PieceMove},
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

    pub fn get_move(&mut self, board: &mut Board, time_to_think: u64) -> (u128, u8, f32, PieceMove) {
        let mut depth = 1;
        let best_move = Arc::new(Mutex::new(PieceMove::new(
            INVALID_BOARD_POSITION,
            0,
            INVALID_BOARD_POSITION,
        )));

        let transposition_table: Arc<Mutex<TranspositionTable>> = Arc::new(Mutex::new(TranspositionTable::new()));
        
        let mut max = true;
        
        let alpha = Arc::new(Mutex::new(f32::MIN));

        let start_time = Instant::now();

        while Instant::now().duration_since(start_time) < Duration::new(time_to_think, 0) 
            && depth < u8::MAX {

            let current_best_move = best_move.clone();

            let pieces: Vec<Piece> = board.get_pieces();

            let best_move_guard = current_best_move.lock().unwrap();

            let moves: Vec<PieceMove> =
                get_sorted_moves(&Some(best_move_guard.to_owned()), board, true, &pieces);

            drop(best_move_guard);

            moves.par_iter().for_each(|_move| {
                let mut new_board = board.clone();

                let _ = new_board.move_piece(&_move);

                let score = -self.negamax(
                    &mut new_board,
                    -f32::MAX,
                    -*alpha.lock().unwrap(),
                    !max,
                    depth,
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

            drop(current_best_move);

            println!(
                "Transposition table size on depth {}: ~{}kb",
                depth,
                _transposition_table.estimated_memory_usage_kb()
            );

            drop(_transposition_table)
        }

        let _transposition_table = transposition_table.lock().unwrap();

        let locked_alpha = alpha.lock().unwrap();

        let locked_best_move = best_move.lock().unwrap();

        let best_move = locked_best_move.to_owned();

        println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}", 
            _transposition_table.len(), start_time.elapsed().as_millis(), depth, _transposition_table.get_hits(), best_move.get_move_worth());

        (start_time.elapsed().as_millis(), depth, locked_alpha.to_owned(), best_move)
    }

    fn negamax(
        &self,
        board: &mut Board,
        alpha: f32,
        beta: f32,
        max: bool,
        depth: u8,
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
            let value: f32 = get_board_value(board, max, &pieces);

            let mut _transposition_table = transposition_table.lock().unwrap();

            _transposition_table.store(
                board.get_zobrist_hash(),
                TranspositionTableEntry {
                    depth,
                    value,
                    best_move,
                },
            );

            // Favors checkmates the require less moves
            return if board.is_game_finished() && depth > 1 {
                value * depth as f32
            } else {
                value
            };
        }

        let moves: Vec<PieceMove> = get_sorted_moves(&best_move, board, max, &pieces);

        let mut alpha = alpha;
        for (i, _move) in moves.iter().enumerate() {
            let _ = board.move_piece(_move);

            let mut new_depth = depth - 1;

            // Considering the move sorting is good: we could decrease
            // the depth from the 5th move forward.
            if i >= 4 && !_move.is_capture() && depth >= 2 {
                new_depth = 1;
            }

            let score = -self.negamax(
                board,
                -beta,
                -alpha,
                !max,
                new_depth,
                transposition_table,
            );

            let game_finished = board.is_game_finished();

            let draw = board.get_winner() == (PieceColor::Black.value() | PieceColor::White.value());

            board.undo_last_move();

            if score > alpha {
                alpha = score;

                best_move = Some(_move.clone());

                if alpha >= beta {
                    break;
                }
            }

            if game_finished && !draw {
                break;
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

        alpha
    }
}
