use crate::{ai::ai_player::AIPlayer, common::contants::INITIAL_FEN, game::board::Board};

pub struct GlobalState {
    pub ai: AIPlayer,
    pub board: Board,
    pub time_to_think: u64,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        let mut board: Board = Board::new();

        board.load_position(INITIAL_FEN);

        GlobalState {
            ai: AIPlayer::new(),
            board,
            time_to_think: 2,
        }
    }
}
