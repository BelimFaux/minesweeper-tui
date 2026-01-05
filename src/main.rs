extern crate termion;

use clap::Parser;
use minesweeper::{Args, GameController};

fn main() {
    let args = Args::parse();
    let controller = GameController::new(args.difficulty);

    controller.run();
}
