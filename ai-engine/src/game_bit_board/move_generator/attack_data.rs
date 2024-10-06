use crate::game_bit_board::{
    board::Board,
    enums::{Color, PieceType},
    positions::{same_anti_diagonal, same_diagonal, same_file, same_rank},
    utils::bitwise_utils::{get_direction_fn_to_square, pop_lsb, to_bitboard_position},
};

use super::{
    contants::{BLACK_PAWN_ATTACKS, KNIGHT_MOVES, WHITE_PAWN_ATTACKS},
    move_generator::MoveGenerator,
    utils::get_king_relevant_squares_related_to_enemy_pawns,
};

pub struct AttackData {
    pub attack_bb: u64,
    pub defenders_bb: u64,
    pub friendly_pins_moves_bbs: [u64; 64],
    pub in_check: bool,
    pub in_double_check: bool,
    pub king_allowed_squares: u64,
    pub king_bb_position: u64,
    pub king_square: usize,
    pub side_to_move: Color,
}

impl AttackData {
    pub fn new() -> Self {
        Self {
            attack_bb: 0,
            defenders_bb: 0,
            friendly_pins_moves_bbs: [u64::MAX; 64],
            in_check: false,
            in_double_check: false,
            king_allowed_squares: 0,
            king_bb_position: 0,
            king_square: 0,
            side_to_move: Color::White,
        }
    }

    fn init(&mut self, board: &Board) {
        self.attack_bb = 0;
        self.defenders_bb = 0;
        self.friendly_pins_moves_bbs = [u64::MAX; 64];
        self.in_check = false;
        self.in_double_check = false;
        self.king_allowed_squares = u64::MAX;

        self.side_to_move = board.get_side_to_move();
        self.king_bb_position = board.get_piece_positions(self.side_to_move, PieceType::King);
        self.king_square = pop_lsb(&mut self.king_bb_position.clone()) as usize;
    }

    pub fn calculate_attack_data(&mut self, board: &Board, move_generator: &MoveGenerator) {
        self.init(board);

        self.check_sliding_attacks(board, PieceType::Rook, move_generator);

        self.check_sliding_attacks(board, PieceType::Bishop, move_generator);

        self.check_sliding_attacks(board, PieceType::Queen, move_generator);

        self.check_knight_attacks(board);

        self.check_pawn_attacks(board);

        if self.in_double_check {
            self.defenders_bb = 0;
            return;
        }

        if self.defenders_bb == 0 && self.attack_bb == 0 {
            self.defenders_bb = u64::MAX;
        }

        if self.attack_bb == 0 {
            self.attack_bb = u64::MAX;
        }

        self.king_allowed_squares &= !board.get_player_pieces_positions(self.side_to_move);

        // if self.in_check {
        //     println!("In Check: {}", self.in_check);
        // }

        // if self.in_double_check {
        //     println!("In Double Check: {}", self.in_double_check);
        // }

        // if self.defenders_bb != u64::MAX {
        //     println!("\nPush bb");

        //     print_board(
        //         Color::White,
        //         self.king_square as u64,
        //         PieceType::King,
        //         self.defenders_bb,
        //     );
        // }

        // if self.attack_bb != u64::MAX {
        //     println!("\nAttack bb");

        //     print_board(
        //         Color::White,
        //         self.king_square as u64,
        //         PieceType::King,
        //         self.attack_bb,
        //     );
        // }

        // println!("\nKing allowed squares");

        // print_board(
        //     Color::White,
        //     self.king_square as u64,
        //     PieceType::King,
        //     self.king_allowed_squares,
        // );

        // self.friendly_pins_moves_bbs.iter().enumerate().for_each(|(i, bb)| {
        //     if *bb != u64::MAX {
        //         println!("\nFriendly pin at {}", Squares::to_string(i));

        //         print_board(Color::White, i as u64, board.get_piece_type(i),
        // *bb);     }
        // });
    }

    fn check_knight_attacks(&mut self, board: &Board) {
        let possible_checkers = KNIGHT_MOVES[self.king_square];

        let mut opponent_knights =
            board.get_piece_positions(self.side_to_move.opponent(), PieceType::Knight);

        let mut attackers = opponent_knights & possible_checkers;

        if attackers != 0 {
            self.in_double_check = self.in_check;
            self.in_check = true;

            while attackers != 0 {
                let attacker_square = pop_lsb(&mut attackers);

                self.attack_bb |= to_bitboard_position(attacker_square as u64);
            }
        }

        while opponent_knights != 0 {
            let square = pop_lsb(&mut opponent_knights) as usize;

            let moves = KNIGHT_MOVES[square];

            self.king_allowed_squares &= !moves;
        }
    }

