const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub fn north_one(b: u64) -> u64 {
    b >> 8
}

pub fn south_one(b: u64) -> u64 {
    b << 8
}

pub fn east_one_post(b: u64) -> u64 {
    (b << 1) & NOT_H_FILE
}

pub fn north_east_one_post(b: u64) -> u64 {
    (b >> 7) & NOT_H_FILE
}

pub fn south_east_one_post(b: u64) -> u64 {
    (b << 9) & NOT_H_FILE
}

pub fn west_one_post(b: u64) -> u64 {
    (b >> 1) & NOT_A_FILE
}

pub fn north_west_one_post(b: u64) -> u64 {
    (b >> 9) & NOT_A_FILE
}

pub fn south_west_one_post(b: u64) -> u64 {
    (b << 7) & NOT_A_FILE
}

pub fn east_one_pre(b: u64) -> u64 {
    (b & NOT_H_FILE) << 1
}

pub fn north_east_one_pre(b: u64) -> u64 {
    (b & NOT_H_FILE) >> 7
}

pub fn south_east_one_pre(b: u64) -> u64 {
    (b & NOT_H_FILE) << 9
}

pub fn west_one_pre(b: u64) -> u64 {
    (b & NOT_A_FILE) >> 1
}

pub fn north_west_one_pre(b: u64) -> u64 {
    (b & NOT_A_FILE) >> 9
}

pub fn south_west_one_pre(b: u64) -> u64 {
    (b & NOT_A_FILE) << 7
}

// returns bboard of all white pawns going one square forwards
pub fn w_single_pawn_push_targets(w_pawns: u64, empty_squares: u64) -> u64 {
    north_one(w_pawns) & empty_squares
}

// returns bboard of all black pawns going one square forwards
pub fn b_single_pawn_push_targets(b_pawns: u64, empty_squares: u64) -> u64 {
    south_one(b_pawns) & empty_squares
}
