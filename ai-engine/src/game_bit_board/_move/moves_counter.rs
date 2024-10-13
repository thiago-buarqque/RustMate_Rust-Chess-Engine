use std::{collections::HashMap, time::Instant};

use crate::game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator};

pub fn count_moves(
    board: &mut Board, depth: usize, track_moves: bool, move_generator: &mut MoveGenerator,
) -> u64 {
    let mut lookup_table = HashMap::new();

    let start = Instant::now();

    let result = _count_moves(board, depth, track_moves, move_generator, &mut lookup_table);

    if track_moves {
        println!("\nTotal: {result}");

        println!("\nTime spent: {:#?}", start.elapsed());
    }

    result
}

fn _count_moves(
    board: &mut Board, depth: usize, track_moves: bool, move_generator: &mut MoveGenerator,
    lookup_table: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if depth == 0 || board.is_game_finished() {
        return 1;
    }

    let mut moves = move_generator.get_moves(board);

    let mut num_positions = 0;

    let new_depth = depth - 1;

    moves.iter_mut().for_each(|_move| {
        board.move_piece(_move);

        let moves_count = _count_moves(board, new_depth, false, move_generator, lookup_table);

        num_positions += moves_count;

        if track_moves {
            println!("{}: {}", _move.to_algebraic_notation(), moves_count);
        }

        board.unmake_last_move();
    });

    num_positions
}

#[cfg(test)]
mod moves_counter_test {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::game_bit_board::{
        _move::moves_counter::count_moves, board::Board,
        move_generator::move_generator::MoveGenerator,
    };

    static MOVE_GENERATOR: Lazy<Mutex<MoveGenerator>> =
        Lazy::new(|| Mutex::new(MoveGenerator::new()));

    fn assert_fen_moves(
        fen: String, move_generator: &mut MoveGenerator, resuls: Vec<(usize, u64)>,
    ) {
        println!("\nPerft position {fen}");
        let mut board = Board::from_fen(fen.as_str());

        for (depth, expected_result) in resuls {
            print!("Depth {depth}: ");
            assert_eq!(
                expected_result,
                count_moves(&mut board, depth, false, move_generator)
            );
            println!("OK!");
        }
    }

    #[test]
    fn test_perft_pos_1() {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut results = Vec::new();

        results.push((1, 20));
        results.push((2, 400));
        results.push((3, 8_902));
        results.push((4, 197_281));
        results.push((5, 4_865_609));
        results.push((6, 119_060_324));

        assert_fen_moves(
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            &mut move_generator,
            results,
        );
    }

    #[test]
    fn test_perft_pos_2() {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut results = Vec::new();

        results.push((1, 48));
        results.push((2, 2_039));
        results.push((3, 97_862));
        results.push((4, 4_085_603));
        results.push((5, 193_690_690));

        assert_fen_moves(
            String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"),
            &mut move_generator,
            results,
        );
    }

    #[test]
    fn test_perft_pos_3() {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut results = Vec::new();

        results.push((1, 14));
        results.push((2, 191));
        results.push((3, 2_812));
        results.push((4, 43_238));
        results.push((5, 674_624));
        results.push((6, 11_030_083));
        results.push((7, 178_633_661));

        assert_fen_moves(
            String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -"),
            &mut move_generator,
            results,
        );
    }

    #[test]
    fn test_perft_pos_4() {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut results = Vec::new();

        results.push((1, 6));
        results.push((2, 264));
        results.push((3, 9_467));
        results.push((4, 422_333));
        results.push((5, 15_833_292));

        assert_fen_moves(
            String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"),
            &mut move_generator,
            results,
        );
    }

    #[test]
    fn test_perft_pos_5() {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut results = Vec::new();

        results.push((1, 44));
        results.push((2, 1486));
        results.push((3, 62_379));
        results.push((4, 2_103_487));
        results.push((5, 89_941_194));

        assert_fen_moves(
            String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
            &mut move_generator,
            results,
        );
    }

    #[test]
    fn test_perft_pos_6() {
        let mut move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut results = Vec::new();

        results.push((1, 46));
        results.push((2, 2_079));
        results.push((3, 89_890));
        results.push((4, 3_894_594));
        results.push((5, 164_075_551));

        assert_fen_moves(
            String::from(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            ),
            &mut move_generator,
            results,
        );
    }
}
