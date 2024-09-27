
use std::{
    collections::HashMap, time::Instant
};

use crate::game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator};

pub fn count_moves(
    board: &mut Board, depth: usize, track_moves: bool, move_generator: &mut MoveGenerator,
) -> u64 {
    
    let mut lookup_table = HashMap::new();

    let start = Instant::now();

    let result = _count_moves(board, depth, track_moves, move_generator, &mut lookup_table);

    println!("Total: {result}");

    println!("\nTime spent: {:#?}", start.elapsed());

    result
}

fn _count_moves(
    board: &mut Board, depth: usize, track_moves: bool, move_generator: &mut MoveGenerator,
    lookup_table: &mut HashMap<(u64, usize), u64>
) -> u64 {
 
    if depth == 0 || board.is_game_finished() {
        lookup_table.insert((board.get_zobrist_hash(), depth), 1);
        
        return 1;
    }

    let moves = move_generator.get_moves(board);

    let mut num_positions = 0;

    let new_depth = depth - 1;

    moves.iter().for_each(|_move| {
        board.move_piece(_move);

        let table_key = (board.get_zobrist_hash(), new_depth);

        let moves_count = if lookup_table.contains_key(&table_key) {
            *lookup_table.get(&table_key).unwrap()
        } else {
            _count_moves(board, new_depth, false, move_generator, lookup_table)
        };

        num_positions += moves_count;

        if track_moves {
            println!("{}: {}", _move.to_algebraic_notation(), moves_count);
        }

        board.unmake_last_move();
    });

    lookup_table.insert((board.get_zobrist_hash(), depth), num_positions);

    num_positions
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
        let mut move_generator = MoveGenerator::new();
        // Positions for initial FEN

        assert_eq!(count_moves(&mut board, 1, false, &mut move_generator), 20);
        assert_eq!(count_moves(&mut board, 2, false, &mut move_generator), 400);
        assert_eq!(count_moves(&mut board, 3, false, &mut move_generator), 8_902);
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
