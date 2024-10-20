use crate::constants::{BOARD_SIZE, PIECE_FROM_ENUM};

#[derive(Default)]
pub struct Move {
    start: u64,
    end: u64,
    piece_index: usize,
}

impl Move {
    pub fn new(start_in: u64, end_in: u64, piece_in: usize) -> Move {
        Move {
            start: start_in,
            end: end_in,
            piece_index: piece_in,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let u_board_size = BOARD_SIZE as u32;
        write!(
            f,
            "({}{}{}) -> ({}{}{})",
            PIECE_FROM_ENUM[self.piece_index],
            ((self.start.trailing_zeros() % u_board_size) as u8 + b'a') as char,
            8 - self.start.trailing_zeros() / u_board_size,
            PIECE_FROM_ENUM[self.piece_index],
            ((self.end.trailing_zeros() % u_board_size) as u8 + b'a') as char,
            8 - self.end.trailing_zeros() / u_board_size,
        )
    }
}
