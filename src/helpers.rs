const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub fn north(b: u64) -> u64 {
    b >> 8
}

pub fn south(b: u64) -> u64 {
    b << 8
}

pub fn east_post(b: u64) -> u64 {
    (b << 1) & NOT_H_FILE
}

pub fn north_east_post(b: u64) -> u64 {
    (b >> 7) & NOT_H_FILE
}

pub fn south_east_post(b: u64) -> u64 {
    (b << 9) & NOT_H_FILE
}

pub fn west_post(b: u64) -> u64 {
    (b >> 1) & NOT_A_FILE
}

pub fn north_west_post(b: u64) -> u64 {
    (b >> 9) & NOT_A_FILE
}

pub fn south_west_post(b: u64) -> u64 {
    (b << 7) & NOT_A_FILE
}

pub fn east_pre(b: u64) -> u64 {
    (b & NOT_H_FILE) << 1
}

pub fn north_east_pre(b: u64) -> u64 {
    (b & NOT_H_FILE) >> 7
}

pub fn south_east_pre(b: u64) -> u64 {
    (b & NOT_H_FILE) << 9
}

pub fn west_pre(b: u64) -> u64 {
    (b & NOT_A_FILE) >> 1
}

pub fn north_west_pre(b: u64) -> u64 {
    (b & NOT_A_FILE) >> 9
}

pub fn south_west_pre(b: u64) -> u64 {
    (b & NOT_A_FILE) << 7
}

// returns bboard of all white pawns going one square forwards
pub fn w_single_pawn_push_targets(w_pawns: u64, empty_squares: u64) -> u64 {
    north(w_pawns) & empty_squares
}

// returns bboard of all black pawns going one square forwards
pub fn b_single_pawn_push_targets(b_pawns: u64, empty_squares: u64) -> u64 {
    south(b_pawns) & empty_squares
}

pub fn w_pawn_east_attacks(w_pawns: u64) -> u64 {
    north_east_pre(w_pawns)
}

pub fn w_pawn_west_attacks(w_pawns: u64) -> u64 {
    north_west_pre(w_pawns)
}

pub fn b_pawn_east_attacks(b_pawns: u64) -> u64 {
    south_east_pre(b_pawns)
}

pub fn b_pawn_west_attacks(b_pawns: u64) -> u64 {
    south_west_pre(b_pawns)
}

pub fn w_pawn_any_attacks(w_pawns: u64) -> u64 {
    w_pawn_east_attacks(w_pawns) | w_pawn_west_attacks(w_pawns)
}

pub fn w_pawn_double_attacks(w_pawns: u64) -> u64 {
    w_pawn_east_attacks(w_pawns) & w_pawn_west_attacks(w_pawns)
}

pub fn w_pawn_single_attacks(w_pawns: u64) -> u64 {
    w_pawn_east_attacks(w_pawns) ^ w_pawn_west_attacks(w_pawns)
}

pub fn b_pawn_any_attacks(b_pawns: u64) -> u64 {
    b_pawn_east_attacks(b_pawns) | b_pawn_west_attacks(b_pawns)
}

pub fn b_pawn_double_attacks(b_pawns: u64) -> u64 {
    b_pawn_east_attacks(b_pawns) & b_pawn_west_attacks(b_pawns)
}

pub fn b_pawn_single_attacks(b_pawns: u64) -> u64 {
    b_pawn_east_attacks(b_pawns) ^ b_pawn_west_attacks(b_pawns)
}
