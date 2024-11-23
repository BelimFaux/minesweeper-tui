use std::io::{self, Read};

use minesweeper::*;

fn get_int(msg: &str) -> usize {
    println!("{msg}: ");
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let trimmed = buffer.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => return i,
            Err(..) => {
                println!("Input is not an Integer. Try again.");
                continue
            },
        }
    }
}

fn main() {
    let mut field = Field::new(Mode::EASY);

    loop {
        println!("{}", field);

        let x = get_int("Input X Coordinate");
        let y = get_int("Input Y Coordinate");

        let bomb = match field.click(x, y) {
            Ok(b) => b,
            Err(s) => { println!("{}", s); continue; },
        };

        if !bomb {
            println!("Youve hit a bomb. GAME OVER.");
            break;
        }

        if field.won() {
            println!("You won! Congratulations.");
            break;
        }
    }
}
