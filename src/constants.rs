use phf::phf_map;

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub const BOARD_SIZE: usize = 8;

pub const NUM_TYPES_PIECES: usize = 12;

pub enum PieceTypes {
    BlackPawn = 0,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
}

pub const PIECE_FROM_ENUM: [char; 12] =
    ['p', 'n', 'b', 'r', 'q', 'k', 'P', 'N', 'B', 'R', 'Q', 'K'];
pub const ENUM_FROM_PIECE: phf::Map<char, usize> = phf_map! {
    'p' => 0,
    'n' => 1,
    'b' => 2,
    'r' => 3,
    'q' => 4,
    'k' => 5,
    'P' => 6,
    'N' => 7,
    'B' => 8,
    'R' => 9,
    'Q' => 10,
    'K' => 11
};
