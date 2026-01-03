extern crate termion;

use minesweeper::*;

fn main() {
    let mode = Mode::EASY;
    let controller = GameController::new(mode);

    controller.run();
}
