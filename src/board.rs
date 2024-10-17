use crate::constants::{self, NUM_TYPES_PIECES};

#[derive(Default, Debug)]
pub struct Board {
    pieces_bb: [u64; constants::NUM_TYPES_PIECES],
    all_pieces_bb: u64,
    single_color_pieces_bb: [u64; 2],
    slide_attacking: [u64; 2],
    no_slide_attacking: [u64; 2],
    castle_rights: u8,
    white_turn: bool,
    current_eval: f64,
    game_over: bool,
}

impl Board {
    pub fn init(fen: String) -> Board {
        let mut board: Board = Board {
            castle_rights: 7,
            white_turn: true,
            ..Board::default()
        };

        let mut row = 0;
        let mut col = 0;

        for c in fen.chars() {
            if c == '/' {
                row += 1;
                col = 0;
            } else if c.is_numeric() {
                col += c.to_digit(10).expect("Int");
            } else if constants::ENUM_FROM_PIECE.contains_key(&c) {
                board.pieces_bb[constants::ENUM_FROM_PIECE[&c]] |= 1 << (row * 8 + col);
                col += 1;
            } else {
                panic!("Unrecognized input \"{}\" from FEN\n", c);
            }
        }

        for i in 0..board.pieces_bb.len() / 2 {
            board.single_color_pieces_bb[0] |= board.pieces_bb[i];
            board.single_color_pieces_bb[1] |= board.pieces_bb[i + board.pieces_bb.len() / 2];
        }

        board.all_pieces_bb = board.single_color_pieces_bb[0] | board.single_color_pieces_bb[1];

        board
    }

    pub fn print_board(&self) {
        let mut output: [[char; constants::BOARD_SIZE]; constants::BOARD_SIZE] =
            [['.'; constants::BOARD_SIZE]; constants::BOARD_SIZE];

        for i in 0..NUM_TYPES_PIECES {
            let mut pieces = self.pieces_bb[i];

            while pieces != 0 {
                let start_square: usize = pieces.trailing_zeros() as usize;
                pieces &= pieces - 1;

                output[start_square / constants::BOARD_SIZE]
                    [start_square % constants::BOARD_SIZE] = constants::PIECE_FROM_ENUM[i];
            }
        }

        for row in output {
            for char in row {
                print!("{} ", char);
            }
            println!();
        }
    }
}
