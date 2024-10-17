use crate::constants::{self, BOARD_SIZE};

#[derive(Default)]
pub struct Move {
    start: u64,
    end: u64,
}

impl Move {
    pub fn init(start_in: u64, end_in: u64) -> Move {
        Move {
            start: start_in,
            end: end_in,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let u_board_size = BOARD_SIZE as u32;
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.start.trailing_zeros() / u_board_size,
            self.start.trailing_zeros() % u_board_size,
            self.end.trailing_zeros() / u_board_size,
            self.end.trailing_zeros() % u_board_size
        )
    }
}
