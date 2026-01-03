use std::{
    cmp::{max, min},
    fmt::Write as FmtWrite,
    io::{self, stdin, stdout, Stdout, Write},
};

use termion::{
    event::{Event, Key, MouseButton, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
};

use crate::game::{controller::Action, field::Field, Mode};

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum GameOver {
    Quit,
    Lost,
    Won,
}

const INFOBUF_SIZE: usize = 30;

pub struct FieldView {
    cursor_pos: Position,
    max_cursor: Position,
    min_cursor: Position,
    infobuf: String,
    stdout: MouseTerminal<RawTerminal<Stdout>>,
}

impl FieldView {
    const COORD_WIDTH: u16 = 3;
    const HEADER_LINES: u16 = 1;

    pub fn new(mode: Mode) -> FieldView {
        let coords_width = mode.x_size().checked_ilog10().unwrap_or(0) as u16 + 1;
        let coords_height = mode.y_size().checked_ilog10().unwrap_or(0) as u16 + 1;

        let cursor_pos = Position {
            x: coords_width + Self::HEADER_LINES + 2,
            y: coords_height + 2,
        };

        let min_cursor = cursor_pos;

        // maximum allowed x/y value in screen space
        let max_cursor = Position {
            x: ((mode.x_size() as u16) - 1) * Self::COORD_WIDTH + cursor_pos.x,
            y: ((mode.y_size() as u16) - 1) + cursor_pos.y,
        };

        let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

        FieldView {
            cursor_pos,
            max_cursor,
            min_cursor,
            infobuf: String::with_capacity(INFOBUF_SIZE),
            stdout,
        }
    }

    pub fn print_field(&mut self, field: &Field, info: &str) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(0, Self::HEADER_LINES + 1),
            field,
            termion::cursor::Goto(self.cursor_pos.y, self.cursor_pos.x)
        )?;
        let (x, y) = self.get_field_coords();

        let info = if info.is_empty() {
            if self.infobuf.is_empty() {
                let _ = write!(self.infobuf, "({x},{y})");
            }
            &self.infobuf
        } else {
            info
        };
        write!(
            self.stdout,
            "{}Info: {}\n\r",
            termion::cursor::Goto(1, 1),
            info
        )?;
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(self.cursor_pos.x, self.cursor_pos.y)
        )?;
        self.stdout.flush()?;

        Ok(())
    }

    fn get_field_coords(&self) -> (u16, u16) {
        let x = (self.cursor_pos.x - self.min_cursor.x) / Self::COORD_WIDTH;
        let y = self.cursor_pos.y - self.min_cursor.y;

        (x, y)
    }

    pub fn handle_inputs(&mut self) -> io::Result<Action> {
        for c in stdin().events() {
            let evt = c?;
            self.infobuf.clear();
            match evt {
                Event::Key(Key::Char('q')) => return Ok(Action::Quit),
                Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
                    self.cursor_pos.y += 1;
                    self.cursor_pos.y = min(self.cursor_pos.y, self.max_cursor.y);
                    return Ok(Action::None);
                }
                Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
                    self.cursor_pos.y -= 1;
                    self.cursor_pos.y = max(self.cursor_pos.y, self.min_cursor.y);
                    return Ok(Action::None);
                }
                Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
                    self.cursor_pos.x -= Self::COORD_WIDTH;
                    self.cursor_pos.x = max(self.cursor_pos.x, self.min_cursor.x);
                    return Ok(Action::None);
                }
                Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
                    self.cursor_pos.x += Self::COORD_WIDTH;
                    self.cursor_pos.x = min(self.cursor_pos.x, self.max_cursor.x);
                    return Ok(Action::None);
                }
                Event::Mouse(MouseEvent::Press(button, nx, ny)) => {
                    self.cursor_pos.y = ny.clamp(self.min_cursor.y, self.max_cursor.y);

                    self.cursor_pos.x = nx.clamp(self.min_cursor.x, self.max_cursor.x);
                    self.cursor_pos.x -=
                        (self.cursor_pos.x - self.min_cursor.x) % Self::COORD_WIDTH;

                    let (x, y) = self.get_field_coords();

                    if let MouseButton::Left = button {
                        let _ = write!(self.infobuf, "Uncovered ({x}, {y}). No Bomb!");
                        return Ok(Action::Uncover(x, y));
                    } else {
                        let _ = write!(self.infobuf, "Flag set at ({x}, {y}).");
                        return Ok(Action::SetFlag(x, y));
                    }
                }
                Event::Key(Key::Char('\n')) | Event::Key(Key::Char('u')) => {
                    let (x, y) = self.get_field_coords();
                    let _ = write!(self.infobuf, "Uncovered ({x}, {y}). No Bomb!");
                    return Ok(Action::Uncover(x, y));
                }
                Event::Key(Key::Backspace) | Event::Key(Key::Char('f')) => {
                    let (x, y) = self.get_field_coords();
                    let _ = write!(self.infobuf, "Flag set at ({x}, {y}).");
                    return Ok(Action::SetFlag(x, y));
                }
                _ => {}
            }
        }

        Ok(Action::None)
    }

    pub fn game_over(mut self, field: Field, cause: GameOver) -> io::Result<()> {
        let msg = match cause {
            GameOver::Quit => "You quit.",
            GameOver::Won => "You won! Congratulations.",
            GameOver::Lost => "You lost. Try again.",
        };
        write!(
            self.stdout,
            "{}{} {}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            msg,
            termion::cursor::Goto(0, Self::HEADER_LINES + 1),
            field,
        )?;
        self.stdout.flush()?;

        Ok(())
    }
}