    fn check_pawn_attacks(&mut self, board: &Board) {
        let pawn_attacks = if self.side_to_move.is_white() {
            BLACK_PAWN_ATTACKS
        } else {
            WHITE_PAWN_ATTACKS
        };

        let relevant_squares =
            get_king_relevant_squares_related_to_enemy_pawns(self.side_to_move, self.king_square);

        let opponent_pawns =
            board.get_piece_positions(self.side_to_move.opponent(), PieceType::Pawn);

        let mut possible_attackers = opponent_pawns & relevant_squares;

        if possible_attackers == 0 {
            return;
        }

        while possible_attackers != 0 {
            let attacker_square = pop_lsb(&mut possible_attackers) as usize;

            let pawn_attacks = pawn_attacks[attacker_square];

            if pawn_attacks & self.king_bb_position != 0 {
                self.in_double_check = self.in_check;
                self.in_check = true;

                self.attack_bb |= to_bitboard_position(attacker_square as u64);
            }

            self.king_allowed_squares &= !pawn_attacks;
        }
    }

    fn check_sliding_attacks(
        &mut self, board: &Board, piece_type: PieceType, move_generator: &MoveGenerator,
    ) {
        let opponent = self.side_to_move.opponent();

        let mut opponent_sliding_pieces = board.get_piece_positions(opponent, piece_type);
        let opponent_pieces = board.get_player_pieces_positions(opponent);

        while opponent_sliding_pieces != 0 {
            let square = pop_lsb(&mut opponent_sliding_pieces) as usize;

            let mut attacks = 0;

            let same_orthogonal_ray =
                same_rank(square, self.king_square) || same_file(square, self.king_square);

            let same_diagonal_ray = same_diagonal(square, self.king_square)
                || same_anti_diagonal(square, self.king_square);

            if piece_type == PieceType::Queen {
                attacks |= move_generator.get_orthogonal_attacks(
                    board,
                    opponent,
                    square,
                    &opponent_pieces,
                );
                attacks |=
                    move_generator.get_diagonal_attacks(board, opponent, square, &opponent_pieces);

                self.king_allowed_squares &= !attacks;

                if !same_orthogonal_ray && !same_diagonal_ray {
                    continue;
                }
            } else if piece_type == PieceType::Rook {
                attacks |= move_generator.get_orthogonal_attacks(
                    board,
                    opponent,
                    square,
                    &opponent_pieces,
                );

                self.king_allowed_squares &= !attacks;

                if !same_orthogonal_ray {
                    continue;
                }
            } else if piece_type == PieceType::Bishop {
                attacks |=
                    move_generator.get_diagonal_attacks(board, opponent, square, &opponent_pieces);

                self.king_allowed_squares &= !attacks;

                if !same_diagonal_ray {
                    continue;
                }
            }

            if attacks & self.king_bb_position != 0 {
                self.handle_sliding_piece_check(square);
            } else {
                self.check_for_pins(board, square);
            }
        }
    }

    fn handle_sliding_piece_check(&mut self, square: usize) {
        self.attack_bb |= 1 << square;
        self.in_double_check = self.in_check;
        self.in_check = true;

        let direction_fn = get_direction_fn_to_square(square, self.king_square);

        let mut path_to_king = direction_fn(1 << square);
        let mut current_pos = path_to_king;

        while current_pos & self.king_bb_position == 0 {
            path_to_king |= current_pos;
            current_pos = direction_fn(current_pos);
        }

        current_pos = direction_fn(current_pos);

        while current_pos != 0 {
            self.king_allowed_squares &= !current_pos;

            current_pos = direction_fn(current_pos);
        }

        self.defenders_bb |= path_to_king;
    }

    fn check_for_pins(&mut self, board: &Board, square: usize) {
        let mut friendly_pin_bb_pos = 0;
        let mut opponent_pin_bb_pos = 0;

        let attacker_bb_pos = 1 << square;

        let mut path_to_king = attacker_bb_pos;

        let direction_fn = get_direction_fn_to_square(square, self.king_square);

        let mut current_pos = direction_fn(path_to_king);

        while current_pos != 0 {
            if self.king_bb_position == current_pos {
                break;
            }

            let piece_type = board.get_piece_type_by_bb_pos(current_pos);

            if piece_type != PieceType::Empty {
                let piece_color = board.get_piece_color_by_bb_pos(current_pos);

                if friendly_pin_bb_pos != 0 || opponent_pin_bb_pos != 0 {
                    return;
                }

                if piece_color == self.side_to_move {
                    friendly_pin_bb_pos = current_pos;
                } else {
                    opponent_pin_bb_pos = current_pos;
                }
            }

            path_to_king |= current_pos;

            current_pos = direction_fn(current_pos);
        }

        if friendly_pin_bb_pos != 0 {
            let square = pop_lsb(&mut friendly_pin_bb_pos) as usize;

            self.friendly_pins_moves_bbs[square] = path_to_king;
        }
    }
}

