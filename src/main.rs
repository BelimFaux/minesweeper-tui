extern crate termion;

use clap::Parser;
use minesweeper::*;

fn main() {
    let args = Args::parse();
    let controller = GameController::new(args.difficulty);

    controller.run();
}
