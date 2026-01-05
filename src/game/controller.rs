use crate::{
    Mode,
    field::Field,
    view::{FieldView, GameOver},
};

/// Represents the available actions that can be taken
#[derive(Debug, Clone, Copy)]
pub enum Action {
    None,
    Quit,
    Uncover(u16, u16),
    SetFlag(u16, u16),
}

pub struct GameController<'a> {
    info: &'a str,
    field: Field,
    view: FieldView,
}

impl<'a> GameController<'a> {
    /// Create a new game controller for the given mode
    #[must_use]
    pub fn new(mode: Mode) -> GameController<'a> {
        GameController {
            info: "",
            field: Field::new(mode),
            view: FieldView::new(mode),
        }
    }

    /// helper to perform an [Action]
    /// returns `Some` containing the reason when this action leads to a game over else `None`
    fn perform_action(&mut self, action: Action) -> Option<GameOver> {
        self.info = match action {
            Action::Uncover(x, y) => match self.field.click(x as usize, y as usize) {
                Ok(false) => {
                    self.field.uncover();
                    return Some(GameOver::Lost);
                }
                Err(s) => s,
                _ => "",
            },
            Action::SetFlag(x, y) => {
                if let Err(s) = self.field.toggle_flag(x as usize, y as usize) {
                    s
                } else {
                    ""
                }
            }
            Action::Quit => {
                self.field.uncover();
                return Some(GameOver::Quit);
            }
            Action::None => "",
        };

        None
    }

    /// Run the main loop for the game
    pub fn run(mut self) {
        let _ = self.view.print_field(&self.field, self.info);

        while let Ok(action) = self.view.handle_inputs() {
            if let Some(game_over) = self.perform_action(action) {
                let _ = self.view.game_over(&self.field, game_over);
                break;
            }
            if self.field.won() {
                self.field.uncover();
                let _ = self.view.game_over(&self.field, GameOver::Won);
                break;
            }
            let _ = self.view.print_field(&self.field, self.info);
        }
    }
}
