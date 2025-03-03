extern crate termion;

use std::io::{stdin, stdout, Write};
use std::process::exit;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use minesweeper::*;

fn game_over<W>(mut stdout: MouseTerminal<W>, mut field: Field, message: &str) -> !
where
    W: Write,
{
    field.uncover();
    write!(
        stdout,
        "{}{}{message}\n\r{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        field
    )
    .unwrap();
    stdout.flush().unwrap();
    drop(stdout); // explicitly drop the Terminal to exit raw mode
    exit(0)
}

const START_X: u16 = 1;
const COORD_WIDTH: u16 = 3;
const HEADER_LINES: u16 = 1;

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mode = Mode::EASY;

    // how many lines is the header long in total. lines + space needed for coord numbers.
    let xstart_cursor = mode.y_size().checked_ilog10().unwrap_or(0) as u16 + 2 + HEADER_LINES;
    // how much space is needed to the left. thickness of coord numbers + start_x
    let ystart_cursor = mode.x_size().checked_ilog10().unwrap_or(0) as u16 + 3 + START_X;
    // where the field starts after the header lines
    let start_y = HEADER_LINES + 1;
    // maximum allowed x/y value in screen space
    let x_clamp = ((mode.x_size() as u16) - 1) * COORD_WIDTH + ystart_cursor;
    let y_clamp = ((mode.y_size() as u16) - 1) + xstart_cursor;
    let mut field = Field::new(mode);

    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(START_X, start_y),
        field,
        termion::cursor::Goto(ystart_cursor, xstart_cursor)
    )
    .unwrap();
    stdout.flush().unwrap();

    let (mut x, mut y) = (ystart_cursor, xstart_cursor);

    for c in stdin.events() {
        let mut info = String::new();
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                game_over(stdout, field, "Game was terminated by User.");
            }
            Event::Key(Key::Char('j')) | Event::Key(Key::Down) => y += 1,
            Event::Key(Key::Char('k')) | Event::Key(Key::Up) => y -= 1,
            Event::Key(Key::Char('h')) | Event::Key(Key::Left) => x -= 3,
            Event::Key(Key::Char('l')) | Event::Key(Key::Right) => x += 3,
            Event::Mouse(MouseEvent::Press(button, mx, my)) => {
                x = mx.clamp(ystart_cursor, x_clamp);
                x -= (x - ystart_cursor) % COORD_WIDTH;
                y = my.clamp(xstart_cursor, y_clamp);
                info = format!("Uncovered ({x}, {y}). No Bomb!");
                let x = (x - ystart_cursor) / COORD_WIDTH;
                let y = y - xstart_cursor;

                if let MouseButton::Left = button {
                    let ret = field.click(x.into(), y.into());
                    match ret {
                        Err(str) => info = str,
                        Ok(false) => game_over(stdout, field, "Game Over!"),
                        _ => {}
                    }
                } else {
                    info = format!("Flag set at ({x}, {y}).");
                    let res = field.toggle_flag(x.into(), y.into());
                    if let Err(str) = res {
                        info = str;
                    }
                }
            }
            Event::Key(Key::Char('\n')) | Event::Key(Key::Char('u')) => {
                let x = (x - ystart_cursor) / COORD_WIDTH;
                let y = y - xstart_cursor;
                info = format!("Uncovered ({x}, {y}). No Bomb!");
                let ret = field.click(x.into(), y.into());
                match ret {
                    Err(str) => info = str,
                    Ok(false) => game_over(stdout, field, "Game Over!"),
                    _ => {}
                }
            }
            Event::Key(Key::Backspace) | Event::Key(Key::Char('f')) => {
                let x = (x - ystart_cursor) / COORD_WIDTH;
                let y = y - xstart_cursor;
                info = format!("Flag set at ({x}, {y}).");
                let res = field.toggle_flag(x.into(), y.into());
                if let Err(str) = res {
                    info = str;
                }
            }
            _ => {}
        }

        if field.won() {
            game_over(stdout, field, "You did it! Congratulations.");
        }

        x = x.clamp(ystart_cursor, x_clamp);
        y = y.clamp(xstart_cursor, y_clamp);

        if info.is_empty() {
            let x = (x - ystart_cursor) / COORD_WIDTH;
            let y = y - xstart_cursor;
            info = format!("Current Position: ({x},{y})")
        }

        write!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(START_X, start_y),
            field,
            termion::cursor::Goto(x, y),
        )
        .unwrap();
        write!(stdout, "{}Info: {info}\n\r", termion::cursor::Goto(1, 1)).unwrap();

        write!(stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
        stdout.flush().unwrap();
    }
}
