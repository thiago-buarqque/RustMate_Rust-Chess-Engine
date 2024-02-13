use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::common::{board_utils::get_move_notation, contants::EMPTY_PIECE, piece_utils::get_promotion_options};

use super::board::Board;

pub fn count_moves(board: &mut Board, depth: usize, track_moves: bool) -> u64 {
    if depth == 0 || board.is_game_finished() {
        return 1;
    }

    let pieces = board.get_pieces();

    let num_positions = Arc::new(AtomicU64::new(0));

    for piece in pieces.iter() {
        if (piece.get_value() == EMPTY_PIECE) || (piece.is_white() != board.is_white_move()) {
            continue;
        }

        piece.get_moves_clone().par_iter().for_each(|piece_move|{
            let mut board = board.clone();
            let mut promotion_char_options = vec![piece_move.get_promotion_value()];

            if piece_move.is_promotion() {
                promotion_char_options = get_promotion_options(piece.is_white());
            }

            let mut piece_move = piece_move.clone();

            for promotion_option in promotion_char_options {
                piece_move.set_promotion_value(promotion_option);

                let _ = board.move_piece(&piece_move);

                let moves_count = count_moves(&mut board, depth - 1, false);
                
                num_positions.fetch_add(moves_count, Ordering::SeqCst);

                if track_moves {
                    if piece_move.is_promotion() {
                        println!(
                            "{}{}: {}",
                            get_move_notation(&piece_move),
                            promotion_option.clone(),
                            moves_count
                        )
                    } else {
                        println!("{}: {}", get_move_notation(&piece_move), moves_count)
                    }
                }

                board.undo_last_move();
            }
        });
    }

    num_positions.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use crate::game::{board::Board, moves_counter::count_moves};

    #[test]
    fn test_move_generation_count() {
        let mut board = Board::new();
        // Positions for initial FEN

        assert_eq!(count_moves(&mut board, 1, false), 20);
        assert_eq!(count_moves(&mut board, 2, false), 400);
        assert_eq!(count_moves(&mut board, 3, false), 8_902);
        assert_eq!(count_moves(&mut board, 4, false), 197_281);

        board
            .load_position("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");

        assert_eq!(count_moves(&mut board, 1, false), 48);
        assert_eq!(count_moves(&mut board, 2, false), 2_039);
        assert_eq!(count_moves(&mut board, 3, false), 97_862);
        assert_eq!(count_moves(&mut board, 4, false), 4_085_603);
    }
}
