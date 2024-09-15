use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use crate::game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator};

pub fn count_moves(
    board: &mut Board, depth: usize, track_moves: bool, move_generator: &MoveGenerator,
) -> u64 {
    let start = Instant::now();

    let result = _count_moves(board, depth, track_moves, move_generator);
    println!("Total: {result}");

    println!("\nTime spend: {:#?}", start.elapsed());

    result
}

fn _count_moves(
    board: &mut Board, depth: usize, track_moves: bool, move_generator: &MoveGenerator,
) -> u64 {
    if depth == 0 || board.is_game_finished() {
        return 1;
    }

    let moves = move_generator.get_moves(board);

    // moves.sort_by(|a, b|
    // a.to_algebraic_notation().cmp(&b.to_algebraic_notation()));

    let num_positions = Arc::new(AtomicU64::new(0));

    moves.par_iter().for_each(|_move| {
        let mut board = board.clone();
        let _ = board.move_piece(_move.clone());

        let moves_count = _count_moves(&mut board, depth - 1, false, move_generator);

        num_positions.fetch_add(moves_count, Ordering::SeqCst);

        if track_moves {
            println!("{}: {}", _move.to_algebraic_notation(), moves_count)
        }

        board.unmake_last_move();
    });

    num_positions.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use crate::game_bit_board::{
        _move::moves_counter::count_moves, board::Board,
        move_generator::move_generator::MoveGenerator,
    };

    #[test]
    fn test_move_generation_count() {
        let mut board = Board::new();
        let move_generator = MoveGenerator::new();
        // Positions for initial FEN

        assert_eq!(count_moves(&mut board, 1, false, &move_generator), 20);
        assert_eq!(count_moves(&mut board, 2, false, &move_generator), 400);
        assert_eq!(count_moves(&mut board, 3, false, &move_generator), 8_902);
        // assert_eq!(count_moves(&mut board, 4, false, &move_generator),
        // 197_281); assert_eq!(
        //     count_moves(&mut board, 5, false, &move_generator),
        //     4_865_609
        // );
        // assert_eq!(
        //     count_moves(&mut board, 6, false, &move_generator),
        //     119_060_324
        // );

        // board
        //     .load_position("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/
        // PPPBBPPP/R3K2R w KQkq -");

        // assert_eq!(count_moves(&mut board, 1, false), 48);
        // assert_eq!(count_moves(&mut board, 2, false), 2_039);
        // assert_eq!(count_moves(&mut board, 3, false), 97_862);
        // assert_eq!(count_moves(&mut board, 4, false), 4_085_603);
    }
}