#[cfg(test)]
mod attack_data_tests {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator};

    use super::AttackData;

    static MOVE_GENERATOR: Lazy<Mutex<MoveGenerator>> =
        Lazy::new(|| Mutex::new(MoveGenerator::new()));

    #[test]
    fn test_pawn_check() {
        let move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut attack_data = AttackData::new();

        let mut board = Board::from_fen("8/8/2p3p1/3pp3/4Kpp1/8/8/8 w - - 0 1");

        // 8 . . . . . . . .
        // 7 . . . . . . . .
        // 6 . . ♟ . . . ♟ .
        // 5 . . . ♟ ♟ . . .
        // 4 . . . . ♔ ♟ ♟ .
        // 3 . . . . . . . .
        // 2 . . . . . . . .
        // 1 . . . . . . . .
        //   a b c d e f g h
        board.display();

        attack_data.calculate_attack_data(&mut board, &move_generator);

        assert_eq!(0xFFFFFF55C30FFFFF, attack_data.king_allowed_squares);
        assert_eq!(0x0000000800000000, attack_data.attack_bb);

        // Since there is no way to block a pawn check, push bb is empty
        assert_eq!(0x0000000000000000, attack_data.defenders_bb);
    }

    #[test]
    fn test_in_double_check() {
        let move_generator = MOVE_GENERATOR.lock().unwrap();

        let mut attack_data = AttackData::new();

        let mut board =
            Board::from_fen("rnbqkbnr/pp3ppp/2pN4/3p4/8/5p2/PPPPQPPP/R1B1KB1R b KQkq - 1 1");

        // 8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        // 7 ♟ ♟ . . . ♟ ♟ ♟
        // 6 . . ♟ ♘ . . . .
        // 5 . . . ♟ . . . .
        // 4 . . . . . . . .
        // 3 . . . . . ♟ . .
        // 2 ♙ ♙ ♙ ♙ ♕ ♙ ♙ ♙
        // 1 ♖ . ♗ . ♔ ♗ . ♖
        //   a b c d e f g h
        board.display();

        attack_data.calculate_attack_data(&mut board, &move_generator);

        // Assert double check from queen and knight
        assert_eq!(true, attack_data.in_check);
        assert_eq!(true, attack_data.in_double_check);
        assert_eq!(0x0000080000001000, attack_data.attack_bb);
        assert_eq!(0, attack_data.defenders_bb);

        board = Board::from_fen("rnbqkbnr/pp3ppp/3N4/2p5/B7/3Ppp2/PPP1QPPP/R1B1K2R b KQkq - 0 1");

        // 8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        // 7 ♟ ♟ . . . ♟ ♟ ♟
        // 6 . . . ♘ . . . .
        // 5 . . ♟ . . . . .
        // 4 ♗ . . . . . . .
        // 3 . . . ♙ ♟ ♟ . .
        // 2 ♙ ♙ ♙ . ♕ ♙ ♙ ♙
        // 1 ♖ . ♗ . ♔ . . ♖
        //   a b c d e f g h
        board.display();

        attack_data.calculate_attack_data(&mut board, &move_generator);

        // Assert double check from bishop and knight
        assert_eq!(true, attack_data.in_check);
        assert_eq!(true, attack_data.in_double_check);
        assert_eq!(0x0000080001000000, attack_data.attack_bb);
        assert_eq!(0, attack_data.defenders_bb);

        board = Board::from_fen("rnbqkbnr/pp3ppp/2B5/2p2N2/3p4/3PRp2/PPPQ1PPP/R1B1K3 b Qkq - 0 1");

        // 8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        // 7 ♟ ♟ . . . ♟ ♟ ♟
        // 6 . . ♗ . . . . .
        // 5 . . ♟ . . ♘ . .
        // 4 . . . ♟ . . . .
        // 3 . . . ♙ ♖ ♟ . .
        // 2 ♙ ♙ ♙ ♕ . ♙ ♙ ♙
        // 1 ♖ . ♗ . ♔ . . .
        //   a b c d e f g h
        board.display();

        attack_data.calculate_attack_data(&mut board, &move_generator);

        // Assert double check from bishop and rook
        assert_eq!(true, attack_data.in_check);
        assert_eq!(true, attack_data.in_double_check);
        assert_eq!(0x0000040000100000, attack_data.attack_bb);
        assert_eq!(0, attack_data.defenders_bb);
    }
}
