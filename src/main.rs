mod board;
mod constants;
mod engine;
mod game;
mod moves;
mod worker;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 't', long = "num_threads", default_value_t = 8)]
    num_threads: usize,
    #[arg(short = 'f', long = "fen", default_value_t = constants::DEFAULT_FEN.to_string())]
    fen: String,
    #[arg(short = 'd', long = "depth", default_value_t = 6)]
    depth: usize,
}

fn main() {
    let args = Cli::parse();
    let board: board::Board = board::Board::init(args.fen);
    board.print_board();
}
