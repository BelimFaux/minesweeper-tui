extern crate termion;

use minesweeper::*;

fn main() {
    let mode = Mode::EASY;
    let mut view = FieldView::new(mode);
    let mut game = Field::new(mode);

    let _ = view.print_field(&game, "");

    while let Ok(action) = view.handle_inputs() {
        let info = match action {
            Action::Uncover(x, y) => match game.click(x as usize, y as usize) {
                Ok(false) => {
                    game.uncover();
                    let _ = view.game_over(game, GameOver::Lost);
                    break;
                }
                Err(s) => s,
                _ => "",
            },
            Action::SetFlag(x, y) => {
                if let Err(s) = game.toggle_flag(x as usize, y as usize) {
                    s
                } else {
                    ""
                }
            }
            Action::Quit => {
                game.uncover();
                let _ = view.game_over(game, GameOver::Quit);
                break;
            }
            Action::None => "",
        };
        if game.won() {
            game.uncover();
            let _ = view.game_over(game, GameOver::Won);
            break;
        }
        let _ = view.print_field(&game, info);
    }
}
