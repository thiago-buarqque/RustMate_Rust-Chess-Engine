use crate::{constants::constants::INITIAL_FEN, game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator}};

pub struct GlobalState {
    // pub ai: AIPlayer,
    pub board: Board,
    pub move_generator: MoveGenerator,
    pub seconds_to_think: u64,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        let board: Board = Board::from_fen(INITIAL_FEN);

        GlobalState {
            board,
            move_generator: MoveGenerator::new(),
            seconds_to_think: 2,
        }
    }
}
